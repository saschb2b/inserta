use bevy::prelude::*;

use crate::components::{
    ActionType, ArenaConfig, CleanupOnStateExit, EnemyConfig, EnemyType, FighterConfig, GameState,
};
use crate::constants::*;

/// Marker for the main menu container
#[derive(Component)]
pub struct MainMenu;

/// Marker for menu items that can be selected
#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
    pub action: MenuAction,
}

/// Available menu actions
#[derive(Clone, Debug)]
pub enum MenuAction {
    StartTestBattle,
    // Future: StartCampaign, Options, Quit, etc.
}

/// Currently selected menu index
#[derive(Resource)]
pub struct MenuSelection(pub usize);

/// Total number of menu items
#[derive(Resource)]
pub struct MenuItemCount(pub usize);

/// Setup the main menu
pub fn setup_menu(mut commands: Commands) {
    // Menu background - cyber grid style
    commands.spawn((
        Sprite {
            color: Color::srgb(0.03, 0.03, 0.1),
            custom_size: Some(Vec2::new(SCREEN_WIDTH + 200.0, SCREEN_HEIGHT + 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        MainMenu,
        CleanupOnStateExit(GameState::MainMenu),
    ));

    // Decorative grid lines
    for i in 0..20 {
        let y = (i as f32 - 10.0) * 50.0;
        commands.spawn((
            Sprite {
                color: Color::srgba(0.15, 0.25, 0.5, 0.15),
                custom_size: Some(Vec2::new(SCREEN_WIDTH * 1.5, 1.5)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.1),
            MainMenu,
            CleanupOnStateExit(GameState::MainMenu),
        ));
    }
    for i in 0..30 {
        let x = (i as f32 - 15.0) * 50.0;
        commands.spawn((
            Sprite {
                color: Color::srgba(0.15, 0.25, 0.5, 0.1),
                custom_size: Some(Vec2::new(1.5, SCREEN_HEIGHT * 1.5)),
                ..default()
            },
            Transform::from_xyz(x, 0.0, 0.1),
            MainMenu,
            CleanupOnStateExit(GameState::MainMenu),
        ));
    }

    // Title
    commands.spawn((
        Text2d::new("INSERTA"),
        TextFont::from_font_size(80.0),
        TextColor(Color::srgb(0.9, 0.4, 0.3)),
        Transform::from_xyz(0.0, 250.0, 1.0),
        MainMenu,
        CleanupOnStateExit(GameState::MainMenu),
    ));

    // Subtitle
    commands.spawn((
        Text2d::new("Main Hub"),
        TextFont::from_font_size(28.0),
        TextColor(Color::srgb(0.5, 0.7, 0.9)),
        Transform::from_xyz(0.0, 190.0, 1.0),
        MainMenu,
        CleanupOnStateExit(GameState::MainMenu),
    ));

    // Menu items
    let menu_items = vec![("Start Test Battle", MenuAction::StartTestBattle)];

    let item_count = menu_items.len();
    let start_y = 50.0;
    let item_spacing = 60.0;

    for (i, (label, action)) in menu_items.into_iter().enumerate() {
        let y = start_y - (i as f32 * item_spacing);

        // Menu item background (selection indicator)
        commands.spawn((
            Sprite {
                color: Color::srgba(0.3, 0.5, 0.8, 0.0), // Initially invisible
                custom_size: Some(Vec2::new(400.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(0.0, y, 0.5),
            MainMenu,
            MenuItem { index: i, action },
            CleanupOnStateExit(GameState::MainMenu),
        ));

        // Menu item text
        commands.spawn((
            Text2d::new(label),
            TextFont::from_font_size(32.0),
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Transform::from_xyz(0.0, y, 1.0),
            MainMenu,
            CleanupOnStateExit(GameState::MainMenu),
        ));
    }

    // Instructions at bottom
    commands.spawn((
        Text2d::new("UP/DOWN to select  |  ENTER/SPACE to confirm"),
        TextFont::from_font_size(18.0),
        TextColor(Color::srgba(0.6, 0.6, 0.6, 0.8)),
        Transform::from_xyz(0.0, -300.0, 1.0),
        MainMenu,
        CleanupOnStateExit(GameState::MainMenu),
    ));

    // Initialize selection state
    commands.insert_resource(MenuSelection(0));
    commands.insert_resource(MenuItemCount(item_count));
}

/// Handle menu navigation input
pub fn update_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut selection: ResMut<MenuSelection>,
    item_count: Res<MenuItemCount>,
) {
    let mut delta = 0;

    // Keyboard
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        delta -= 1;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        delta += 1;
    }

    // Gamepad
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            delta -= 1;
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            delta += 1;
        }
    }

    if delta != 0 {
        let new_selection = (selection.0 as i32 + delta).rem_euclid(item_count.0 as i32);
        selection.0 = new_selection as usize;
    }
}

/// Handle menu selection/confirmation
pub fn handle_menu_selection(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    selection: Res<MenuSelection>,
    menu_items: Query<&MenuItem>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut confirm_pressed = false;

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        confirm_pressed = true;
    }

    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::South) || gamepad.just_pressed(GamepadButton::Start)
        {
            confirm_pressed = true;
        }
    }

    if confirm_pressed {
        // Find the selected menu item
        for item in &menu_items {
            if item.index == selection.0 {
                match &item.action {
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
                break;
            }
        }
    }
}

/// Update visual state of menu items (highlight selected)
pub fn update_menu_visuals(
    selection: Res<MenuSelection>,
    mut menu_items: Query<(&MenuItem, &mut Sprite)>,
    time: Res<Time>,
) {
    let t = time.elapsed_secs();
    let pulse = 0.2 + 0.1 * (t * 3.0).sin();

    for (item, mut sprite) in &mut menu_items {
        if item.index == selection.0 {
            // Selected - show highlight with pulse
            sprite.color = Color::srgba(0.3, 0.5, 0.8, pulse);
        } else {
            // Not selected - invisible
            sprite.color = Color::srgba(0.3, 0.5, 0.8, 0.0);
        }
    }
}

/// Cleanup menu resources
pub fn cleanup_menu(mut commands: Commands) {
    commands.remove_resource::<MenuSelection>();
    commands.remove_resource::<MenuItemCount>();
}
