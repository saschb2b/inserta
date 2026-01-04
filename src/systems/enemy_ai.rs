use bevy::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::constants::*;

/// Enemy AI: random movement within enemy territory
pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut GridPosition, &mut EnemyAI), With<Enemy>>,
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
            if new_x >= PLAYER_AREA_WIDTH && new_x < GRID_WIDTH && new_y >= 0 && new_y < GRID_HEIGHT
            {
                pos.x = new_x;
                pos.y = new_y;
            }
        }
    }
}

/// Enemy AI: shoot projectiles toward player
pub fn enemy_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&GridPosition, &mut EnemyAI), With<Enemy>>,
) {
    for (pos, mut ai) in &mut query {
        ai.shoot_timer.tick(time.delta());

        if ai.shoot_timer.is_finished() {
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
        }
    }
}
