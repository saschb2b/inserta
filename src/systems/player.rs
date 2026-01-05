use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

/// Player movement system - handles WASD/Arrow key input
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

// NOTE: Shooting is now handled by the weapon system in src/weapons/mod.rs
// The player_shoot function has been removed and replaced with weapon_input_system
