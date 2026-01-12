use bevy::prelude::*;

use crate::components::{
    ActionSlot, ActionState, ActionType, BaseColor, Bullet, ChargedShot, Enemy, EnemyBullet,
    FlashTimer, GridPosition, Health, HealthText, Lifetime, Player, PlayerHealthText, Shield,
    WideSwordSlash,
};
use crate::constants::*;
use crate::systems::grid_utils::tile_floor_world;

/// Process action inputs (keys 1-3)
/// NOTE: Weapon shooting (Space key) is handled by the weapon system
pub fn action_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &GridPosition, &mut Health), With<Player>>,
    mut action_query: Query<&mut ActionSlot>,
    mut hp_text_query: Query<&mut Text2d, With<PlayerHealthText>>,
    mut commands: Commands,
) {
    // Key mappings: 1 = Heal, 2 = Shield, 3 = WideSword
    let keys = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
    ];

    // Gamepad mappings: West=Slot1, North=Slot2, East=Slot3
    let gamepad_buttons = [
        (GamepadButton::West, 0),
        (GamepadButton::North, 1),
        (GamepadButton::East, 2),
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
            let mut triggered = false;

            // Check Keyboard
            for (key, slot_idx) in &keys {
                if action.slot_index == *slot_idx && keyboard.just_pressed(*key) {
                    triggered = true;
                }
            }

            // Check Gamepad
            for gamepad in gamepads.iter() {
                for (button, slot_idx) in &gamepad_buttons {
                    if action.slot_index == *slot_idx && gamepad.just_pressed(*button) {
                        triggered = true;
                    }
                }
            }

            if triggered && action.is_ready() {
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

fn execute_action(
    commands: &mut Commands,
    action: &mut ActionSlot,
    player_entity: Entity,
    player_pos: GridPosition,
    health: &mut Health,
    hp_text_query: &mut Query<&mut Text2d, With<PlayerHealthText>>,
) {
    match action.action_type {
        ActionType::Heal => {
            apply_heal(commands, player_entity, health, hp_text_query);
        }
        ActionType::Shield => {
            activate_shield(commands, player_entity);
        }
        ActionType::WideSword => {
            spawn_widesword_slash(commands, player_pos);
        }
    }
    action.start_cooldown();
}

// NOTE: spawn_charged_shot was removed - charged shots are now handled by the weapon system

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

fn activate_shield(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).insert(Shield {
        duration_timer: Timer::from_seconds(SHIELD_DURATION, TimerMode::Once),
    });

    // Visual effect: spawn a shield sprite around the player
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: COLOR_SHIELD,
                custom_size: Some(Vec2::new(120.0, 160.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 40.0, 0.5),
            ShieldVisual,
        ));
    });
}

/// Marker for shield visual effect
#[derive(Component)]
pub struct ShieldVisual;

/// System to update shield duration and remove when expired
pub fn update_shield(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Shield, &Children), With<Player>>,
    shield_visual_query: Query<Entity, With<ShieldVisual>>,
) {
    for (player_entity, mut shield, children) in &mut player_query {
        shield.duration_timer.tick(time.delta());

        if shield.duration_timer.is_finished() {
            // Remove shield component
            commands.entity(player_entity).remove::<Shield>();

            // Remove shield visual
            for child in children.iter() {
                if shield_visual_query.get(child).is_ok() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
}

/// System to block incoming damage when shield is active
pub fn shield_blocks_damage(
    mut commands: Commands,
    shield_query: Query<&Shield, With<Player>>,
    bullet_query: Query<(Entity, &GridPosition), With<EnemyBullet>>,
    player_query: Query<&GridPosition, With<Player>>,
) {
    // Only check if player has shield
    if shield_query.is_empty() {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    // Destroy enemy bullets that hit player while shielded
    for (bullet_entity, bullet_pos) in &bullet_query {
        if bullet_pos == player_pos {
            commands.entity(bullet_entity).despawn();
            // TODO: Could add a "blocked" visual/sound effect here
        }
    }
}

fn spawn_widesword_slash(commands: &mut Commands, player_pos: GridPosition) {
    // WideSword hits the column in front of player (all 3 rows)
    let target_x = player_pos.x + 1;

    // If target column is outside enemy area, still spawn visual but no hits
    let hit_tiles: Vec<(i32, i32)> = if target_x >= PLAYER_AREA_WIDTH && target_x < GRID_WIDTH {
        (0..GRID_HEIGHT).map(|y| (target_x, y)).collect()
    } else {
        vec![]
    };

    // Calculate world position for the slash visual (center of target column)
    let center_y = 1; // Middle row
    let floor_pos = tile_floor_world(target_x, center_y);

    commands.spawn((
        Sprite {
            color: COLOR_WIDESWORD_SLASH,
            custom_size: Some(Vec2::new(80.0, TILE_STEP_Y * 3.0 + 40.0)), // Tall slash covering 3 rows
            ..default()
        },
        Transform::from_xyz(floor_pos.x, floor_pos.y + 20.0, Z_BULLET + 1.0),
        WideSwordSlash {
            damage: WIDESWORD_DAMAGE,
            hit_tiles,
        },
        Lifetime(Timer::from_seconds(
            WIDESWORD_SLASH_DURATION,
            TimerMode::Once,
        )),
    ));
}

/// System to handle WideSword hitting enemies
pub fn widesword_hit_enemy(
    mut commands: Commands,
    mut slash_query: Query<(Entity, &mut WideSwordSlash)>,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    for (_slash_entity, mut slash) in &mut slash_query {
        // Only process hits once (check if hit_tiles is not empty)
        if slash.hit_tiles.is_empty() {
            continue;
        }

        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            // Check if enemy is in any of the hit tiles
            if slash
                .hit_tiles
                .iter()
                .any(|(x, y)| *x == enemy_pos.x && *y == enemy_pos.y)
            {
                health.current -= slash.damage;

                // Update HP text
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = health.current.max(0).to_string();
                    }
                }

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                } else {
                    // Flash feedback only if still alive
                    commands
                        .entity(enemy_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }
            }
        }

        // Clear hit_tiles so we don't hit again
        slash.hit_tiles.clear();
    }
}

/// System to despawn WideSword slash after lifetime expires
pub fn despawn_widesword_slash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime), With<WideSwordSlash>>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
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

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                } else {
                    // Flash feedback only if still alive
                    commands
                        .entity(enemy_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }
            }
        }
    }
}
