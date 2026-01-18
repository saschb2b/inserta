use bevy::prelude::*;

use crate::actions::{ActionSlot, ActionState};
use crate::components::{ActionChargeBar, ActionCooldownOverlay};
use crate::constants::*;
use crate::systems::setup::ActionReadyIndicator;

/// Updates the action bar UI based on action states
pub fn update_action_bar_ui(
    action_query: Query<&ActionSlot>,
    mut cooldown_query: Query<(&ActionCooldownOverlay, &mut Sprite, &mut Transform)>,
    mut charge_query: Query<
        (&ActionChargeBar, &mut Sprite, &mut Visibility),
        Without<ActionCooldownOverlay>,
    >,
    mut ready_query: Query<
        (&ActionReadyIndicator, &mut Visibility),
        (Without<ActionCooldownOverlay>, Without<ActionChargeBar>),
    >,
) {
    for action in &action_query {
        // Update cooldown overlay
        for (overlay, mut sprite, mut transform) in &mut cooldown_query {
            if overlay.slot_index == action.slot_index {
                match action.state {
                    ActionState::OnCooldown => {
                        // Show cooldown from top to bottom (remaining cooldown)
                        let remaining = 1.0 - action.cooldown_progress();
                        let height = (ACTION_SLOT_SIZE - 4.0) * remaining;
                        sprite.custom_size = Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, height));
                        // Position from top
                        transform.translation.y = (ACTION_SLOT_SIZE - 4.0 - height) / 2.0;
                    }
                    _ => {
                        sprite.custom_size = Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 0.0));
                    }
                }
            }
        }

        // Update charge bar
        for (charge_bar, mut sprite, mut visibility) in &mut charge_query {
            if charge_bar.slot_index == action.slot_index {
                if action.state == ActionState::Charging {
                    *visibility = Visibility::Visible;
                    let progress = action.charge_progress();
                    let width = (ACTION_SLOT_SIZE - 8.0) * progress;
                    sprite.custom_size = Some(Vec2::new(width, 4.0));
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }

        // Update ready indicator
        for (indicator, mut visibility) in &mut ready_query {
            if indicator.slot_index == action.slot_index {
                *visibility = if action.is_ready() {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
