use bevy::prelude::*;
use bevy::ui::RepeatedGridTrack;

use crate::components::{CleanupOnStateExit, GameState};
use crate::resources::{PlayerCurrency, PlayerUpgrades};

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
    BackToMenu,
}

#[derive(Component)]
pub struct ShopButtonAction(pub ShopAction);

#[derive(Component)]
pub struct ShopButtonText(pub ShopAction);

// ============================================================================
// Setup
// ============================================================================

pub fn setup_shop(mut commands: Commands, currency: Res<PlayerCurrency>) {
    // Root Node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.15)),
            ShopMenu,
            CleanupOnStateExit(GameState::Shop),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("DATA SHOP"),
                TextFont::from_font_size(60.0),
                TextColor(Color::srgb(0.4, 0.9, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Currency Display
            parent.spawn((
                Text::new(format!("ZENNY: {}", currency.zenny)),
                TextFont::from_font_size(40.0),
                TextColor(Color::srgb(1.0, 0.9, 0.2)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Spacer (removed wave info since we use campaign now)
            parent.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            // Shop Items Container (Grid-like)
            parent
                .spawn(Node {
                    display: Display::Grid,
                    // 2 columns, equal width
                    grid_template_columns: vec![RepeatedGridTrack::flex(2, 1.0)],
                    // 3 rows, equal height
                    grid_template_rows: vec![RepeatedGridTrack::flex(3, 1.0)],
                    row_gap: Val::Px(20.0),
                    column_gap: Val::Px(20.0),
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|grid| {
                    let actions = [
                        ShopAction::UpgradeDamage,
                        ShopAction::UpgradeHealth,
                        ShopAction::UpgradeFireRate,
                        ShopAction::UpgradeCritChance,
                    ];

                    for action in actions {
                        grid.spawn((
                            Button,
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            BackgroundColor(Color::srgb(0.3, 0.5, 0.8)),
                            ShopButtonAction(action),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("Upgrade ..."),
                                TextFont::from_font_size(24.0),
                                TextColor(Color::WHITE),
                                ShopButtonText(action),
                            ));
                        });
                    }
                });

            // Back to Menu Button (Separate)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(40.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(Color::srgb(0.5, 0.5, 0.7)),
                    ShopButtonAction(ShopAction::BackToMenu),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("BACK TO MENU"),
                        TextFont::from_font_size(32.0),
                        TextColor(Color::WHITE),
                        ShopButtonText(ShopAction::BackToMenu),
                    ));
                });
        });
}

// ============================================================================
// Update
// ============================================================================

pub fn handle_shop_interaction(
    interaction_query: Query<
        (&Interaction, &ShopButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut currency: ResMut<PlayerCurrency>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, shop_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match shop_action.0 {
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
                ShopAction::BackToMenu => {
                    next_state.set(GameState::MainMenu);
                }
            }
        }
    }
}

pub fn update_shop_visuals(
    // Update button colors based on interaction and affordability
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ShopButtonAction,
        ),
        With<Button>,
    >,
    // Update text content and color
    mut text_query: Query<(&mut Text, &mut TextColor, &ShopButtonText)>,
    upgrades: Res<PlayerUpgrades>,
    currency: Res<PlayerCurrency>,
) {
    // Helper to check affordability
    let can_afford = |action: ShopAction| -> bool {
        match action {
            ShopAction::UpgradeDamage => currency.zenny >= upgrades.cost_damage(),
            ShopAction::UpgradeHealth => currency.zenny >= upgrades.cost_health(),
            ShopAction::UpgradeFireRate => currency.zenny >= upgrades.cost_fire_rate(),
            ShopAction::UpgradeCritChance => currency.zenny >= upgrades.cost_crit_chance(),
            ShopAction::BackToMenu => true,
        }
    };

    // Update Buttons
    for (interaction, mut bg, mut border, action) in &mut button_query {
        let affordable = can_afford(action.0);

        match interaction {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.2, 0.4, 0.7);
                *border = BorderColor::all(Color::srgb(0.8, 0.8, 0.8));
            }
            Interaction::Hovered => {
                // Dim if not affordable
                if affordable {
                    bg.0 = Color::srgb(0.4, 0.6, 0.9);
                    *border = BorderColor::all(Color::WHITE);
                } else {
                    bg.0 = Color::srgb(0.3, 0.3, 0.3); // Dim red/grey
                    *border = BorderColor::all(Color::srgb(0.5, 0.2, 0.2));
                }
            }
            Interaction::None => {
                if affordable {
                    if action.0 == ShopAction::BackToMenu {
                        bg.0 = Color::srgb(0.5, 0.5, 0.7);
                    } else {
                        bg.0 = Color::srgb(0.3, 0.5, 0.8);
                    }
                    *border = BorderColor::all(Color::NONE);
                } else {
                    bg.0 = Color::srgb(0.2, 0.2, 0.2);
                    *border = BorderColor::all(Color::NONE);
                }
            }
        }
    }

    // Update Text
    for (mut text, mut color, text_action) in &mut text_query {
        let (label, cost) = match text_action.0 {
            ShopAction::UpgradeDamage => (
                format!("Damage Lv.{}", upgrades.damage_level),
                upgrades.cost_damage(),
            ),
            ShopAction::UpgradeHealth => (
                format!("Max HP Lv.{}", upgrades.health_level),
                upgrades.cost_health(),
            ),
            ShopAction::UpgradeFireRate => (
                format!("Fire Rate Lv.{}", upgrades.fire_rate_level),
                upgrades.cost_fire_rate(),
            ),
            ShopAction::UpgradeCritChance => (
                format!("Crit Chance Lv.{}", upgrades.crit_chance_level),
                upgrades.cost_crit_chance(),
            ),
            ShopAction::BackToMenu => ("BACK TO MENU".to_string(), 0),
        };

        if text_action.0 == ShopAction::BackToMenu {
            text.0 = label;
            color.0 = Color::WHITE;
        } else {
            text.0 = format!("{} ({} Z)", label, cost);
            if currency.zenny >= cost {
                color.0 = Color::WHITE;
            } else {
                color.0 = Color::srgb(0.5, 0.5, 0.5);
            }
        }
    }
}

pub fn cleanup_shop(mut commands: Commands, query: Query<Entity, With<ShopMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
