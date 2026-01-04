use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

/// Process action inputs (keys 1-4)
pub fn action_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &GridPosition, &mut Health), With<Player>>,
    mut action_query: Query<&mut ActionSlot>,
    mut hp_text_query: Query<&mut Text2d, With<PlayerHealthText>>,
    mut commands: Commands,
) {
    // Key mappings: 1 = ChargedShot, 2 = Heal
    let keys = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
    ];

    for (player_entity, player_pos, mut health) in &mut player_query {
        for mut action in &mut action_query {
            // Update cooldown timers
            if action.state == ActionState::OnCooldown {
                action.cooldown_timer.tick(time.delta());
                if action.cooldown_timer.is_finished() {
                    action.state = ActionState::Ready;
                }
            }

            // Update charge timers
            if action.state == ActionState::Charging {
                if let Some(ref mut timer) = action.charge_timer {
                    timer.tick(time.delta());
                    if timer.is_finished() {
                        // Execute the action!
                        execute_action(
                            &mut commands,
                            &mut action,
                            player_entity,
                            *player_pos,
                            &mut health,
                            &mut hp_text_query,
                        );
                    }
                }
            }

            // Check for key press
            for (key, slot_idx) in &keys {
                if action.slot_index == *slot_idx && keyboard.just_pressed(*key) {
                    if action.is_ready() {
                        if action.charge_duration > 0.0 {
                            // Start charging
                            action.start_charging();
                        } else {
                            // Instant action
                            execute_action(
                                &mut commands,
                                &mut action,
                                player_entity,
                                *player_pos,
                                &mut health,
                                &mut hp_text_query,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn execute_action(
    commands: &mut Commands,
    action: &mut ActionSlot,
    player_entity: Entity,
    player_pos: GridPosition,
    health: &mut Health,
    hp_text_query: &mut Query<&mut Text2d, With<PlayerHealthText>>,
) {
    match action.action_type {
        ActionType::ChargedShot => {
            spawn_charged_shot(commands, player_pos);
        }
        ActionType::Heal => {
            apply_heal(commands, player_entity, health, hp_text_query);
        }
    }
    action.start_cooldown();
}

fn spawn_charged_shot(commands: &mut Commands, player_pos: GridPosition) {
    commands.spawn((
        Sprite {
            color: COLOR_CHARGED_SHOT,
            custom_size: Some(CHARGED_SHOT_SIZE),
            ..default()
        },
        Transform::default(),
        GridPosition {
            x: player_pos.x,
            y: player_pos.y,
        },
        RenderConfig {
            offset: BULLET_OFFSET,
            base_z: Z_BULLET + 0.5,
        },
        Bullet,
        ChargedShot {
            damage: CHARGED_SHOT_DAMAGE,
        },
        MoveTimer(Timer::from_seconds(BULLET_MOVE_TIMER, TimerMode::Repeating)),
    ));
}

fn apply_heal(
    commands: &mut Commands,
    player_entity: Entity,
    health: &mut Health,
    hp_text_query: &mut Query<&mut Text2d, With<PlayerHealthText>>,
) {
    health.current = (health.current + HEAL_AMOUNT).min(health.max);

    // Update HP text
    for mut text in hp_text_query.iter_mut() {
        text.0 = format!("HP: {}", health.current);
    }

    // Flash green to indicate heal
    commands
        .entity(player_entity)
        .insert(HealFlashTimer(Timer::from_seconds(0.3, TimerMode::Once)));
}

/// Component for heal flash effect
#[derive(Component)]
pub struct HealFlashTimer(pub Timer);

/// System to handle heal flash visual
pub fn heal_flash_effect(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &BaseColor, &mut HealFlashTimer)>,
) {
    for (entity, mut sprite, base, mut flash) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.is_finished() {
            sprite.color = base.0;
            commands.entity(entity).remove::<HealFlashTimer>();
        } else {
            // Green flash for heal
            let t = flash.0.fraction();
            let green = Color::srgb(0.3, 1.0, 0.4);
            sprite.color = base.0.mix(&green, 1.0 - t);
        }
    }
}

/// Handle charged shot hitting enemies (more damage than regular bullets)
pub fn charged_shot_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GridPosition, &ChargedShot), With<Bullet>>,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    for (bullet_entity, bullet_pos, charged_shot) in &bullet_query {
        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            if bullet_pos == enemy_pos {
                health.current -= charged_shot.damage;
                commands.entity(bullet_entity).despawn();

                // Update HP text
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = health.current.max(0).to_string();
                    }
                }

                // Flash feedback
                commands
                    .entity(enemy_entity)
                    .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}
