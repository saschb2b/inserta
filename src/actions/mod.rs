// ============================================================================
// Actions Module - Battle Chip / Action System
// ============================================================================
//
// This module implements a composable action system inspired by MMBN Battle Chips.
// Actions are defined using blueprints that combine:
// - Stats (damage, cooldown, charge time)
// - Targeting (self, tiles, rows, columns, areas)
// - Effects (damage, heal, shield, status)
// - Visuals (colors, sprites, animations)
//
// Adding a new action:
// 1. Add variant to ActionId enum in components.rs
// 2. Create blueprint function in blueprints.rs
// 3. Add match arm in ActionBlueprint::get()
// 4. (Optional) Add visual assets

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

/// Plugin that registers all action-related systems
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                action_input_system,
                execute_pending_actions,
                update_action_cooldowns,
                // Effect systems
                process_damage_effects,
                process_heal_effects,
                process_shield_effects,
                update_active_shields,
                // Visual systems
                update_action_visuals,
                despawn_action_visuals,
            )
                .chain()
                .run_if(in_state(crate::components::GameState::Playing)),
        );
    }
}
