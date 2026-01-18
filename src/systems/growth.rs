use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::{CleanupOnStateExit, GameState};
use crate::resources::{PlayerCurrency, PlayerUpgrades};
use crate::systems::shop::{ShopAction, ShopButtonAction}; // Import from shop for reuse

// ============================================================================
// Growth Tree Data
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeType {
    Damage,
    Health,
    FireRate,
    CritChance,
    Core, // Starting point
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GrowthNodeData {
    pub id: u32,
    pub upgrade_type: UpgradeType,
    pub cost: u64,
    pub parent_id: Option<u32>,
    pub x: f32, // Relative to center
    pub y: f32,
    pub label: &'static str,
    pub description: &'static str,
}

// Hardcoded Tree Layout
pub const GROWTH_NODES: &[GrowthNodeData] = &[
    // Center
    GrowthNodeData {
        id: 0,
        upgrade_type: UpgradeType::Core,
        cost: 0,
        parent_id: None,
        x: 0.0,
        y: 0.0,
        label: "CORE",
        description: "The source of your power.",
    },
    // Tier 1 - Cardinal Directions
    GrowthNodeData {
        id: 1,
        upgrade_type: UpgradeType::Damage,
        cost: 100,
        parent_id: Some(0),
        x: 0.0,
        y: -120.0, // Up
        label: "ATK +1",
        description: "Increases weapon damage by 1.",
    },
    GrowthNodeData {
        id: 2,
        upgrade_type: UpgradeType::Health,
        cost: 50,
        parent_id: Some(0),
        x: 0.0,
        y: 120.0, // Down
        label: "HP +20",
        description: "Increases max health by 20.",
    },
    GrowthNodeData {
        id: 3,
        upgrade_type: UpgradeType::FireRate,
        cost: 150,
        parent_id: Some(0),
        x: -120.0, // Left
        y: 0.0,
        label: "SPD +5%",
        description: "Reduces weapon cooldown by 5%.",
    },
    GrowthNodeData {
        id: 4,
        upgrade_type: UpgradeType::CritChance,
        cost: 200,
        parent_id: Some(0),
        x: 120.0, // Right
        y: 0.0,
        label: "CRT +2%",
        description: "Increases critical hit chance by 2%.",
    },
    // Tier 2 - Diagonals / Extensions
    GrowthNodeData {
        id: 5,
        upgrade_type: UpgradeType::Damage,
        cost: 250,
        parent_id: Some(1),
        x: 0.0,
        y: -240.0, // Up-Up
        label: "ATK +1",
        description: "Further increases weapon damage.",
    },
    GrowthNodeData {
        id: 6,
        upgrade_type: UpgradeType::Health,
        cost: 150,
        parent_id: Some(2),
        x: 0.0,
        y: 240.0, // Down-Down
        label: "HP +20",
        description: "Further increases max health.",
    },
    GrowthNodeData {
        id: 7,
        upgrade_type: UpgradeType::FireRate,
        cost: 300,
        parent_id: Some(3),
        x: -240.0, // Left-Left
        y: 0.0,
        label: "SPD +5%",
        description: "Further reduces weapon cooldown.",
    },
    GrowthNodeData {
        id: 8,
        upgrade_type: UpgradeType::CritChance,
        cost: 400,
        parent_id: Some(4),
        x: 240.0, // Right-Right
        y: 0.0,
        label: "CRT +2%",
        description: "Further increases critical chance.",
    },
];

// ============================================================================
// Resources & Components
// ============================================================================

#[derive(Resource, Default)]
pub struct GrowthTreeState {
    pub unlocked_nodes: HashSet<u32>,
}

#[derive(Component)]
pub struct GrowthMenu;

#[derive(Component)]
pub struct InfoPanelTitle;

#[derive(Component)]
pub struct InfoPanelDesc;

#[derive(Component)]
pub struct InfoPanelCost;

// ============================================================================
// Systems
// ============================================================================

pub fn setup_growth_tree(
    mut commands: Commands,
    mut tree_state: ResMut<GrowthTreeState>,
    currency: Res<PlayerCurrency>,
) {
    // Ensure core is unlocked
    tree_state.unlocked_nodes.insert(0);

    // Root Container (Row)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.08)), // Dark background
            GrowthMenu,
            CleanupOnStateExit(GameState::Shop), // Reusing Shop state for now
        ))
        .with_children(|parent| {
            // Left: Tree Area
            parent
                .spawn(Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    // Use relative positioning for children so we can place them
                    ..default()
                })
                .with_children(|tree_area| {
                    // Spawn Nodes
                    for node in GROWTH_NODES {
                        // Calculate position centered in the 70% area
                        // We use left: 50% and margins to offset
                        tree_area
                            .spawn((
                                Button,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(50.0),
                                    top: Val::Percent(50.0),
                                    // Margin moves it from the anchor point.
                                    // Subtract half width (40.0) to center properly
                                    margin: UiRect {
                                        left: Val::Px(node.x - 40.0),
                                        top: Val::Px(node.y - 40.0),
                                        ..default()
                                    },
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    border: UiRect::all(Val::Px(3.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::BLACK), // Placeholder, updated in update loop
                                BorderColor::all(Color::WHITE),
                                *node, // Component
                            ))
                            .with_children(|btn| {
                                // Icon / Label
                                btn.spawn((
                                    Text::new(match node.upgrade_type {
                                        UpgradeType::Core => "CORE",
                                        UpgradeType::Damage => "ATK",
                                        UpgradeType::Health => "HP",
                                        UpgradeType::FireRate => "SPD",
                                        UpgradeType::CritChance => "CRT",
                                    }),
                                    TextFont::from_font_size(20.0),
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                });

            // Right: Info Panel
            parent
                .spawn(Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|panel| {
                    // Header
                    panel.spawn((
                        Text::new("GROWTH CHART"),
                        TextFont::from_font_size(30.0),
                        TextColor(Color::srgb(0.5, 0.7, 0.9)),
                        Node {
                            margin: UiRect::bottom(Val::Px(40.0)),
                            ..default()
                        },
                    ));

                    // Selected Skill Name
                    panel.spawn((
                        Text::new("-"),
                        TextFont::from_font_size(40.0),
                        TextColor(Color::WHITE),
                        InfoPanelTitle,
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // Description
                    panel.spawn((
                        Text::new("Select a node."),
                        TextFont::from_font_size(20.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        InfoPanelDesc,
                        Node {
                            margin: UiRect::bottom(Val::Px(40.0)),
                            ..default()
                        },
                    ));

                    // Cost / Status
                    panel.spawn((
                        Text::new(""),
                        TextFont::from_font_size(24.0),
                        TextColor(Color::srgb(1.0, 0.9, 0.2)),
                        InfoPanelCost,
                    ));

                    // Spacer
                    panel.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });

                    // Footer / Currency
                    panel.spawn((
                        Text::new(format!("ZENNY: {}", currency.zenny)),
                        TextFont::from_font_size(30.0),
                        TextColor(Color::srgb(1.0, 0.9, 0.2)),
                    ));

                    // Back to Menu Button
                    panel
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(20.0)),
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
                                TextFont::from_font_size(24.0),
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Controller Hints
                    panel.spawn((
                        Text::new("[D-Pad] Navigate  [A] Unlock  [Esc] Back"),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                        Node {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn update_growth_tree(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut node_query: Query<
        (
            &Interaction,
            &GrowthNodeData,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (With<Button>, Without<ShopButtonAction>),
    >,
    // Reusing ShopButtonAction just for the Next Battle button for now
    mut battle_btn_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (With<Button>, With<ShopButtonAction>),
    >,

    // Note: Text component in 0.18 has public field `.0` which is the String
    mut title_query: Query<
        &mut Text,
        (
            With<InfoPanelTitle>,
            Without<InfoPanelDesc>,
            Without<InfoPanelCost>,
        ),
    >,
    mut desc_query: Query<
        &mut Text,
        (
            With<InfoPanelDesc>,
            Without<InfoPanelTitle>,
            Without<InfoPanelCost>,
        ),
    >,
    mut cost_query: Query<
        &mut Text,
        (
            With<InfoPanelCost>,
            Without<InfoPanelTitle>,
            Without<InfoPanelDesc>,
        ),
    >,

    mut currency: ResMut<PlayerCurrency>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut tree_state: ResMut<GrowthTreeState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Handle back to menu via keyboard/gamepad
    let mut back = keyboard.just_pressed(KeyCode::Escape);
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::East) {
            back = true;
        }
    }
    if back {
        next_state.set(GameState::MainMenu);
        return;
    }
    // 1. Handle Back to Menu Button
    // check for single_mut safely
    if let Some((interaction, mut bg, mut border)) = battle_btn_query.iter_mut().next() {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.6, 0.6, 0.8);
                *border = BorderColor::all(Color::WHITE);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.5, 0.5, 0.7);
                *border = BorderColor::all(Color::NONE);
            }
        }
    }

    // 2. Handle Tree Nodes
    for (interaction, data, mut bg, mut border) in &mut node_query {
        let is_unlocked = tree_state.unlocked_nodes.contains(&data.id);
        let is_parent_unlocked = data
            .parent_id
            .is_none_or(|pid| tree_state.unlocked_nodes.contains(&pid));
        let can_afford = currency.zenny >= data.cost;
        let is_purchasable = !is_unlocked && is_parent_unlocked;

        // Visuals
        if is_unlocked {
            bg.0 = Color::srgb(0.3, 0.8, 0.4); // Green (Unlocked)
            *border = BorderColor::all(Color::srgb(0.6, 1.0, 0.7));
        } else if is_purchasable {
            if can_afford {
                bg.0 = Color::srgb(0.3, 0.5, 0.8); // Blue (Available)
                *border = BorderColor::all(Color::srgb(0.5, 0.7, 1.0));
            } else {
                bg.0 = Color::srgb(0.5, 0.2, 0.2); // Red (Too expensive)
                *border = BorderColor::all(Color::srgb(0.7, 0.4, 0.4));
            }
        } else {
            bg.0 = Color::srgb(0.2, 0.2, 0.2); // Grey (Locked)
            *border = BorderColor::all(Color::srgb(0.3, 0.3, 0.3));
        }

        // Interaction (Hover/Focus updates Info Panel)
        if *interaction == Interaction::Hovered {
            // Highlight
            *border = BorderColor::all(Color::WHITE);

            // Update Info - using iter_mut().next() safely
            if let Some(mut text) = title_query.iter_mut().next() {
                text.0 = data.label.to_string();
            }
            if let Some(mut text) = desc_query.iter_mut().next() {
                text.0 = data.description.to_string();
            }

            if let Some(mut text) = cost_query.iter_mut().next() {
                if is_unlocked {
                    text.0 = "LEARNED!".to_string();
                } else if !is_parent_unlocked {
                    text.0 = "LOCKED".to_string();
                } else {
                    text.0 = format!("COST: {} Z", data.cost);
                }
            }
        }

        // Interaction (Pressed buys)
        if *interaction == Interaction::Pressed && is_purchasable && can_afford {
            // Deduct cost
            currency.zenny -= data.cost;
            // Unlock node
            tree_state.unlocked_nodes.insert(data.id);
            // Apply stats
            match data.upgrade_type {
                UpgradeType::Damage => upgrades.damage_level += 1,
                UpgradeType::Health => upgrades.health_level += 1,
                UpgradeType::FireRate => upgrades.fire_rate_level += 1,
                UpgradeType::CritChance => upgrades.crit_chance_level += 1,
                UpgradeType::Core => {}
            }
        }
    }
}

pub fn cleanup_growth(mut commands: Commands, query: Query<Entity, With<GrowthMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
