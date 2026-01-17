// ============================================================================
// Legacy Enemy AI - for backward compatibility
// ============================================================================
//
// NOTE: This module contains the OLD enemy AI systems.
// New enemies should use the behavior system in src/enemies/ instead.
// These systems only run on enemies that DON'T have the BehaviorEnemy marker.
//
// The new system provides:
// - Composable MovementBehavior (random, chase, patrol, teleport, etc.)
// - Composable AttackBehavior (projectile, melee, area, laser, etc.)
// - Composable EnemyTraits (armor, regen, enrage, etc.)
//
// See src/enemies/blueprints.rs for how to define new enemies.

use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;
use crate::enemies::BehaviorEnemy;

/// Legacy enemy AI: random movement within enemy territory
/// Only runs on enemies WITHOUT the BehaviorEnemy marker
pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut GridPosition, &mut EnemyAI), (With<Enemy>, Without<BehaviorEnemy>)>,
) {
    let mut rng = rand::rng();

    for (mut pos, mut ai) in &mut query {
        ai.move_timer.tick(time.delta());

        if ai.move_timer.is_finished() {
            // Random direction: 0=up, 1=down, 2=left, 3=right, 4+=stay
            let direction = rng.random_range(0..6);

            let (dx, dy) = match direction {
                0 => (0, 1),  // up
                1 => (0, -1), // down
                2 => (-1, 0), // left
                3 => (1, 0),  // right
                _ => (0, 0),  // stay in place
            };

            let new_x = pos.x + dx;
            let new_y = pos.y + dy;

            // Constrain to enemy territory (right side of grid)
            if (PLAYER_AREA_WIDTH..GRID_WIDTH).contains(&new_x) && (0..GRID_HEIGHT).contains(&new_y)
            {
                pos.x = new_x;
                pos.y = new_y;
            }
        }
    }
}

/// Legacy enemy AI: shoot projectiles toward player
/// Only runs on enemies WITHOUT the BehaviorEnemy marker
pub fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (&GridPosition, &mut EnemyAI, &mut Sprite, &BaseColor),
        (With<Enemy>, Without<BehaviorEnemy>),
    >,
) {
    for (pos, mut ai, mut sprite, base_color) in &mut query {
        // Check if we are currently charging a shot
        if let Some(timer) = &mut ai.charge_timer {
            timer.tick(time.delta());

            // Visual feedback: rapid flashing during charge
            // Use sine wave for smooth pulsing or modulo for strobe
            let t = timer.elapsed_secs();
            if (t * 30.0).sin() > 0.0 {
                sprite.color = Color::srgb(1.0, 0.3, 0.3); // Bright red warning
            } else {
                sprite.color = base_color.0;
            }

            // Charge complete? Fire!
            if timer.is_finished() {
                // Spawn enemy bullet traveling left
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.9, 0.2, 0.3), // Red bullet for enemy
                        custom_size: Some(BULLET_DRAW_SIZE),
                        ..default()
                    },
                    Transform::default(),
                    GridPosition { x: pos.x, y: pos.y },
                    RenderConfig {
                        offset: Vec2::new(-BULLET_OFFSET.x, BULLET_OFFSET.y), // Offset to the left
                        base_z: Z_BULLET,
                    },
                    Bullet,
                    EnemyBullet,
                    MoveTimer(Timer::from_seconds(BULLET_MOVE_TIMER, TimerMode::Repeating)),
                ));

                // Reset state
                ai.charge_timer = None;
                ai.shoot_timer.reset(); // Restart the cooldown timer
                sprite.color = base_color.0; // Restore original color
            }
        } else {
            // Not charging, tick the cooldown
            ai.shoot_timer.tick(time.delta());

            if ai.shoot_timer.is_finished() {
                // Cooldown done, start charging
                ai.charge_timer = Some(Timer::from_seconds(ENEMY_CHARGE_TIME, TimerMode::Once));
            }
        }
    }
}
