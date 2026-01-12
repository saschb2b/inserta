use bevy::prelude::*;

use crate::components::{
    ActionType, ArenaConfig, CleanupOnStateExit, EnemyConfig, EnemyType, FighterConfig, GameState,
};
use crate::constants::*;
use crate::resources::{GameProgress, PlayerCurrency, PlayerUpgrades};

// ============================================================================
// Shop State
// ============================================================================

#[derive(Component)]
pub struct ShopMenu;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopAction {
    UpgradeDamage,
    UpgradeHealth,
    UpgradeFireRate,
    UpgradeCritChance,
    NextBattle,
}

#[derive(Component)]
pub struct ShopItem {
    pub index: usize,
    pub action: ShopAction,
}

#[derive(Component)]
pub struct ShopText {
    pub action: ShopAction,
}

#[derive(Resource)]
pub struct ShopSelection(pub usize);

#[derive(Resource)]
pub struct ShopItemCount(pub usize);

// ============================================================================
// Setup
// ============================================================================

pub fn setup_shop(
    mut commands: Commands,
    currency: Res<PlayerCurrency>,
    progress: Res<GameProgress>,
) {
    // Background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.05, 0.05, 0.15),
            custom_size: Some(Vec2::new(SCREEN_WIDTH + 200.0, SCREEN_HEIGHT + 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        ShopMenu,
        CleanupOnStateExit(GameState::Shop),
    ));

    // Title
    commands.spawn((
        Text2d::new("DATA SHOP"),
        TextFont::from_font_size(60.0),
        TextColor(Color::srgb(0.4, 0.9, 0.6)),
        Transform::from_xyz(0.0, 300.0, 1.0),
        ShopMenu,
        CleanupOnStateExit(GameState::Shop),
    ));

    // Currency Display
    commands.spawn((
        Text2d::new(format!("ZENNY: {}", currency.zenny)),
        TextFont::from_font_size(40.0),
        TextColor(Color::srgb(1.0, 0.9, 0.2)),
        Transform::from_xyz(0.0, 240.0, 1.0),
        ShopMenu,
        CleanupOnStateExit(GameState::Shop),
    ));

    // Wave Info
    commands.spawn((
        Text2d::new(format!("Next Wave: {}", progress.current_level + 1)),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Transform::from_xyz(0.0, 200.0, 1.0),
        ShopMenu,
        CleanupOnStateExit(GameState::Shop),
    ));

    let menu_items = vec![
        ShopAction::UpgradeDamage,
        ShopAction::UpgradeHealth,
        ShopAction::UpgradeFireRate,
        ShopAction::UpgradeCritChance,
        ShopAction::NextBattle,
    ];

    let item_count = menu_items.len();
    let start_y = 100.0;
    let item_spacing = 80.0;

    for (i, action) in menu_items.into_iter().enumerate() {
        let y = start_y - (i as f32 * item_spacing);
        let is_next_battle = action == ShopAction::NextBattle;

        // Background Highlight
        commands.spawn((
            Sprite {
                color: Color::srgba(0.3, 0.5, 0.8, 0.0),
                custom_size: Some(Vec2::new(600.0, 60.0)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.5),
            ShopMenu,
            ShopItem { index: i, action },
            CleanupOnStateExit(GameState::Shop),
        ));

        // Text
        let label = if is_next_battle {
            "INITIATE BATTLE".to_string()
        } else {
            "Upgrade ...".to_string() // Placeholder, updated in update_visuals
        };

        let color = if is_next_battle {
            Color::srgb(0.9, 0.5, 0.5)
        } else {
            Color::srgb(0.8, 0.8, 0.8)
        };

        commands.spawn((
            Text2d::new(label),
            TextFont::from_font_size(32.0),
            TextColor(color),
            Transform::from_xyz(0.0, y, 1.0),
            ShopMenu,
            ShopText { action },
            CleanupOnStateExit(GameState::Shop),
        ));
    }

    // Instructions
    commands.spawn((
        Text2d::new("UP/DOWN to select  |  ENTER/SPACE to buy/start"),
        TextFont::from_font_size(18.0),
        TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
        Transform::from_xyz(0.0, -300.0, 1.0),
        ShopMenu,
        CleanupOnStateExit(GameState::Shop),
    ));

    commands.insert_resource(ShopSelection(0));
    commands.insert_resource(ShopItemCount(item_count));
}

// ============================================================================
// Update
// ============================================================================

pub fn update_shop_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<ShopSelection>,
    item_count: Res<ShopItemCount>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        if selection.0 > 0 {
            selection.0 -= 1;
        } else {
            selection.0 = item_count.0.saturating_sub(1);
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        selection.0 = (selection.0 + 1) % item_count.0;
    }
}

pub fn handle_shop_selection(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    selection: Res<ShopSelection>,
    menu_items: Query<&ShopItem>,
    mut currency: ResMut<PlayerCurrency>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut next_state: ResMut<NextState<GameState>>,
    progress: Res<GameProgress>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        for item in &menu_items {
            if item.index == selection.0 {
                match item.action {
                    ShopAction::UpgradeDamage => {
                        let cost = upgrades.cost_damage();
                        if currency.zenny >= cost {
                            currency.zenny -= cost;
                            upgrades.damage_level += 1;
                        }
                    }
                    ShopAction::UpgradeHealth => {
                        let cost = upgrades.cost_health();
                        if currency.zenny >= cost {
                            currency.zenny -= cost;
                            upgrades.health_level += 1;
                        }
                    }
                    ShopAction::UpgradeFireRate => {
                        let cost = upgrades.cost_fire_rate();
                        if currency.zenny >= cost {
                            currency.zenny -= cost;
                            upgrades.fire_rate_level += 1;
                        }
                    }
                    ShopAction::UpgradeCritChance => {
                        let cost = upgrades.cost_crit_chance();
                        if currency.zenny >= cost {
                            currency.zenny -= cost;
                            upgrades.crit_chance_level += 1;
                        }
                    }
                    ShopAction::NextBattle => {
                        start_battle(&mut commands, &progress);
                        next_state.set(GameState::Playing);
                    }
                }
                break;
            }
        }
    }
}

pub fn update_shop_visuals(
    selection: Res<ShopSelection>,
    mut item_query: Query<(&ShopItem, &mut Sprite)>,
    mut text_query: Query<(&ShopText, &mut Text2d, &mut TextColor)>,
    upgrades: Res<PlayerUpgrades>,
    currency: Res<PlayerCurrency>,
    time: Res<Time>,
) {
    let t = time.elapsed_secs();
    let pulse = 0.2 + 0.1 * (t * 10.0).sin();

    // Update Highlight
    for (item, mut sprite) in &mut item_query {
        if item.index == selection.0 {
            sprite.color = Color::srgba(0.3, 0.5, 0.8, pulse);
        } else {
            sprite.color = Color::srgba(0.3, 0.5, 0.8, 0.0);
        }
    }

    // Update Text Content and Color (Affordability)
    for (shop_text, mut text, mut color) in &mut text_query {
        let (label, _cost, can_afford) = match shop_text.action {
            ShopAction::UpgradeDamage => {
                let cost = upgrades.cost_damage();
                (
                    format!("Damage Lv.{} ({} Z)", upgrades.damage_level, cost),
                    cost,
                    currency.zenny >= cost,
                )
            }
            ShopAction::UpgradeHealth => {
                let cost = upgrades.cost_health();
                (
                    format!("Max HP Lv.{} ({} Z)", upgrades.health_level, cost),
                    cost,
                    currency.zenny >= cost,
                )
            }
            ShopAction::UpgradeFireRate => {
                let cost = upgrades.cost_fire_rate();
                (
                    format!("Fire Rate Lv.{} ({} Z)", upgrades.fire_rate_level, cost),
                    cost,
                    currency.zenny >= cost,
                )
            }
            ShopAction::UpgradeCritChance => {
                let cost = upgrades.cost_crit_chance();
                (
                    format!("Crit Chance Lv.{} ({} Z)", upgrades.crit_chance_level, cost),
                    cost,
                    currency.zenny >= cost,
                )
            }
            ShopAction::NextBattle => ("INITIATE BATTLE".to_string(), 0, true),
        };

        text.0 = label;

        if shop_text.action != ShopAction::NextBattle {
            if can_afford {
                color.0 = Color::WHITE;
            } else {
                color.0 = Color::srgb(0.5, 0.5, 0.5); // Greyed out
            }
        }
    }
}

pub fn cleanup_shop(mut commands: Commands, query: Query<(Entity, &CleanupOnStateExit)>) {
    // Remove resources
    commands.remove_resource::<ShopSelection>();
    commands.remove_resource::<ShopItemCount>();

    // Despawn shop entities
    for (entity, scoped) in &query {
        if scoped.0 == GameState::Shop {
            commands.entity(entity).despawn();
        }
    }
}
fn start_battle(commands: &mut Commands, progress: &GameProgress) {
    // Scale enemy HP based on level
    let base_hp = 100;
    let hp_per_level = 50;
    let enemy_hp = base_hp + (progress.current_level as i32 * hp_per_level);

    let config = ArenaConfig {
        fighter: FighterConfig {
            start_x: 1,
            start_y: 1,
            max_hp: 100, // This is overridden by PlayerUpgrades in setup_arena
            actions: vec![ActionType::Heal, ActionType::Shield, ActionType::WideSword],
        },
        enemies: vec![EnemyConfig {
            enemy_type: EnemyType::Slime,
            start_x: 4,
            start_y: 1,
            max_hp: enemy_hp,
        }],
    };
    commands.insert_resource(config);
}
