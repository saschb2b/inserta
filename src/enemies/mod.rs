// ============================================================================
// Enemy System - Composable enemy behaviors like LEGO blocks
// ============================================================================
//
// This module provides a data-driven, composable system for defining enemies.
// Instead of hardcoding behavior in systems, enemies are defined by combining:
//
// 1. EnemyStats - Base stats (HP, damage, speed modifiers)
// 2. MovementBehavior - How the enemy moves (random, chase, patrol, stationary)
// 3. AttackBehavior - How the enemy attacks (projectile, melee, area, etc.)
// 4. EnemyTraits - Optional modifiers (armored, regenerating, etc.)
//
// Example usage:
// ```
// let mettaur = EnemyBlueprint {
//     id: EnemyId::Mettaur,
//     name: "Mettaur",
//     stats: EnemyStats { base_hp: 40, contact_damage: 10, .. },
//     movement: MovementBehavior::Stationary,
//     attack: AttackBehavior::ShockWave { damage: 20, speed: 0.15 },
//     traits: vec![EnemyTrait::Armored { damage_reduction: 5 }],
//     visuals: EnemyVisuals { ... },
// };
// ```

mod behaviors;
mod blueprints;
mod components;
mod systems;
mod visuals;

pub use behaviors::*;
pub use blueprints::*;
pub use components::*;
pub use systems::*;
pub use visuals::*;

use bevy::prelude::*;

/// Plugin that registers all enemy-related systems
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (execute_movement_behavior, execute_attack_behavior)
                .chain()
                .run_if(in_state(crate::components::GameState::Playing))
                .run_if(crate::systems::intro::intro_complete),
        );
    }
}
