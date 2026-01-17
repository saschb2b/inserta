use bevy::prelude::*;

// ============================================================================
// Game State
// ============================================================================

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Shop,
    Playing,
}

/// Marker component for entities that should be despawned when leaving a state
#[derive(Component)]
pub struct CleanupOnStateExit(pub GameState);

// ============================================================================
// Arena Configuration
// ============================================================================

/// Configuration for spawning a fighter
#[derive(Clone, Debug)]
pub struct FighterConfig {
    pub start_x: i32,
    pub start_y: i32,
    pub max_hp: i32,
    pub actions: Vec<ActionType>,
}

impl Default for FighterConfig {
    fn default() -> Self {
        Self {
            start_x: 1,
            start_y: 1,
            max_hp: 100,
            // NOTE: ChargedShot is now part of the weapon system (Blaster)
            // Actions are special abilities separate from the equipped weapon
            actions: vec![ActionType::Heal, ActionType::Shield, ActionType::WideSword],
        }
    }
}

/// Configuration for spawning an enemy
#[derive(Clone, Debug)]
pub struct EnemyConfig {
    pub enemy_id: EnemyId,
    pub start_x: i32,
    pub start_y: i32,
    /// Override HP (if None, uses blueprint's scaled HP)
    pub hp_override: Option<i32>,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            enemy_id: EnemyId::Slime,
            start_x: 4,
            start_y: 1,
            hp_override: None,
        }
    }
}

impl EnemyConfig {
    /// Create a config for a specific enemy type at a position
    pub fn new(enemy_id: EnemyId, x: i32, y: i32) -> Self {
        Self {
            enemy_id,
            start_x: x,
            start_y: y,
            hp_override: None,
        }
    }

    /// Create a config with specific HP
    pub fn with_hp(mut self, hp: i32) -> Self {
        self.hp_override = Some(hp);
        self
    }
}

/// Types of enemies - re-export from enemies module for convenience
pub use crate::enemies::EnemyId;

/// Configuration for a complete arena battle
#[derive(Resource, Clone, Debug)]
pub struct ArenaConfig {
    pub fighter: FighterConfig,
    pub enemies: Vec<EnemyConfig>,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            fighter: FighterConfig::default(),
            enemies: vec![EnemyConfig::default()],
        }
    }
}

// ============================================================================
// Core Components
// ============================================================================

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;

/// Marker for enemy bullets (travel left instead of right)
#[derive(Component)]
pub struct EnemyBullet;

/// Marker for projectiles that have hit (in impact/finish animation)
#[derive(Component)]
pub struct ProjectileHit;

#[derive(Component)]
pub struct MuzzleFlash;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct HealthText;

#[derive(Component, Clone, Copy)]
pub struct RenderConfig {
    pub offset: Vec2,
    pub base_z: f32,
}

#[derive(Component)]
pub struct MoveTimer(pub Timer);

#[derive(Component)]
pub struct Lifetime(pub Timer);

#[derive(Component)]
pub struct BaseColor(pub Color);

#[derive(Component)]
pub struct FlashTimer(pub Timer);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterAnimState {
    Idle,
    Walk,
    Shoot,
}

#[derive(Component)]
pub struct FighterAnim {
    pub state: FighterAnimState,
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlimeAnimState {
    Idle,
    Shoot,
    Dead,
}

#[derive(Component)]
pub struct SlimeAnim {
    pub state: SlimeAnimState,
    pub frame: usize,
    pub timer: Timer,
}

/// Marker for the inner panel surface that can be highlighted
#[derive(Component)]
pub struct TilePanel {
    pub x: i32,
    pub y: i32,
}

/// Stores the base color of a tile panel for restoration after highlight
#[derive(Component)]
pub struct TileBaseColor(pub Color);

/// Tracks the highlight state of a tile for smooth transitions
#[derive(Component)]
pub struct TileHighlightState {
    /// Current highlight intensity (0.0 = normal, 1.0 = fully highlighted)
    pub intensity: f32,
    /// Target intensity (what we're transitioning to)
    pub target: f32,
    /// Whether this is a player-side tile (red) or enemy-side (blue)
    pub is_player_side: bool,
}

impl TileHighlightState {
    pub fn new(is_player_side: bool) -> Self {
        Self {
            intensity: 0.0,
            target: 0.0,
            is_player_side,
        }
    }
}

/// Resource holding tile texture assets for normal and highlighted states
#[derive(Resource)]
pub struct TileAssets {
    pub red_normal: Handle<Image>,
    pub red_highlighted: Handle<Image>,
    pub blue_normal: Handle<Image>,
    pub blue_highlighted: Handle<Image>,
}

/// Player health display text marker
#[derive(Component)]
pub struct PlayerHealthText;

/// Enemy AI timers
#[derive(Component)]
pub struct EnemyAI {
    pub move_timer: Timer,
    pub shoot_timer: Timer,
    pub charge_timer: Option<Timer>,
}

#[derive(Resource)]
pub struct InputCooldown(pub Timer);

// ============================================================================
// Action System
// ============================================================================

/// Types of actions a fighter can perform
/// NOTE: ChargedShot was removed - it's now handled by the weapon system (Blaster's charged shot)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Heal,
    Shield,
    WideSword,
    // Future actions:
    // AreaBomb,
    // Dash,
    // etc.
}

/// State of an action slot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionState {
    Ready,
    Charging,
    OnCooldown,
}

/// A single action slot (1-4)
#[derive(Component)]
pub struct ActionSlot {
    pub slot_index: usize,
    pub action_type: ActionType,
    pub state: ActionState,
    pub cooldown_timer: Timer,
    pub charge_timer: Option<Timer>,
    pub cooldown_duration: f32,
    pub charge_duration: f32,
}

impl ActionSlot {
    pub fn new(slot_index: usize, action_type: ActionType, cooldown: f32, charge: f32) -> Self {
        Self {
            slot_index,
            action_type,
            state: ActionState::Ready,
            cooldown_timer: Timer::from_seconds(cooldown, TimerMode::Once),
            charge_timer: None,
            cooldown_duration: cooldown,
            charge_duration: charge,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == ActionState::Ready
    }

    pub fn start_charging(&mut self) {
        if self.charge_duration > 0.0 {
            self.state = ActionState::Charging;
            self.charge_timer = Some(Timer::from_seconds(self.charge_duration, TimerMode::Once));
        }
    }

    pub fn start_cooldown(&mut self) {
        self.state = ActionState::OnCooldown;
        self.cooldown_timer = Timer::from_seconds(self.cooldown_duration, TimerMode::Once);
        self.charge_timer = None;
    }

    pub fn cooldown_progress(&self) -> f32 {
        if self.state == ActionState::OnCooldown {
            self.cooldown_timer.fraction()
        } else {
            1.0
        }
    }

    pub fn charge_progress(&self) -> f32 {
        if let Some(ref timer) = self.charge_timer {
            timer.fraction()
        } else {
            0.0
        }
    }
}

/// Marker for the charged shot projectile
#[derive(Component)]
pub struct ChargedShot {
    pub damage: i32,
}

/// Marker for heal effect visual
#[derive(Component)]
pub struct HealEffect;

/// Shield that blocks incoming damage
#[derive(Component)]
pub struct Shield {
    pub duration_timer: Timer,
}

/// Marker for WideSword slash effect
#[derive(Component)]
pub struct WideSwordSlash {
    pub damage: i32,
    pub hit_tiles: Vec<(i32, i32)>, // (x, y) tiles that will be hit
}

// ============================================================================
// Tile Targeting System
// ============================================================================

/// Component for entities that target/highlight specific tiles.
/// Used by the tile highlight system to show which tiles are being attacked.
///
/// For single-tile attacks (bullets), use `TargetsTiles::single()`.
/// For multi-tile attacks (WideSword), use `TargetsTiles::multiple()`.
#[derive(Component)]
pub struct TargetsTiles {
    /// The tiles being targeted. If empty, uses the entity's GridPosition.
    pub tiles: Vec<(i32, i32)>,
    /// If true, uses GridPosition instead of explicit tiles list
    pub use_grid_position: bool,
}

impl TargetsTiles {
    /// Target a single tile (uses the entity's GridPosition)
    pub fn single() -> Self {
        Self {
            tiles: Vec::new(),
            use_grid_position: true,
        }
    }

    /// Target multiple specific tiles
    pub fn multiple(tiles: Vec<(i32, i32)>) -> Self {
        Self {
            tiles,
            use_grid_position: false,
        }
    }
}

// ============================================================================
// Action Bar UI Components
// ============================================================================

/// Marker for the action bar container
#[derive(Component)]
pub struct ActionBar;

/// Marker for an action slot UI element
#[derive(Component)]
pub struct ActionSlotUI {
    pub slot_index: usize,
}

/// Marker for the cooldown overlay on an action slot
#[derive(Component)]
pub struct ActionCooldownOverlay {
    pub slot_index: usize,
}

/// Marker for the charge progress bar
#[derive(Component)]
pub struct ActionChargeBar {
    pub slot_index: usize,
}

/// Marker for the key binding text on action slot
#[derive(Component)]
pub struct ActionKeyText {
    pub slot_index: usize,
}
