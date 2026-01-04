use bevy::prelude::*;

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

/// Player health display text marker
#[derive(Component)]
pub struct PlayerHealthText;

/// Enemy AI timers
#[derive(Component)]
pub struct EnemyAI {
    pub move_timer: Timer,
    pub shoot_timer: Timer,
}

#[derive(Resource)]
pub struct InputCooldown(pub Timer);

#[derive(Resource)]
pub struct ShootCooldown(pub Timer);

// ============================================================================
// Action System
// ============================================================================

/// Types of actions a fighter can perform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    ChargedShot,
    Heal,
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
