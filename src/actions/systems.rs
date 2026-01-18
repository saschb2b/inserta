// ============================================================================
// Action Systems - Execution and effects
// ============================================================================

use bevy::prelude::*;

use super::{
    ActionBlueprint, ActionEffect, ActionId, ActionSlot, ActionState, ActionTarget, ActionVisual,
    ActiveShield, DamageZone, Element, HealFlash, ShieldType,
};
use crate::components::{
    BaseColor, CleanupOnStateExit, Enemy, FlashTimer, GameState, GridPosition, Health, HealthText,
    Player, PlayerHealthText, TargetsTiles,
};
use crate::constants::*;
use crate::resources::ArenaLayout;

// ============================================================================
// Input Handling
// ============================================================================

/// Process action inputs (keys 1-3)
pub fn action_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    _layout: Res<ArenaLayout>,
    player_query: Query<(Entity, &GridPosition), With<Player>>,
    mut action_query: Query<&mut ActionSlot>,
    mut commands: Commands,
) {
    let keys = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
    ];

    let gamepad_buttons = [
        (GamepadButton::West, 0),
        (GamepadButton::North, 1),
        (GamepadButton::East, 2),
        (GamepadButton::South, 3),
    ];

    let Ok((player_entity, player_pos)) = player_query.single() else {
        return;
    };

    for mut action in &mut action_query {
        // Update cooldown timers
        if action.state == ActionState::OnCooldown {
            action.cooldown_timer.tick(time.delta());
            if action.cooldown_timer.is_finished() {
                action.state = ActionState::Ready;
            }
        }

        // Update charge timers - execute when done
        if action.state == ActionState::Charging {
            if let Some(ref mut timer) = action.charge_timer {
                timer.tick(time.delta());
                if timer.is_finished() {
                    // Queue the action for execution
                    queue_action(&mut commands, action.action_id, player_entity, *player_pos);
                    action.start_cooldown();
                }
            }
        }

        // Check for input
        let mut triggered = false;

        for (key, slot_idx) in &keys {
            if action.slot_index == *slot_idx && keyboard.just_pressed(*key) {
                triggered = true;
            }
        }

        for gamepad in gamepads.iter() {
            for (button, slot_idx) in &gamepad_buttons {
                if action.slot_index == *slot_idx && gamepad.just_pressed(*button) {
                    triggered = true;
                }
            }
        }

        if triggered && action.is_ready() {
            let blueprint = ActionBlueprint::get(action.action_id);

            if blueprint.charge_time > 0.0 {
                action.start_charging();
            } else {
                // Instant action - queue immediately
                queue_action(&mut commands, action.action_id, player_entity, *player_pos);
                action.start_cooldown();
            }
        }
    }
}

/// Queue an action for execution
fn queue_action(
    commands: &mut Commands,
    action_id: ActionId,
    source_entity: Entity,
    source_position: GridPosition,
) {
    commands.spawn((
        super::PendingAction {
            action_id,
            source_entity,
            source_position: (source_position.x, source_position.y),
        },
        CleanupOnStateExit(GameState::Playing),
    ));
}

// ============================================================================
// Action Execution
// ============================================================================

/// Execute pending actions
pub fn execute_pending_actions(
    mut commands: Commands,
    pending_query: Query<(Entity, &super::PendingAction)>,
    mut player_query: Query<&mut Health, With<Player>>,
    mut hp_text_query: Query<&mut Text2d, With<PlayerHealthText>>,
    layout: Res<ArenaLayout>,
) {
    for (pending_entity, pending) in &pending_query {
        let blueprint = ActionBlueprint::get(pending.action_id);

        // Execute based on effect type
        match &blueprint.effect {
            ActionEffect::Heal { amount } => {
                execute_heal(
                    &mut commands,
                    pending.source_entity,
                    *amount,
                    &mut player_query,
                    &mut hp_text_query,
                );
            }

            ActionEffect::Shield {
                duration,
                threshold,
            } => {
                execute_shield(&mut commands, pending.source_entity, *duration, *threshold);
            }

            ActionEffect::Invisibility { duration } => {
                execute_invis(&mut commands, pending.source_entity, *duration);
            }

            ActionEffect::Damage {
                amount, element, ..
            } => {
                execute_damage_action(
                    &mut commands,
                    &blueprint,
                    pending.source_position,
                    *amount,
                    *element,
                    &layout,
                );
            }

            ActionEffect::Combo { effects } => {
                // Execute each sub-effect
                for effect in effects {
                    match effect {
                        ActionEffect::Heal { amount } => {
                            execute_heal(
                                &mut commands,
                                pending.source_entity,
                                *amount,
                                &mut player_query,
                                &mut hp_text_query,
                            );
                        }
                        ActionEffect::Damage {
                            amount, element, ..
                        } => {
                            execute_damage_action(
                                &mut commands,
                                &blueprint,
                                pending.source_position,
                                *amount,
                                *element,
                                &layout,
                            );
                        }
                        _ => {
                            // Other effects handled elsewhere
                        }
                    }
                }
            }

            _ => {
                // Other effects (panel manipulation, etc.) - TODO
            }
        }

        // Despawn the pending action
        commands.entity(pending_entity).despawn();
    }
}

/// Execute a heal effect
fn execute_heal(
    commands: &mut Commands,
    target: Entity,
    amount: i32,
    player_query: &mut Query<&mut Health, With<Player>>,
    hp_text_query: &mut Query<&mut Text2d, With<PlayerHealthText>>,
) {
    if let Ok(mut health) = player_query.get_mut(target) {
        health.current = (health.current + amount).min(health.max);

        // Update HP text
        for mut text in hp_text_query.iter_mut() {
            text.0 = format!("HP: {}", health.current);
        }

        // Add heal flash
        commands.entity(target).insert(HealFlash {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
            heal_amount: amount,
        });
    }
}

/// Execute a shield effect
fn execute_shield(commands: &mut Commands, target: Entity, duration: f32, threshold: Option<i32>) {
    let shield_type = match threshold {
        None => ShieldType::Basic,
        Some(0) => ShieldType::Barrier,
        Some(_) => ShieldType::Aura,
    };

    commands.entity(target).insert(ActiveShield {
        duration_timer: Timer::from_seconds(duration, TimerMode::Once),
        damage_threshold: threshold,
        shield_type,
    });

    // Spawn shield visual as child
    commands.entity(target).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: Color::srgba(0.3, 0.6, 1.0, 0.5),
                custom_size: Some(Vec2::new(120.0, 160.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 40.0, 0.5),
            ShieldVisualMarker,
        ));
    });
}

/// Marker for shield visuals
#[derive(Component)]
pub struct ShieldVisualMarker;

/// Execute an invisibility effect
fn execute_invis(commands: &mut Commands, target: Entity, duration: f32) {
    commands.entity(target).insert(ActiveShield {
        duration_timer: Timer::from_seconds(duration, TimerMode::Once),
        damage_threshold: None,
        shield_type: ShieldType::Invis,
    });
}

/// Execute a damage-dealing action
fn execute_damage_action(
    commands: &mut Commands,
    blueprint: &ActionBlueprint,
    source_pos: (i32, i32),
    damage: i32,
    element: Element,
    layout: &ArenaLayout,
) {
    let hit_tiles = calculate_hit_tiles(&blueprint.target, source_pos);

    if hit_tiles.is_empty() {
        return;
    }

    // Calculate visual position (center of affected area)
    let center_tile = hit_tiles[hit_tiles.len() / 2];
    let floor_pos = layout.tile_floor_world(center_tile.0, center_tile.1);

    // Spawn damage zone with visual
    commands.spawn((
        Sprite {
            color: blueprint.visuals.effect_color,
            custom_size: Some(blueprint.visuals.effect_size * layout.scale),
            ..default()
        },
        Transform::from_xyz(
            floor_pos.x,
            floor_pos.y + 20.0 * layout.scale,
            Z_BULLET + 1.0,
        ),
        DamageZone {
            damage,
            element,
            hit_tiles: hit_tiles.clone(),
            applied: false,
        },
        TargetsTiles::multiple(hit_tiles),
        ActionVisual {
            lifetime: Timer::from_seconds(blueprint.visuals.effect_duration, TimerMode::Once),
            source: None,
        },
        CleanupOnStateExit(GameState::Playing),
    ));
}

/// Calculate which tiles an action hits based on targeting
fn calculate_hit_tiles(target: &ActionTarget, source_pos: (i32, i32)) -> Vec<(i32, i32)> {
    match target {
        ActionTarget::OnSelf => vec![source_pos],

        ActionTarget::SingleTile { range } => {
            vec![(source_pos.0 + range, source_pos.1)]
        }

        ActionTarget::Column { x_offset } => {
            let target_x = source_pos.0 + x_offset;
            (0..GRID_HEIGHT).map(|y| (target_x, y)).collect()
        }

        ActionTarget::Row {
            x_offset,
            traveling,
        } => {
            let start_x = source_pos.0 + x_offset;
            if *traveling {
                // Hits entire row from start to edge
                (start_x..GRID_WIDTH).map(|x| (x, source_pos.1)).collect()
            } else {
                // Instant - hits just the row
                (start_x..GRID_WIDTH).map(|x| (x, source_pos.1)).collect()
            }
        }

        ActionTarget::Pattern { tiles } => tiles
            .iter()
            .map(|(dx, dy)| (source_pos.0 + dx, source_pos.1 + dy))
            .filter(|(x, y)| *x >= 0 && *x < GRID_WIDTH && *y >= 0 && *y < GRID_HEIGHT)
            .collect(),

        ActionTarget::Projectile { x_offset, .. } => {
            // For now, projectile just hits the first enemy in row
            // Full projectile system would track movement
            let start_x = source_pos.0 + x_offset;
            (start_x..GRID_WIDTH).map(|x| (x, source_pos.1)).collect()
        }

        ActionTarget::ProjectileSpread {
            x_offset,
            spread_rows,
        } => {
            let start_x = source_pos.0 + x_offset;
            let mut tiles = Vec::new();
            for row_offset in spread_rows {
                let row = source_pos.1 + row_offset;
                if row >= 0 && row < GRID_HEIGHT {
                    for x in start_x..GRID_WIDTH {
                        tiles.push((x, row));
                    }
                }
            }
            tiles
        }

        ActionTarget::AreaAroundSelf { radius } => {
            let mut tiles = Vec::new();
            for dx in -radius..=*radius {
                for dy in -radius..=*radius {
                    let x = source_pos.0 + dx;
                    let y = source_pos.1 + dy;
                    if x >= 0 && x < GRID_WIDTH && y >= 0 && y < GRID_HEIGHT {
                        tiles.push((x, y));
                    }
                }
            }
            tiles
        }

        ActionTarget::AreaAtPosition {
            x_offset,
            y_offset,
            pattern,
        } => {
            let center_x = source_pos.0 + x_offset;
            let center_y = source_pos.1 + y_offset;
            pattern
                .iter()
                .map(|(dx, dy)| (center_x + dx, center_y + dy))
                .filter(|(x, y)| *x >= 0 && *x < GRID_WIDTH && *y >= 0 && *y < GRID_HEIGHT)
                .collect()
        }

        ActionTarget::EnemyArea => {
            let mut tiles = Vec::new();
            for x in PLAYER_AREA_WIDTH..GRID_WIDTH {
                for y in 0..GRID_HEIGHT {
                    tiles.push((x, y));
                }
            }
            tiles
        }

        ActionTarget::RandomEnemy { count: _ } => {
            // TODO: Pick random tiles with enemies
            // For now, just return empty
            Vec::new()
        }
    }
}

// ============================================================================
// Cooldown Updates
// ============================================================================

/// Update cooldown timers (separate from input for clarity)
pub fn update_action_cooldowns(_time: Res<Time>, _action_query: Query<&mut ActionSlot>) {
    // Cooldown updates are now handled in action_input_system
    // This function is kept for potential future use
}

// ============================================================================
// Damage Processing
// ============================================================================

/// Process damage zones hitting enemies
pub fn process_damage_effects(
    mut commands: Commands,
    mut damage_query: Query<(Entity, &mut DamageZone)>,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    for (_zone_entity, mut zone) in &mut damage_query {
        if zone.applied {
            continue;
        }

        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            if zone
                .hit_tiles
                .iter()
                .any(|(x, y)| *x == enemy_pos.x && *y == enemy_pos.y)
            {
                // Apply damage with element bonus
                let final_damage = zone.damage;

                // TODO: Check enemy element and apply weakness bonus

                health.current -= final_damage;

                // Update HP text
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = health.current.max(0).to_string();
                    }
                }

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                } else {
                    commands
                        .entity(enemy_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }
            }
        }

        zone.applied = true;
    }
}

// ============================================================================
// Heal Processing
// ============================================================================

/// Process heal flash effects
pub fn process_heal_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &BaseColor, &mut HealFlash)>,
) {
    for (entity, mut sprite, base, mut flash) in &mut query {
        flash.timer.tick(time.delta());

        if flash.timer.is_finished() {
            sprite.color = base.0;
            commands.entity(entity).remove::<HealFlash>();
        } else {
            let t = flash.timer.fraction();
            let green = Color::srgb(0.3, 1.0, 0.4);
            sprite.color = base.0.mix(&green, 1.0 - t);
        }
    }
}

// ============================================================================
// Shield Processing
// ============================================================================

/// Process shield duration and removal
pub fn process_shield_effects(
    mut commands: Commands,
    shield_query: Query<&ActiveShield, With<Player>>,
    enemy_bullet_query: Query<(Entity, &GridPosition), With<crate::components::EnemyBullet>>,
    player_query: Query<&GridPosition, With<Player>>,
) {
    if shield_query.is_empty() {
        return;
    }

    let Ok(player_pos) = player_query.single() else {
        return;
    };

    // Block enemy bullets
    for (bullet_entity, bullet_pos) in &enemy_bullet_query {
        if bullet_pos == player_pos {
            commands.entity(bullet_entity).despawn();
        }
    }
}

/// Update active shields (duration countdown)
pub fn update_active_shields(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut ActiveShield, Option<&Children>), With<Player>>,
    shield_visual_query: Query<Entity, With<ShieldVisualMarker>>,
) {
    for (player_entity, mut shield, children) in &mut player_query {
        shield.duration_timer.tick(time.delta());

        if shield.duration_timer.is_finished() {
            commands.entity(player_entity).remove::<ActiveShield>();

            // Remove shield visuals
            if let Some(children) = children {
                for child in children.iter() {
                    if shield_visual_query.get(child).is_ok() {
                        commands.entity(child).despawn();
                    }
                }
            }
        }
    }
}

// ============================================================================
// Visual Updates
// ============================================================================

/// Update action visual effects (lifetimes, animations)
pub fn update_action_visuals(time: Res<Time>, mut query: Query<&mut ActionVisual>) {
    for mut visual in &mut query {
        visual.lifetime.tick(time.delta());
    }
}

/// Despawn expired action visuals
pub fn despawn_action_visuals(mut commands: Commands, query: Query<(Entity, &ActionVisual)>) {
    for (entity, visual) in &query {
        if visual.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
