use bevy::prelude::*;

use crate::constants::{
    ARENA_Y_OFFSET, GRID_HEIGHT, GRID_WIDTH, ROW_SKEW_X, TILE_ASSET_HEIGHT, TILE_ASSET_WIDTH,
    TILE_LIP_HEIGHT,
};

// ============================================================================
// Arena Layout Resource (Responsive Scaling)
// ============================================================================

/// Computed arena layout based on screen dimensions.
/// Tiles scale to fill screen width while maintaining aspect ratio.
#[derive(Resource, Debug, Clone)]
pub struct ArenaLayout {
    /// Current screen width
    pub screen_width: f32,
    /// Current screen height
    pub screen_height: f32,
    /// Computed tile width (fills screen width with GRID_WIDTH tiles)
    pub tile_width: f32,
    /// Computed tile height (maintains aspect ratio with original asset)
    pub tile_height: f32,
    /// Computed lip height (scaled proportionally)
    pub lip_height: f32,
    /// Computed visible tile height (tile_height - lip_height)
    pub visible_height: f32,
    /// Step between tile centers in X
    pub step_x: f32,
    /// Step between tile centers in Y (visible_height for overlap)
    pub step_y: f32,
    /// Scale factor relative to original asset size
    pub scale: f32,
    /// Arena Y offset (for action bar)
    pub arena_y_offset: f32,
}

impl Default for ArenaLayout {
    fn default() -> Self {
        Self::from_screen_size(1280.0, 800.0)
    }
}

impl ArenaLayout {
    /// Compute arena layout from screen dimensions.
    /// Tiles fill the full screen width.
    pub fn from_screen_size(screen_width: f32, screen_height: f32) -> Self {
        // Tile width = screen width / number of columns
        let tile_width = screen_width / GRID_WIDTH as f32;

        // Scale factor based on width
        let scale = tile_width / TILE_ASSET_WIDTH;

        // Scale all tile dimensions proportionally
        let tile_height = TILE_ASSET_HEIGHT * scale;
        let lip_height = TILE_LIP_HEIGHT * scale;
        let visible_height = tile_height - lip_height;

        Self {
            screen_width,
            screen_height,
            tile_width,
            tile_height,
            lip_height,
            visible_height,
            step_x: tile_width,
            step_y: visible_height,
            scale,
            arena_y_offset: ARENA_Y_OFFSET,
        }
    }

    /// Get world position for tile sprite center at grid (x, y)
    pub fn tile_sprite_world(&self, x: i32, y: i32) -> Vec2 {
        let center_x = (GRID_WIDTH as f32 - 1.0) / 2.0;
        let center_y = (GRID_HEIGHT as f32 - 1.0) / 2.0;

        let relative_x = (x as f32) - center_x;
        let relative_y = (y as f32) - center_y;

        let pos_x = relative_x * self.step_x + relative_y * ROW_SKEW_X * self.scale;
        let pos_y = relative_y * self.step_y + self.arena_y_offset;

        Vec2::new(pos_x, pos_y)
    }

    /// Get world position for character feet (floor point) at grid (x, y)
    pub fn tile_floor_world(&self, x: i32, y: i32) -> Vec2 {
        let sprite_pos = self.tile_sprite_world(x, y);

        // Floor is at center of visible area (above lip)
        let floor_offset_y = self.lip_height + self.visible_height / 2.0 - self.tile_height / 2.0;

        Vec2::new(sprite_pos.x, sprite_pos.y + floor_offset_y)
    }

    /// Get tile size as Vec2 for sprite custom_size
    pub fn tile_size(&self) -> Vec2 {
        Vec2::new(self.tile_width, self.tile_height)
    }

    /// Scale a Vec2 by the arena scale factor
    pub fn scale_vec2(&self, v: Vec2) -> Vec2 {
        v * self.scale
    }

    /// Scale a single value by the arena scale factor
    pub fn scale_val(&self, v: f32) -> f32 {
        v * self.scale
    }
}

// ============================================================================
// Global Progression Resources
// ============================================================================

/// Tracks the player's currency
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct PlayerCurrency {
    pub zenny: u64,
}

/// Tracks the current progression level (wave/stage)
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct GameProgress {
    pub current_level: u32,
    pub enemies_defeated: u32,
}

impl GameProgress {
    pub fn next_level(&mut self) {
        self.current_level += 1;
    }
}

/// Persistent stats that can be upgraded
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PlayerUpgrades {
    /// Weapon base damage upgrade count
    pub damage_level: u32,
    /// Max HP upgrade count
    pub health_level: u32,
    /// Fire rate (cooldown reduction) upgrade count
    pub fire_rate_level: u32,
    /// Critical chance upgrade count
    pub crit_chance_level: u32,
}

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WaveState {
    #[default]
    Spawning,
    Active,
    Cleared,
}

impl PlayerUpgrades {
    // Calculation helpers for actual values

    pub fn get_bonus_damage(&self) -> i32 {
        self.damage_level as i32 // +1 damage per level
    }

    pub fn get_max_hp(&self) -> i32 {
        100 + (self.health_level as i32 * 20) // +20 HP per level
    }

    pub fn get_cooldown_modifier(&self) -> f32 {
        // 5% faster fire rate per level, capped at some reasonable amount (e.g. 50%)
        let reduction = (self.fire_rate_level as f32 * 0.05).min(0.5);
        1.0 - reduction
    }

    pub fn get_crit_chance_bonus(&self) -> f32 {
        self.crit_chance_level as f32 * 0.02 // +2% crit chance per level
    }

    // Cost calculations

    pub fn cost_damage(&self) -> u64 {
        100 * (1.5_f32.powi(self.damage_level as i32) as u64)
    }

    pub fn cost_health(&self) -> u64 {
        50 * (1.2_f32.powi(self.health_level as i32) as u64)
    }

    pub fn cost_fire_rate(&self) -> u64 {
        150 * (1.6_f32.powi(self.fire_rate_level as i32) as u64)
    }

    pub fn cost_crit_chance(&self) -> u64 {
        200 * (1.8_f32.powi(self.crit_chance_level as i32) as u64)
    }
}
