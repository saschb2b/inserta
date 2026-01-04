use bevy::prelude::*;

mod assets;
mod components;
mod constants;
mod systems;

use components::{InputCooldown, ShootCooldown};
use constants::{MOVE_COOLDOWN, SHOOT_COOLDOWN};
use systems::{
    animation::{animate_player, animate_slime},
    combat::{
        bullet_hit_enemy, bullet_movement, bullet_tile_highlight, enemy_flash, muzzle_lifetime,
    },
    common::update_transforms,
    player::{move_player, player_shoot},
    setup::setup,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Megaman BN Style Arena".into(),
                        resolution: (800.0, 600.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(InputCooldown(Timer::from_seconds(
            MOVE_COOLDOWN,
            TimerMode::Once,
        )))
        .insert_resource(ShootCooldown(Timer::from_seconds(
            SHOOT_COOLDOWN,
            TimerMode::Once,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                player_shoot,
                animate_player,
                animate_slime,
                bullet_movement,
                bullet_hit_enemy,
                bullet_tile_highlight,
                enemy_flash,
                muzzle_lifetime,
                update_transforms,
            ),
        )
        .run();
}
