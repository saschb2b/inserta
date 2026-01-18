use bevy::prelude::*;

use crate::components::{ArenaConfig, CleanupOnStateExit, FighterConfig, GameState};
use crate::resources::{CampaignProgress, PlayerLoadout, SelectedBattle, get_all_arcs};

// ============================================================================
// Campaign UI Components
// ============================================================================

/// Marker for the campaign screen root
#[derive(Component)]
pub struct CampaignScreen;

/// Marker for battle selection cursor
#[derive(Component)]
pub struct BattleCursor {
    pub battle_index: usize,
}

/// Marker for a battle square in the UI
#[derive(Component)]
pub struct BattleSquare {
    pub arc_index: usize,
    pub battle_index: usize,
}

/// Marker for the battle info panel
#[derive(Component)]
pub struct BattleInfoPanel;

/// Marker for the battle name text
#[derive(Component)]
pub struct BattleNameText;

/// Marker for the battle description text
#[derive(Component)]
pub struct BattleDescText;

/// Resource for cursor navigation state
#[derive(Resource, Default)]
pub struct CampaignCursor {
    pub arc_index: usize,
    pub battle_index: usize,
}

// ============================================================================
// Colors
// ============================================================================

const SQUARE_LOCKED: Color = Color::srgba(0.2, 0.2, 0.2, 0.5);
const SQUARE_AVAILABLE: Color = Color::srgb(0.3, 0.5, 0.8);
const SQUARE_COMPLETED: Color = Color::srgb(0.2, 0.7, 0.3);
const SQUARE_BOSS: Color = Color::srgb(0.8, 0.3, 0.3);
const SQUARE_BOSS_COMPLETED: Color = Color::srgb(0.5, 0.7, 0.3);
const SQUARE_SELECTED: Color = Color::srgb(1.0, 0.9, 0.3);

// ============================================================================
// Setup System
// ============================================================================

pub fn setup_campaign(mut commands: Commands, campaign_progress: Res<CampaignProgress>) {
    // Initialize cursor resource
    commands.insert_resource(CampaignCursor::default());

    let arcs = get_all_arcs();
    let current_arc = &arcs[0]; // Start with Arc 1

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.03, 0.03, 0.1)),
            CampaignScreen,
            CleanupOnStateExit(GameState::Campaign),
        ))
        .with_children(|parent| {
            // Title: Arc Name
            parent.spawn((
                Text::new(current_arc.name),
                TextFont::from_font_size(50.0),
                TextColor(Color::srgb(0.9, 0.7, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Arc Description
            parent.spawn((
                Text::new(current_arc.description),
                TextFont::from_font_size(20.0),
                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Battle Grid Container (horizontal row of 10 squares)
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(15.0),
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },))
                .with_children(|grid_parent| {
                    for (battle_idx, battle) in current_arc.battles.iter().enumerate() {
                        let is_completed = campaign_progress.is_battle_won(0, battle_idx);
                        let is_available = battle_idx == 0
                            || campaign_progress.is_battle_won(0, battle_idx.saturating_sub(1));

                        let base_color = if !is_available {
                            SQUARE_LOCKED
                        } else if battle.is_boss {
                            if is_completed {
                                SQUARE_BOSS_COMPLETED
                            } else {
                                SQUARE_BOSS
                            }
                        } else if is_completed {
                            SQUARE_COMPLETED
                        } else {
                            SQUARE_AVAILABLE
                        };

                        // Battle Square
                        grid_parent
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(70.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(3.0)),
                                    ..default()
                                },
                                BorderColor::all(if battle_idx == 0 {
                                    SQUARE_SELECTED
                                } else {
                                    Color::srgba(0.5, 0.5, 0.5, 0.5)
                                }),
                                BackgroundColor(base_color),
                                BattleSquare {
                                    arc_index: 0,
                                    battle_index: battle_idx,
                                },
                            ))
                            .with_children(|square_parent| {
                                // Battle number or BOSS label
                                let label = if battle.is_boss {
                                    "B".to_string()
                                } else {
                                    (battle_idx + 1).to_string()
                                };

                                square_parent.spawn((
                                    Text::new(label),
                                    TextFont::from_font_size(24.0),
                                    TextColor(if is_available {
                                        Color::WHITE
                                    } else {
                                        Color::srgba(0.5, 0.5, 0.5, 0.6)
                                    }),
                                ));

                                // Checkmark for completed battles
                                if is_completed {
                                    square_parent.spawn((
                                        Text::new("*"),
                                        TextFont::from_font_size(16.0),
                                        TextColor(Color::srgb(1.0, 1.0, 0.3)),
                                        Node {
                                            position_type: PositionType::Absolute,
                                            top: Val::Px(2.0),
                                            right: Val::Px(5.0),
                                            ..default()
                                        },
                                    ));
                                }
                            });

                        // Connection line (except after last square)
                        if battle_idx < 9 {
                            grid_parent.spawn((
                                Node {
                                    width: Val::Px(10.0),
                                    height: Val::Px(4.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.4)),
                            ));
                        }
                    }
                });

            // Battle Info Panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        min_height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.9)),
                    BorderColor::all(Color::srgba(0.4, 0.4, 0.6, 0.8)),
                    BattleInfoPanel,
                ))
                .with_children(|panel| {
                    // Battle Name
                    panel.spawn((
                        Text::new(current_arc.battles[0].name),
                        TextFont::from_font_size(28.0),
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                        BattleNameText,
                    ));

                    // Battle Description (enemy composition)
                    panel.spawn((
                        Text::new(current_arc.battles[0].description),
                        TextFont::from_font_size(20.0),
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 0.9)),
                        BattleDescText,
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new(
                    "Arrow Keys / D-Pad: Select Battle  |  Enter / A: Start Battle  |  Esc: Back",
                ),
                TextFont::from_font_size(18.0),
                TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}

// ============================================================================
// Update System
// ============================================================================

pub fn update_campaign(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cursor: ResMut<CampaignCursor>,
    campaign_progress: Res<CampaignProgress>,
    player_loadout: Res<PlayerLoadout>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut battle_squares: Query<(
        &BattleSquare,
        &Interaction,
        &mut BorderColor,
        &mut BackgroundColor,
    )>,
    mut name_text: Query<&mut Text, (With<BattleNameText>, Without<BattleDescText>)>,
    mut desc_text: Query<&mut Text, (With<BattleDescText>, Without<BattleNameText>)>,
) {
    let arcs = get_all_arcs();
    let current_arc = &arcs[cursor.arc_index];
    let old_battle = cursor.battle_index;

    // Handle mouse click/hover on battle squares
    let mut clicked_battle: Option<usize> = None;
    for (square, interaction, _, _) in battle_squares.iter() {
        // Check if this battle is available
        let is_available = square.battle_index == 0
            || campaign_progress
                .is_battle_won(cursor.arc_index, square.battle_index.saturating_sub(1));

        if is_available {
            match *interaction {
                Interaction::Pressed => {
                    // Start battle on click
                    clicked_battle = Some(square.battle_index);
                }
                Interaction::Hovered => {
                    // Update cursor on hover
                    if cursor.battle_index != square.battle_index {
                        cursor.battle_index = square.battle_index;
                    }
                }
                Interaction::None => {}
            }
        }
    }

    // Handle left/right navigation
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        if cursor.battle_index > 0 {
            // Check if previous battle is available (either first or previous completed)
            let target = cursor.battle_index - 1;
            if target == 0
                || campaign_progress.is_battle_won(cursor.arc_index, target.saturating_sub(1))
            {
                cursor.battle_index = target;
            }
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        if cursor.battle_index < 9 {
            // Check if next battle is available (current must be completed OR it's battle 0)
            let target = cursor.battle_index + 1;
            if target == 0 || campaign_progress.is_battle_won(cursor.arc_index, cursor.battle_index)
            {
                cursor.battle_index = target;
            }
        }
    }

    // Update visuals if cursor moved
    if cursor.battle_index != old_battle {
        // Update battle info text
        let battle = &current_arc.battles[cursor.battle_index];

        for mut text in name_text.iter_mut() {
            **text = battle.name.to_string();
        }
        for mut text in desc_text.iter_mut() {
            **text = battle.description.to_string();
        }
    }

    // Always update square visuals (for hover effects and selection)
    for (square, _, mut border, mut bg) in battle_squares.iter_mut() {
        if square.arc_index == cursor.arc_index {
            let is_selected = square.battle_index == cursor.battle_index;
            let is_completed =
                campaign_progress.is_battle_won(square.arc_index, square.battle_index);
            let is_available = square.battle_index == 0
                || campaign_progress
                    .is_battle_won(square.arc_index, square.battle_index.saturating_sub(1));
            let is_boss = current_arc.battles[square.battle_index].is_boss;

            // Update border
            *border = BorderColor::all(if is_selected {
                SQUARE_SELECTED
            } else {
                Color::srgba(0.5, 0.5, 0.5, 0.5)
            });

            // Update background
            bg.0 = if !is_available {
                SQUARE_LOCKED
            } else if is_boss {
                if is_completed {
                    SQUARE_BOSS_COMPLETED
                } else {
                    SQUARE_BOSS
                }
            } else if is_completed {
                SQUARE_COMPLETED
            } else {
                SQUARE_AVAILABLE
            };
        }
    }

    // Handle battle start via keyboard
    let keyboard_start =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    // Start battle if clicked or keyboard pressed
    if clicked_battle.is_some() || keyboard_start {
        let battle_to_start = clicked_battle.unwrap_or(cursor.battle_index);

        // Check if battle is available
        let is_available = battle_to_start == 0
            || campaign_progress.is_battle_won(cursor.arc_index, battle_to_start.saturating_sub(1));

        if is_available {
            let battle = &current_arc.battles[battle_to_start];

            // Store selected battle for return after victory
            commands.insert_resource(SelectedBattle {
                arc: cursor.arc_index,
                battle: battle_to_start,
            });

            // Create arena config from battle definition using player's loadout
            let config = ArenaConfig {
                fighter: FighterConfig {
                    start_x: 1,
                    start_y: 1,
                    max_hp: 100,
                    actions: player_loadout.equipped_actions(),
                },
                enemies: battle.enemies.clone(),
            };
            commands.insert_resource(config);

            next_state.set(GameState::Playing);
        }
    }

    // Handle back to menu
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

// ============================================================================
// Cleanup System
// ============================================================================

pub fn cleanup_campaign(mut commands: Commands) {
    commands.remove_resource::<CampaignCursor>();
}
