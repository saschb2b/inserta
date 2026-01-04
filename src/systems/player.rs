use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cooldown: ResMut<InputCooldown>,
    mut query: Query<&mut GridPosition, With<Player>>,
) {
    cooldown.0.tick(time.delta());

    if !cooldown.0.is_finished() {
        return;
    }

    let mut moved = false;
    let mut direction = IVec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1;
        moved = true;
    }

    if moved {
        for mut pos in &mut query {
            let new_x = pos.x + direction.x;
            let new_y = pos.y + direction.y;

            if new_y >= 0 && new_y < GRID_HEIGHT && new_x >= 0 && new_x < PLAYER_AREA_WIDTH {
                pos.x = new_x;
                pos.y = new_y;
                cooldown.0.reset();
            }
        }
    }
}

pub fn player_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cooldown: ResMut<ShootCooldown>,
    query: Query<&GridPosition, With<Player>>,
) {
    cooldown.0.tick(time.delta());

    if !cooldown.0.is_finished() {
        return;
    }

    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    for player_pos in &query {
        // Spawn bullets from the *tile*, not from sprite size/anchor.
        commands.spawn((
            Sprite {
                color: COLOR_BULLET,
                custom_size: Some(BULLET_DRAW_SIZE),
                ..default()
            },
            Transform::default(),
            GridPosition {
                x: player_pos.x,
                y: player_pos.y,
            },
            RenderConfig {
                offset: BULLET_OFFSET,
                base_z: Z_BULLET,
            },
            Bullet,
            MoveTimer(Timer::from_seconds(BULLET_MOVE_TIMER, TimerMode::Repeating)),
        ));

        // Optional muzzle flash (tile-based)
        commands.spawn((
            Sprite {
                color: COLOR_MUZZLE,
                custom_size: Some(Vec2::new(22.0, 12.0)),
                ..default()
            },
            Transform::default(),
            GridPosition {
                x: player_pos.x,
                y: player_pos.y,
            },
            RenderConfig {
                offset: MUZZLE_OFFSET,
                base_z: Z_BULLET + 1.0,
            },
            MuzzleFlash,
            Lifetime(Timer::from_seconds(MUZZLE_TIME, TimerMode::Once)),
        ));

        cooldown.0.reset();
        break;
    }
}
