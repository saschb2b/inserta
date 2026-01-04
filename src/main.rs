use bevy::prelude::*;

mod assets;
mod components;
mod constants;
mod systems;

use components::{InputCooldown, ShootCooldown};
use constants::{MOVE_COOLDOWN, SHOOT_COOLDOWN};
use systems::{
    action_ui::{setup_action_bar, spawn_player_actions, update_action_bar_ui},
    actions::{action_input, charged_shot_hit_enemy, heal_flash_effect},
    animation::{animate_player, animate_slime},
    combat::{
        bullet_hit_enemy, bullet_movement, bullet_tile_highlight, enemy_bullet_hit_player,
        enemy_bullet_movement, entity_flash, muzzle_lifetime,
    },
    common::update_transforms,
    enemy_ai::{enemy_movement, enemy_shoot},
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
                        resolution: (800, 600).into(),
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
        .add_systems(Startup, (setup, setup_action_bar, spawn_player_actions))
        .add_systems(
            Update,
            (
                // Player systems
                move_player,
                player_shoot,
                action_input,
                // Animation
                animate_player,
                animate_slime,
                // Enemy AI
                enemy_movement,
                enemy_shoot,
                // Combat
                bullet_movement,
                enemy_bullet_movement,
                bullet_hit_enemy,
                charged_shot_hit_enemy,
                enemy_bullet_hit_player,
                bullet_tile_highlight,
                // Effects
                entity_flash,
                heal_flash_effect,
                muzzle_lifetime,
                // UI
                update_action_bar_ui,
                // Transform updates (should run last)
                update_transforms,
            ),
        )
        .run();
}
