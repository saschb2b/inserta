// ============================================================================
// Enemy Components - ECS components for the enemy system
// ============================================================================

use super::{AttackBehavior, EnemyTraits, MovementBehavior};
use bevy::prelude::*;

/// Unique identifier for enemy types (used for blueprints and save data)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnemyId {
    #[default]
    Slime,
    // Future enemies:
    // Mettaur,
    // Canodumb,
    // Swordy,
}

/// Core stats for an enemy - attached as a component
#[derive(Component, Debug, Clone)]
pub struct EnemyStats {
    /// Base maximum HP (before scaling)
    pub base_hp: i32,
    /// Damage dealt on contact (if applicable)
    pub contact_damage: i32,
    /// Movement speed multiplier (1.0 = normal)
    pub move_speed: f32,
    /// Attack speed multiplier (1.0 = normal)  
    pub attack_speed: f32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            base_hp: 100,
            contact_damage: 10,
            move_speed: 1.0,
            attack_speed: 1.0,
        }
    }
}

/// Movement behavior component - determines how the enemy moves
#[derive(Component, Debug, Clone)]
pub struct EnemyMovement {
    pub behavior: MovementBehavior,
    pub move_timer: Timer,
    /// Internal state for behaviors that need it
    pub state: MovementState,
}

impl EnemyMovement {
    pub fn new(behavior: MovementBehavior, speed_multiplier: f32) -> Self {
        let base_cooldown = behavior.base_cooldown();
        Self {
            behavior,
            move_timer: Timer::from_seconds(base_cooldown / speed_multiplier, TimerMode::Repeating),
            state: MovementState::default(),
        }
    }
}

/// Internal state for movement behaviors
#[derive(Debug, Clone, Default)]
pub struct MovementState {
    /// For patrol: current direction (true = forward, false = backward)
    pub patrol_forward: bool,
    /// For chase: last known player position
    pub last_player_pos: Option<(i32, i32)>,
    /// For hide-and-peek: currently hidden
    pub is_hidden: bool,
    /// Generic state timer for behaviors that need timed phases
    pub phase_timer: Option<Timer>,
}

/// Attack behavior component - determines how the enemy attacks
#[derive(Component, Debug, Clone)]
pub struct EnemyAttack {
    pub behavior: AttackBehavior,
    pub cooldown_timer: Timer,
    pub charge_timer: Option<Timer>,
    /// Internal state for attacks
    pub state: AttackState,
}

impl EnemyAttack {
    pub fn new(behavior: AttackBehavior, speed_multiplier: f32) -> Self {
        let base_cooldown = behavior.base_cooldown();
        Self {
            behavior,
            cooldown_timer: Timer::from_seconds(
                base_cooldown / speed_multiplier,
                TimerMode::Repeating,
            ),
            charge_timer: None,
            state: AttackState::Ready,
        }
    }
}

/// State machine for attack behaviors
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AttackState {
    #[default]
    Ready,
    Charging,
    Attacking,
    Recovering,
}

/// Container for enemy traits - optional modifiers
#[derive(Component, Debug, Clone, Default)]
pub struct EnemyTraitContainer {
    pub traits: EnemyTraits,
    /// Armor regeneration timer (if applicable)
    pub armor_regen_timer: Option<Timer>,
    /// HP regeneration timer (if applicable)
    pub hp_regen_timer: Option<Timer>,
}

impl EnemyTraitContainer {
    pub fn new(traits: EnemyTraits) -> Self {
        let hp_regen_timer = if traits.hp_regen_per_sec > 0.0 {
            Some(Timer::from_seconds(1.0, TimerMode::Repeating))
        } else {
            None
        };

        Self {
            traits,
            armor_regen_timer: None,
            hp_regen_timer,
        }
    }
}

/// Marker component indicating this enemy uses the new behavior system
#[derive(Component)]
pub struct BehaviorEnemy;

/// Component for charge telegraph visual effect
/// When present, the entity flashes to indicate an incoming attack
#[derive(Component)]
pub struct ChargingTelegraph {
    pub timer: Timer,
}

/// Component to track the enemy's current animation state generically
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnemyAnimState {
    #[default]
    Idle,
    Moving,
    Charging,
    Attacking,
    Hurt,
    Dead,
}
