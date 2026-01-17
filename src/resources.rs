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

// ============================================================================
// Campaign Resources
// ============================================================================

use crate::components::EnemyConfig;
use crate::enemies::EnemyId;

/// Tracks campaign progress (unlocked arcs, completed battles)
#[derive(Resource, Debug, Clone)]
pub struct CampaignProgress {
    /// Highest unlocked arc index (0-based)
    pub unlocked_arc: usize,
    /// For each arc, which battles have been completed (true = won)
    pub completed_battles: Vec<Vec<bool>>,
}

impl Default for CampaignProgress {
    fn default() -> Self {
        Self {
            unlocked_arc: 0,
            completed_battles: vec![vec![false; 10]], // Arc 1 has 10 battles
        }
    }
}

impl CampaignProgress {
    /// Check if a specific battle in an arc has been won
    pub fn is_battle_won(&self, arc: usize, battle: usize) -> bool {
        self.completed_battles
            .get(arc)
            .and_then(|battles| battles.get(battle))
            .copied()
            .unwrap_or(false)
    }

    /// Mark a battle as completed
    pub fn complete_battle(&mut self, arc: usize, battle: usize) {
        // Ensure we have enough arcs
        while self.completed_battles.len() <= arc {
            self.completed_battles.push(vec![false; 10]);
        }
        // Ensure we have enough battles
        while self.completed_battles[arc].len() <= battle {
            self.completed_battles[arc].push(false);
        }
        self.completed_battles[arc][battle] = true;

        // Check if boss battle (index 9) was completed to unlock next arc
        if battle == 9 && arc == self.unlocked_arc {
            self.unlocked_arc += 1;
        }
    }

    /// Check if an arc is unlocked
    pub fn is_arc_unlocked(&self, arc: usize) -> bool {
        arc <= self.unlocked_arc
    }
}

/// Currently selected battle to play
#[derive(Resource, Debug, Clone, Default)]
pub struct SelectedBattle {
    pub arc: usize,
    pub battle: usize,
}

/// Definition of a single battle encounter
#[derive(Debug, Clone)]
pub struct BattleDef {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: Vec<EnemyConfig>,
    pub is_boss: bool,
}

/// Definition of a campaign arc (10 battles)
#[derive(Debug, Clone)]
pub struct ArcDef {
    pub name: &'static str,
    pub description: &'static str,
    pub battles: Vec<BattleDef>,
}

/// Get all arc definitions
pub fn get_all_arcs() -> Vec<ArcDef> {
    vec![arc_1_slime_invasion()]
}

/// Arc 1: Slime Invasion
fn arc_1_slime_invasion() -> ArcDef {
    ArcDef {
        name: "Slime Invasion",
        description: "The slimes are attacking! Defeat them all.",
        battles: vec![
            // Battle 1: 1x Slime
            BattleDef {
                name: "First Contact",
                description: "1x Slime",
                enemies: vec![EnemyConfig::new(EnemyId::Slime, 4, 1)],
                is_boss: false,
            },
            // Battle 2: 2x Slime
            BattleDef {
                name: "Double Trouble",
                description: "2x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime, 4, 0),
                    EnemyConfig::new(EnemyId::Slime, 4, 2),
                ],
                is_boss: false,
            },
            // Battle 3: 3x Slime
            BattleDef {
                name: "Slime Trio",
                description: "3x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime, 4, 0),
                    EnemyConfig::new(EnemyId::Slime, 4, 1),
                    EnemyConfig::new(EnemyId::Slime, 4, 2),
                ],
                is_boss: false,
            },
            // Battle 4: 1x Slime2
            BattleDef {
                name: "Slime II Appears",
                description: "1x Slime II",
                enemies: vec![EnemyConfig::new(EnemyId::Slime2, 4, 1)],
                is_boss: false,
            },
            // Battle 5: 1x Slime2, 1x Slime
            BattleDef {
                name: "Mixed Company",
                description: "1x Slime II, 1x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime2, 5, 1),
                    EnemyConfig::new(EnemyId::Slime, 4, 0),
                ],
                is_boss: false,
            },
            // Battle 6: 1x Slime2, 2x Slime
            BattleDef {
                name: "Slime Squad",
                description: "1x Slime II, 2x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime2, 5, 1),
                    EnemyConfig::new(EnemyId::Slime, 4, 0),
                    EnemyConfig::new(EnemyId::Slime, 4, 2),
                ],
                is_boss: false,
            },
            // Battle 7: 1x Slime2, 3x Slime
            BattleDef {
                name: "Slime Swarm",
                description: "1x Slime II, 3x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime2, 5, 1),
                    EnemyConfig::new(EnemyId::Slime, 4, 0),
                    EnemyConfig::new(EnemyId::Slime, 4, 2),
                    EnemyConfig::new(EnemyId::Slime, 3, 1),
                ],
                is_boss: false,
            },
            // Battle 8: 2x Slime2
            BattleDef {
                name: "Slime II Duo",
                description: "2x Slime II",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime2, 4, 0),
                    EnemyConfig::new(EnemyId::Slime2, 4, 2),
                ],
                is_boss: false,
            },
            // Battle 9: 2x Slime2, 1x Slime
            BattleDef {
                name: "Elite Guard",
                description: "2x Slime II, 1x Slime",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime2, 5, 0),
                    EnemyConfig::new(EnemyId::Slime2, 5, 2),
                    EnemyConfig::new(EnemyId::Slime, 4, 1),
                ],
                is_boss: false,
            },
            // Battle 10: BOSS - 1x Slime3, 2x Slime2
            BattleDef {
                name: "King Slime",
                description: "BOSS: King Slime + 2x Slime II",
                enemies: vec![
                    EnemyConfig::new(EnemyId::Slime3, 5, 1),
                    EnemyConfig::new(EnemyId::Slime2, 4, 0),
                    EnemyConfig::new(EnemyId::Slime2, 4, 2),
                ],
                is_boss: true,
            },
        ],
    }
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
