#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_range_contains)]

use bevy::prelude::*;

mod actions;
mod assets;
mod components;
mod constants;
mod enemies;
mod resources;
mod systems;
mod weapons;

use actions::ActionsPlugin;
use components::{GameState, InputCooldown};
use constants::MOVE_COOLDOWN;
use enemies::EnemyPlugin;
use resources::{
    BattleTimer, CampaignProgress, GameProgress, PlayerCurrency, PlayerUpgrades, SelectedBattle,
    WaveState,
};
use systems::{
    action_ui::update_action_bar_ui,
    animation::{animate_player, animate_slime},
    campaign::{cleanup_campaign, setup_campaign, update_campaign},
    combat::{
        bullet_movement, check_defeat_condition, check_victory_condition, enemy_bullet_hit_player,
        enemy_bullet_movement, entity_flash, muzzle_lifetime, projectile_animation_system,
        tile_attack_highlight, update_wave_state,
    },
    common::update_transforms,
    growth::{GrowthTreeState, cleanup_growth, setup_growth_tree, update_growth_tree},
    intro::{cleanup_intro, intro_complete, setup_intro, update_intro},
    menu::{cleanup_menu, handle_menu_selection, setup_menu, update_menu_visuals},
    outro::{
        check_defeat_outro_complete, check_outro_complete, cleanup_outro, defeat_outro_active,
        outro_not_active, setup_defeat_outro, setup_outro, update_defeat_outro, update_outro,
        victory_outro_active,
    },
    player::move_player,
    setup::{
        cleanup_arena, cleanup_campaign_entities, cleanup_menu_entities, cleanup_splash_entities,
        setup_action_bar, setup_arena, setup_global, spawn_player_actions,
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
        .init_resource::<PlayerCurrency>()
        .init_resource::<GameProgress>()
        .init_resource::<PlayerUpgrades>()
        .init_resource::<WaveState>()
        .init_resource::<BattleTimer>()
        .init_resource::<GrowthTreeState>()
        .init_resource::<CampaignProgress>()
        .init_resource::<SelectedBattle>()
        // Weapon system plugin
        .add_plugins(WeaponPlugin)
        // Action/chip system plugin
        .add_plugins(ActionsPlugin)
        // Enemy behavior system plugin
        .add_plugins(EnemyPlugin)
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
            (handle_menu_selection, update_menu_visuals).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (cleanup_menu, cleanup_menu_entities),
        )
        // ====================================================================
        // Campaign
        // ====================================================================
        .add_systems(OnEnter(GameState::Campaign), setup_campaign)
        .add_systems(
            Update,
            update_campaign.run_if(in_state(GameState::Campaign)),
        )
        .add_systems(
            OnExit(GameState::Campaign),
            (cleanup_campaign, cleanup_campaign_entities),
        )
        // ====================================================================
        // Shop / Growth Tree
        // ====================================================================
        .add_systems(OnEnter(GameState::Shop), setup_growth_tree)
        .add_systems(Update, update_growth_tree.run_if(in_state(GameState::Shop)))
        .add_systems(OnExit(GameState::Shop), cleanup_growth)
        // ====================================================================
        // Playing (Arena)
        // ====================================================================
        .add_systems(
            OnEnter(GameState::Playing),
            (
                setup_arena,
                setup_action_bar,
                spawn_player_actions,
                setup_intro,
                reset_battle_timer,
            ),
        )
        // Pre-battle intro system (runs until countdown complete)
        .add_systems(Update, update_intro.run_if(in_state(GameState::Playing)))
        // Battle timer (only runs during active gameplay, not during outro)
        .add_systems(
            Update,
            tick_battle_timer
                .run_if(in_state(GameState::Playing))
                .run_if(intro_complete)
                .run_if(outro_not_active),
        )
        // Player input systems (only run after intro complete and not during outro)
        // NOTE: Action input is now handled by ActionsPlugin
        .add_systems(
            Update,
            (
                // Player systems
                move_player,
                // Animation
                animate_player,
            )
                .run_if(in_state(GameState::Playing))
                .run_if(intro_complete)
                .run_if(outro_not_active),
        )
        // Enemy animation and effects - chained to avoid Sprite conflicts
        .add_systems(
            Update,
            (
                animate_slime,
                enemies::animate_charging_telegraph,
                entity_flash,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
                .run_if(outro_not_active),
        )
        .add_systems(
            Update,
            (
                // Projectile animations (before movement so sprites are updated)
                projectile_animation_system,
                // Combat
                bullet_movement,
                enemy_bullet_movement,
                enemy_bullet_hit_player,
                tile_attack_highlight,
                // Game Loop
                update_wave_state,
                check_victory_condition,
                check_defeat_condition,
            )
                .run_if(in_state(GameState::Playing))
                .run_if(outro_not_active),
        )
        .add_systems(
            Update,
            (
                // Other effects
                muzzle_lifetime,
                // UI
                update_action_bar_ui,
                // Transform updates (should run last)
                update_transforms,
                // Back to menu on Escape (only when not in outro)
                return_to_menu.run_if(outro_not_active),
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Victory outro systems
        .add_systems(
            Update,
            (setup_outro, update_outro, check_outro_complete)
                .chain()
                .run_if(in_state(GameState::Playing))
                .run_if(victory_outro_active),
        )
        // Defeat outro systems
        .add_systems(
            Update,
            (
                setup_defeat_outro,
                update_defeat_outro,
                check_defeat_outro_complete,
            )
                .chain()
                .run_if(in_state(GameState::Playing))
                .run_if(defeat_outro_active),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (cleanup_arena, cleanup_intro, cleanup_outro),
        )
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

/// Reset battle timer when entering Playing state
fn reset_battle_timer(mut timer: ResMut<BattleTimer>) {
    timer.reset();
}

/// Tick battle timer during active gameplay
fn tick_battle_timer(time: Res<Time>, mut timer: ResMut<BattleTimer>) {
    timer.tick(time.delta_secs());
}
