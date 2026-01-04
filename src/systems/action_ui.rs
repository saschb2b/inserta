use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

/// Spawns the action bar UI at the bottom of the screen
pub fn setup_action_bar(mut commands: Commands) {
    // Calculate total width for centering
    let total_width = (ACTION_SLOT_SIZE * 2.0) + ACTION_SLOT_SPACING; // Only 2 slots for now
    let start_x = -total_width / 2.0 + ACTION_SLOT_SIZE / 2.0;

    // Spawn action bar container
    commands
        .spawn((
            Transform::from_xyz(0.0, ACTION_BAR_Y, Z_UI),
            Visibility::Visible,
            ActionBar,
        ))
        .with_children(|parent| {
            // Slot 1: Charged Shot
            let slot_index = 0;
            let x_offset = start_x;
            let key_label = "1";
            let icon_color = COLOR_CHARGED_SHOT_ICON;

            parent
                .spawn((
                    Sprite {
                        color: COLOR_ACTION_SLOT_BG,
                        custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(x_offset, 0.0, 0.0),
                    ActionSlotUI { slot_index },
                ))
                .with_children(|slot| {
                    // Border
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_SLOT_BORDER,
                            custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE + 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, -0.1),
                    ));

                    // Action icon
                    slot.spawn((
                        Sprite {
                            color: icon_color,
                            custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE * 0.6)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 2.0, 0.1),
                    ));

                    // Cooldown overlay
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_COOLDOWN,
                            custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 0.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.2),
                        ActionCooldownOverlay { slot_index },
                    ));

                    // Charge bar
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_CHARGE,
                            custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 + 6.0, 0.3),
                        Visibility::Hidden,
                        ActionChargeBar { slot_index },
                    ));

                    // Key label
                    slot.spawn((
                        Text2d::new(key_label),
                        TextColor(COLOR_ACTION_KEY_TEXT),
                        TextFont::from_font_size(14.0),
                        Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 - 12.0, 0.1),
                        ActionKeyText { slot_index },
                    ));

                    // Ready indicator
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_SLOT_READY,
                            custom_size: Some(Vec2::splat(8.0)),
                            ..default()
                        },
                        Transform::from_xyz(
                            ACTION_SLOT_SIZE / 2.0 - 8.0,
                            ACTION_SLOT_SIZE / 2.0 - 8.0,
                            0.3,
                        ),
                        ActionReadyIndicator { slot_index },
                    ));
                });

            // Slot 2: Heal
            let slot_index = 1;
            let x_offset = start_x + ACTION_SLOT_SIZE + ACTION_SLOT_SPACING;
            let key_label = "2";
            let icon_color = COLOR_HEAL_ICON;

            parent
                .spawn((
                    Sprite {
                        color: COLOR_ACTION_SLOT_BG,
                        custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(x_offset, 0.0, 0.0),
                    ActionSlotUI { slot_index },
                ))
                .with_children(|slot| {
                    // Border
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_SLOT_BORDER,
                            custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE + 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, -0.1),
                    ));

                    // Action icon
                    slot.spawn((
                        Sprite {
                            color: icon_color,
                            custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE * 0.6)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 2.0, 0.1),
                    ));

                    // Cooldown overlay
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_COOLDOWN,
                            custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 0.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.2),
                        ActionCooldownOverlay { slot_index },
                    ));

                    // Charge bar
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_CHARGE,
                            custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 4.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 + 6.0, 0.3),
                        Visibility::Hidden,
                        ActionChargeBar { slot_index },
                    ));

                    // Key label
                    slot.spawn((
                        Text2d::new(key_label),
                        TextColor(COLOR_ACTION_KEY_TEXT),
                        TextFont::from_font_size(14.0),
                        Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 - 12.0, 0.1),
                        ActionKeyText { slot_index },
                    ));

                    // Ready indicator
                    slot.spawn((
                        Sprite {
                            color: COLOR_ACTION_SLOT_READY,
                            custom_size: Some(Vec2::splat(8.0)),
                            ..default()
                        },
                        Transform::from_xyz(
                            ACTION_SLOT_SIZE / 2.0 - 8.0,
                            ACTION_SLOT_SIZE / 2.0 - 8.0,
                            0.3,
                        ),
                        ActionReadyIndicator { slot_index },
                    ));
                });
        });
}

/// Marker for the ready indicator dot
#[derive(Component)]
pub struct ActionReadyIndicator {
    pub slot_index: usize,
}

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

/// Spawn the actual ActionSlot components (called from setup)
pub fn spawn_player_actions(mut commands: Commands) {
    // Charged Shot - Slot 1
    commands.spawn(ActionSlot::new(
        0,
        ActionType::ChargedShot,
        CHARGED_SHOT_COOLDOWN,
        CHARGED_SHOT_CHARGE_TIME,
    ));

    // Heal - Slot 2
    commands.spawn(ActionSlot::new(
        1,
        ActionType::Heal,
        HEAL_COOLDOWN,
        HEAL_CHARGE_TIME,
    ));
}
