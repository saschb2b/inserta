// ============================================================================
// Enemy Systems - Execute behaviors based on components
// ============================================================================

use bevy::image::TextureAtlas;
use bevy::prelude::*;
use rand::Rng;

use super::{
    AttackBehavior, AttackState, BehaviorEnemy, ChargingTelegraph, EnemyAnimState, EnemyAttack,
    EnemyMovement, EnemyStats, EnemyTraitContainer, MovementBehavior,
};
use crate::assets::{ProjectileAnimation, ProjectileSprites};
use crate::components::{
    BaseColor, Bullet, EnemyBullet, GridPosition, Health, MoveTimer, RenderConfig, TargetsTiles,
};
use crate::constants::*;

// ============================================================================
// Movement System
// ============================================================================

/// Execute movement behaviors for all enemies using the new system
pub fn execute_movement_behavior(
    time: Res<Time>,
    // NOTE: player_query removed to avoid conflict with move_player system
    // For behaviors that need player position (ChasePlayer, MirrorPlayer),
    // we'd need to either chain systems or use a resource to share player position
    mut enemy_query: Query<
        (&mut GridPosition, &mut EnemyMovement, &EnemyStats),
        With<BehaviorEnemy>,
    >,
) {
    let player_pos: Option<&GridPosition> = None; // TODO: Get from resource
    let mut rng = rand::rng();

    for (mut pos, mut movement, stats) in &mut enemy_query {
        movement.move_timer.tick(time.delta());

        if !movement.move_timer.just_finished() {
            continue;
        }

        // Clone behavior to avoid borrow conflict with state
        let behavior = movement.behavior.clone();
        let (dx, dy) = calculate_movement(
            &behavior,
            &mut movement.state,
            &pos,
            player_pos,
            stats.move_speed,
            &mut rng,
        );

        // Apply movement within enemy territory
        let new_x = pos.x + dx;
        let new_y = pos.y + dy;

        if is_valid_enemy_position(new_x, new_y) {
            pos.x = new_x;
            pos.y = new_y;
        }
    }
}

/// Calculate movement delta based on behavior
fn calculate_movement(
    behavior: &MovementBehavior,
    state: &mut super::MovementState,
    pos: &GridPosition,
    player_pos: Option<&GridPosition>,
    _speed_mult: f32,
    rng: &mut impl Rng,
) -> (i32, i32) {
    match behavior {
        MovementBehavior::Stationary => (0, 0),

        MovementBehavior::Random { idle_chance } => {
            // Random chance to stay idle
            if rng.random::<f32>() < *idle_chance {
                return (0, 0);
            }

            // Random direction
            match rng.random_range(0..4) {
                0 => (0, 1),  // up
                1 => (0, -1), // down
                2 => (-1, 0), // left
                3 => (1, 0),  // right
                _ => (0, 0),
            }
        }

        MovementBehavior::ChaseRow => {
            // Move to match player's Y position
            if let Some(player) = player_pos {
                if pos.y < player.y {
                    (0, 1)
                } else if pos.y > player.y {
                    (0, -1)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        }

        MovementBehavior::ChasePlayer => {
            if let Some(player) = player_pos {
                // Prioritize getting in the same row first
                if pos.y != player.y {
                    if pos.y < player.y { (0, 1) } else { (0, -1) }
                } else if pos.x > PLAYER_AREA_WIDTH {
                    // Move toward player (but stay in enemy territory)
                    (-1, 0)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        }

        MovementBehavior::PatrolHorizontal => {
            let dx = if state.patrol_forward { 1 } else { -1 };
            let new_x = pos.x + dx;

            // Reverse at boundaries
            if !is_valid_enemy_position(new_x, pos.y) {
                state.patrol_forward = !state.patrol_forward;
                (if state.patrol_forward { 1 } else { -1 }, 0)
            } else {
                (dx, 0)
            }
        }

        MovementBehavior::PatrolVertical => {
            let dy = if state.patrol_forward { 1 } else { -1 };
            let new_y = pos.y + dy;

            // Reverse at boundaries
            if !(0..GRID_HEIGHT).contains(&new_y) {
                state.patrol_forward = !state.patrol_forward;
                (0, if state.patrol_forward { 1 } else { -1 })
            } else {
                (0, dy)
            }
        }

        MovementBehavior::BackRowOnly => {
            // Stay at rightmost column, move vertically only
            if pos.x < GRID_WIDTH - 1 {
                (1, 0) // Move to back
            } else {
                // Random vertical movement
                match rng.random_range(0..3) {
                    0 => (0, 1),
                    1 => (0, -1),
                    _ => (0, 0),
                }
            }
        }

        MovementBehavior::MirrorPlayer => {
            if let Some(player) = player_pos {
                if pos.y < player.y {
                    (0, 1)
                } else if pos.y > player.y {
                    (0, -1)
                } else {
                    (0, 0)
                }
            } else {
                (0, 0)
            }
        }

        // More complex behaviors that need state management
        MovementBehavior::HideAndPeek { .. } => {
            // Toggle hidden state (actual invulnerability handled elsewhere)
            state.is_hidden = !state.is_hidden;
            (0, 0)
        }

        MovementBehavior::Teleport { .. } => {
            // Random position in enemy territory
            let new_x = rng.random_range(PLAYER_AREA_WIDTH..GRID_WIDTH);
            let new_y = rng.random_range(0..GRID_HEIGHT);
            (new_x - pos.x, new_y - pos.y)
        }

        MovementBehavior::Advance { max_advance } => {
            let min_x = GRID_WIDTH - *max_advance;
            if pos.x > min_x && rng.random::<f32>() < 0.5 {
                (-1, 0) // Advance
            } else if pos.x < GRID_WIDTH - 1 && rng.random::<f32>() < 0.3 {
                (1, 0) // Retreat
            } else {
                // Random vertical
                match rng.random_range(0..3) {
                    0 => (0, 1),
                    1 => (0, -1),
                    _ => (0, 0),
                }
            }
        }
    }
}

/// Check if a position is valid for an enemy
fn is_valid_enemy_position(x: i32, y: i32) -> bool {
    (PLAYER_AREA_WIDTH..GRID_WIDTH).contains(&x) && (0..GRID_HEIGHT).contains(&y)
}

// ============================================================================
// Attack System
// ============================================================================

/// Execute attack behaviors for all enemies using the new system
pub fn execute_attack_behavior(
    mut commands: Commands,
    time: Res<Time>,
    projectiles: Res<ProjectileSprites>,
    mut enemy_query: Query<
        (Entity, &GridPosition, &mut EnemyAttack, &mut EnemyAnimState),
        With<BehaviorEnemy>,
    >,
) {
    for (entity, pos, mut attack, mut anim_state) in &mut enemy_query {
        match attack.state {
            AttackState::Ready => {
                // Tick cooldown
                attack.cooldown_timer.tick(time.delta());

                if attack.cooldown_timer.just_finished() {
                    // Start charging
                    let charge_time = attack.behavior.charge_time();
                    if charge_time > 0.0 {
                        attack.charge_timer =
                            Some(Timer::from_seconds(charge_time, TimerMode::Once));
                        attack.state = AttackState::Charging;
                        *anim_state = EnemyAnimState::Charging;
                        // Add telegraph component for visual effect
                        commands.entity(entity).insert(ChargingTelegraph {
                            timer: Timer::from_seconds(charge_time, TimerMode::Once),
                        });
                    } else {
                        // No charge time, attack immediately
                        attack.state = AttackState::Attacking;
                        *anim_state = EnemyAnimState::Attacking;
                    }
                }
            }

            AttackState::Charging => {
                if let Some(ref mut timer) = attack.charge_timer {
                    timer.tick(time.delta());

                    if timer.just_finished() {
                        attack.state = AttackState::Attacking;
                        *anim_state = EnemyAnimState::Attacking;
                        // Remove telegraph component
                        commands.entity(entity).remove::<ChargingTelegraph>();
                    }
                }
            }

            AttackState::Attacking => {
                // Execute the attack based on behavior
                execute_attack(&mut commands, &attack.behavior, pos, &projectiles);

                // Move to recovery/ready
                attack.state = AttackState::Ready;
                attack.cooldown_timer.reset();
                attack.charge_timer = None;
                *anim_state = EnemyAnimState::Idle;
            }

            AttackState::Recovering => {
                // Optional recovery phase (not used currently)
                attack.state = AttackState::Ready;
                *anim_state = EnemyAnimState::Idle;
            }
        }
    }
}

/// Execute a specific attack type
fn execute_attack(
    commands: &mut Commands,
    behavior: &AttackBehavior,
    pos: &GridPosition,
    projectiles: &ProjectileSprites,
) {
    match behavior {
        AttackBehavior::None => {}

        AttackBehavior::Projectile {
            damage: _, speed, ..
        } => {
            spawn_enemy_projectile(commands, pos.x, pos.y, *speed, projectiles);
        }

        AttackBehavior::ProjectileSpread {
            damage: _,
            speed,
            row_offsets,
            ..
        } => {
            for offset in row_offsets {
                let target_y = pos.y + offset;
                if (0..GRID_HEIGHT).contains(&target_y) {
                    spawn_enemy_projectile(commands, pos.x, target_y, *speed, projectiles);
                }
            }
        }

        AttackBehavior::ShockWave {
            damage: _, speed, ..
        } => {
            // Shockwave is similar to projectile but could have different visuals
            spawn_enemy_projectile(commands, pos.x, pos.y, *speed, projectiles);
        }

        AttackBehavior::Melee { .. } => {
            // TODO: Implement melee hit detection
        }

        AttackBehavior::AreaAttack { .. } => {
            // TODO: Implement area attack
        }

        AttackBehavior::Bomb { .. } => {
            // TODO: Implement bomb spawning
        }

        AttackBehavior::LaserBeam { .. } => {
            // TODO: Implement laser beam
        }

        AttackBehavior::Summon { .. } => {
            // TODO: Implement summon
        }
    }
}

/// Spawn an enemy projectile traveling left
fn spawn_enemy_projectile(
    commands: &mut Commands,
    x: i32,
    y: i32,
    speed: f32,
    projectiles: &ProjectileSprites,
) {
    // Convert speed (tiles per second) to move timer duration
    let move_timer = if speed > 0.0 {
        1.0 / speed
    } else {
        BULLET_MOVE_TIMER
    };

    commands.spawn((
        Sprite {
            image: projectiles.blaster_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: projectiles.blaster_layout.clone(),
                index: 1, // Start at travel frame
            }),
            custom_size: Some(BULLET_DRAW_SIZE),
            ..default()
        },
        Transform::default(),
        GridPosition { x, y },
        RenderConfig {
            offset: Vec2::new(-BULLET_OFFSET.x, BULLET_OFFSET.y),
            base_z: Z_BULLET,
        },
        Bullet,
        EnemyBullet,
        ProjectileAnimation::blaster(),
        MoveTimer(Timer::from_seconds(move_timer, TimerMode::Repeating)),
        TargetsTiles::single(), // Highlight tile at projectile's position
    ));
}

// ============================================================================
// Charging Telegraph Visual Effect
// ============================================================================

/// Animate the charging telegraph (flashing effect)
pub fn animate_charging_telegraph(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &BaseColor, &mut ChargingTelegraph)>,
) {
    for (entity, mut sprite, base_color, mut telegraph) in &mut query {
        telegraph.timer.tick(time.delta());

        // Flash effect using sine wave
        let t = telegraph.timer.elapsed_secs();
        if (t * 30.0).sin() > 0.0 {
            sprite.color = Color::srgb(1.0, 0.3, 0.3); // Red warning flash
        } else {
            sprite.color = base_color.0;
        }

        // Remove when done (backup cleanup - normally removed by attack system)
        if telegraph.timer.just_finished() {
            sprite.color = base_color.0;
            commands.entity(entity).remove::<ChargingTelegraph>();
        }
    }
}

// ============================================================================
// Trait System
// ============================================================================

/// Apply trait effects (regeneration, enrage, etc.)
pub fn apply_enemy_traits(
    time: Res<Time>,
    mut query: Query<(&mut Health, &mut EnemyTraitContainer, &EnemyStats), With<BehaviorEnemy>>,
) {
    for (mut health, mut traits, _stats) in &mut query {
        // HP Regeneration
        if let Some(ref mut timer) = traits.hp_regen_timer {
            timer.tick(time.delta());
            if timer.just_finished() {
                let regen = traits.traits.hp_regen_per_sec as i32;
                health.current = (health.current + regen).min(health.max);
            }
        }

        // Enrage check
        if let Some(ref enrage) = traits.traits.enrage {
            let hp_percent = health.current as f32 / health.max as f32;
            if hp_percent <= enrage.threshold {
                // TODO: Apply enrage multipliers to movement/attack timers
            }
        }
    }
}
