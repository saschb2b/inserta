use bevy::prelude::*;

use crate::components::{
    ActionType, ArenaConfig, CleanupOnStateExit, EnemyConfig, EnemyType, FighterConfig, GameState,
};

/// Marker for the main menu container
#[derive(Component)]
pub struct MainMenu;

/// Marker for menu button actions
#[derive(Component)]
pub struct MenuButtonAction(pub MenuAction);

/// Available menu actions
#[derive(Clone, Debug, Copy)]
pub enum MenuAction {
    StartTestBattle,
    // Future: StartCampaign, Options, Quit, etc.
}

/// Setup the main menu using Bevy UI
pub fn setup_menu(mut commands: Commands) {
    // Root Node (Full Screen)
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
            BackgroundColor(Color::srgb(0.03, 0.03, 0.1)),
            MainMenu,
            CleanupOnStateExit(GameState::MainMenu),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("INSERTA"),
                TextFont::from_font_size(80.0),
                TextColor(Color::srgb(0.9, 0.4, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Main Hub"),
                TextFont::from_font_size(30.0),
                TextColor(Color::srgb(0.5, 0.7, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));

            // Start Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.8)),
                    MenuButtonAction(MenuAction::StartTestBattle),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Test Battle"),
                        TextFont::from_font_size(30.0),
                        TextColor(Color::WHITE),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("Navigation: D-Pad / Arrow Keys | Select: A / Enter"),
                TextFont::from_font_size(18.0),
                TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(100.0)),
                    ..default()
                },
            ));
        });
}

/// Handle menu selection/confirmation via Interaction (Mouse/Touch/Gamepad Navigation)
pub fn handle_menu_selection(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action.0 {
                MenuAction::StartTestBattle => {
                    // Insert the test battle configuration
                    let config = ArenaConfig {
                        fighter: FighterConfig {
                            start_x: 1,
                            start_y: 1,
                            max_hp: 100,
                            actions: vec![
                                ActionType::Heal,
                                ActionType::Shield,
                                ActionType::WideSword,
                            ],
                        },
                        enemies: vec![EnemyConfig {
                            enemy_type: EnemyType::Slime,
                            start_x: 4,
                            start_y: 1,
                            max_hp: 100,
                        }],
                    };
                    commands.insert_resource(config);
                    next_state.set(GameState::Playing);
                }
            }
        }
    }
}

/// Update visual state of menu buttons (highlight hovered/pressed)
pub fn update_menu_visuals(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<Button>>,
) {
    for (interaction, mut bg, mut border) in &mut query {
        match interaction {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.2, 0.4, 0.7);
                *border = BorderColor::all(Color::srgb(0.8, 0.8, 0.8));
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.4, 0.6, 0.9);
                *border = BorderColor::all(Color::WHITE);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.3, 0.5, 0.8);
                *border = BorderColor::all(Color::NONE);
            }
        }
    }
}

/// Cleanup menu resources (Nothing to clean up specifically for UI node logic, cleanup_menu_entities handles root despawn)
pub fn cleanup_menu() {
    // No resources to remove in this version
}
