use bevy::prelude::*;

mod assets;
mod components;
mod constants;
mod systems;
mod weapons;

use components::{GameState, InputCooldown};
use constants::MOVE_COOLDOWN;
use systems::{
    action_ui::update_action_bar_ui,
    actions::{
        action_input, charged_shot_hit_enemy, despawn_widesword_slash, heal_flash_effect,
        shield_blocks_damage, update_shield, widesword_hit_enemy,
    },
    animation::{animate_player, animate_slime},
    combat::{
        bullet_hit_enemy, bullet_movement, bullet_tile_highlight, enemy_bullet_hit_player,
        enemy_bullet_movement, entity_flash, muzzle_lifetime,
    },
    common::update_transforms,
    enemy_ai::{enemy_movement, enemy_shoot},
    menu::{
        cleanup_menu, handle_menu_selection, setup_menu, update_menu_input, update_menu_visuals,
    },
    player::move_player,
    setup::{
        cleanup_arena, cleanup_menu_entities, cleanup_splash_entities, setup_action_bar,
        setup_arena, setup_global, spawn_player_actions,
    },
    splash::{animate_splash, cleanup_splash, setup_splash, update_splash},
};
use weapons::WeaponPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "INSERTA - Battle Network".into(),
                        resolution: (1280, 800).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Global resources
        .insert_resource(InputCooldown(Timer::from_seconds(
            MOVE_COOLDOWN,
            TimerMode::Once,
        )))
        // Weapon system plugin
        .add_plugins(WeaponPlugin)
        // State management
        .init_state::<GameState>()
        // ====================================================================
        // Global startup (runs once)
        // ====================================================================
        .add_systems(Startup, setup_global)
        // ====================================================================
        // Splash Screen
        // ====================================================================
        .add_systems(OnEnter(GameState::Splash), setup_splash)
        .add_systems(
            Update,
            (update_splash, animate_splash).run_if(in_state(GameState::Splash)),
        )
        .add_systems(
            OnExit(GameState::Splash),
            (cleanup_splash, cleanup_splash_entities),
        )
        // ====================================================================
        // Main Menu
        // ====================================================================
        .add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(
            Update,
            (
                update_menu_input,
                handle_menu_selection,
                update_menu_visuals,
            )
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (cleanup_menu, cleanup_menu_entities),
        )
        // ====================================================================
        // Playing (Arena)
        // ====================================================================
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_arena, setup_action_bar, spawn_player_actions),
        )
        .add_systems(
            Update,
            (
                // Player systems
                move_player,
                action_input,
                // Animation
                animate_player,
                animate_slime,
                // Enemy AI
                enemy_movement,
                enemy_shoot,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                // Combat
                bullet_movement,
                enemy_bullet_movement,
                bullet_hit_enemy,
                charged_shot_hit_enemy,
                enemy_bullet_hit_player,
                bullet_tile_highlight,
                // Shield systems (run before damage)
                update_shield,
                shield_blocks_damage,
                // WideSword systems
                widesword_hit_enemy,
                despawn_widesword_slash,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                // Effects
                entity_flash,
                heal_flash_effect,
                muzzle_lifetime,
                // UI
                update_action_bar_ui,
                // Transform updates (should run last)
                update_transforms,
                // Back to menu on Escape
                return_to_menu,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_arena)
        .run();
}

/// System to return to main menu when pressing Escape
fn return_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
