// ============================================================================
// Loadout Menu System - Equip actions to slots
// ============================================================================
//
// Beautiful loadout menu with:
// - 4 action slots (displayed prominently)
// - Hover/select details panel
// - Inventory browser for selecting new actions
// - Full keyboard/gamepad support

use bevy::prelude::*;

use crate::actions::{ActionBlueprint, ActionId, Element, Rarity};
use crate::components::{CleanupOnStateExit, GameState};
use crate::resources::PlayerLoadout;

// ============================================================================
// Constants - Beautiful MMBN-inspired color palette
// ============================================================================

const BG_COLOR: Color = Color::srgb(0.02, 0.02, 0.08);
const PANEL_BG: Color = Color::srgba(0.08, 0.08, 0.15, 0.95);
const PANEL_BORDER: Color = Color::srgb(0.3, 0.4, 0.6);

const SLOT_BG_EMPTY: Color = Color::srgba(0.1, 0.1, 0.15, 0.8);
const SLOT_BG_FILLED: Color = Color::srgba(0.15, 0.2, 0.35, 0.9);
const SLOT_BORDER_NORMAL: Color = Color::srgb(0.4, 0.5, 0.7);
const SLOT_BORDER_SELECTED: Color = Color::srgb(1.0, 0.85, 0.3);
const SLOT_BORDER_HOVER: Color = Color::srgb(0.7, 0.8, 1.0);

const INVENTORY_BG: Color = Color::srgba(0.05, 0.05, 0.1, 0.95);
const INVENTORY_ITEM_BG: Color = Color::srgba(0.1, 0.12, 0.2, 0.9);
const INVENTORY_ITEM_SELECTED: Color = Color::srgba(0.2, 0.25, 0.4, 0.95);
const INVENTORY_ITEM_EQUIPPED: Color = Color::srgba(0.15, 0.15, 0.2, 0.5);

const TEXT_TITLE: Color = Color::srgb(0.9, 0.85, 0.7);
const TEXT_NORMAL: Color = Color::srgb(0.85, 0.85, 0.9);
const TEXT_MUTED: Color = Color::srgb(0.5, 0.5, 0.6);
const TEXT_HIGHLIGHT: Color = Color::srgb(1.0, 0.9, 0.4);

// ============================================================================
// Components
// ============================================================================

/// Marker for the loadout menu root
#[derive(Component)]
pub struct LoadoutMenu;

/// Marker for an action slot in the UI
#[derive(Component)]
pub struct LoadoutSlot {
    pub index: usize,
}

/// Marker for the details panel
#[derive(Component)]
pub struct DetailsPanel;

/// Marker for details text elements
#[derive(Component)]
pub struct DetailsName;

#[derive(Component)]
pub struct DetailsDescription;

#[derive(Component)]
pub struct DetailsStats;

#[derive(Component)]
pub struct DetailsElement;

/// Marker for inventory panel (hidden by default)
#[derive(Component)]
pub struct InventoryPanel;

/// Marker for inventory items
#[derive(Component)]
pub struct InventoryItem {
    pub action_id: ActionId,
    pub index: usize,
}

/// Marker for the inventory list container (for scrolling)
#[derive(Component)]
pub struct InventoryList;

/// Marker for inventory details (shown inside inventory panel)
#[derive(Component)]
pub struct InventoryDetailsName;

#[derive(Component)]
pub struct InventoryDetailsDesc;

#[derive(Component)]
pub struct InventoryDetailsStats;

/// Marker for inventory item name text
#[derive(Component)]
pub struct InventoryItemText;

/// Resource tracking current selection state
#[derive(Resource, Debug, Default)]
pub struct LoadoutState {
    /// Currently selected slot (0-3) when browsing slots
    pub selected_slot: usize,
    /// Currently selected inventory item when browsing inventory
    pub inventory_cursor: usize,
    /// Whether inventory is open
    pub inventory_open: bool,
    /// Slot being edited (when inventory is open)
    pub editing_slot: Option<usize>,
    /// Debounce timer for input
    pub input_cooldown: f32,
    /// Flag to prevent same-frame input processing when opening inventory
    pub just_opened_inventory: bool,
}

impl LoadoutState {
    pub fn reset(&mut self) {
        self.selected_slot = 0;
        self.inventory_cursor = 0;
        self.inventory_open = false;
        self.editing_slot = None;
        self.input_cooldown = 0.0;
        self.just_opened_inventory = false;
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Get color for element
fn element_color(element: Element) -> Color {
    match element {
        Element::None => Color::srgb(0.7, 0.7, 0.7),
        Element::Fire => Color::srgb(1.0, 0.4, 0.2),
        Element::Aqua => Color::srgb(0.3, 0.6, 1.0),
        Element::Elec => Color::srgb(1.0, 0.9, 0.2),
        Element::Wood => Color::srgb(0.3, 0.8, 0.3),
    }
}

/// Get rarity stars string
fn rarity_stars(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "*",
        Rarity::Uncommon => "**",
        Rarity::Rare => "***",
        Rarity::SuperRare => "****",
        Rarity::UltraRare => "*****",
    }
}

/// Get color for rarity
fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::srgb(0.7, 0.7, 0.7),
        Rarity::Uncommon => Color::srgb(0.4, 0.8, 0.4),
        Rarity::Rare => Color::srgb(0.4, 0.6, 1.0),
        Rarity::SuperRare => Color::srgb(0.8, 0.4, 1.0),
        Rarity::UltraRare => Color::srgb(1.0, 0.8, 0.2),
    }
}

/// Get all available actions for inventory
fn get_all_actions() -> Vec<ActionId> {
    vec![
        // Recovery
        ActionId::Recov10,
        ActionId::Recov30,
        ActionId::Recov50,
        ActionId::Recov80,
        ActionId::Recov120,
        ActionId::Recov150,
        ActionId::Recov200,
        ActionId::Recov300,
        // Defense
        ActionId::Barrier,
        ActionId::Shield,
        ActionId::MetGuard,
        ActionId::Invis1,
        ActionId::Invis2,
        ActionId::Invis3,
        ActionId::LifeAura,
        // Swords
        ActionId::Sword,
        ActionId::WideSwrd,
        ActionId::LongSwrd,
        ActionId::FireSwrd,
        ActionId::AquaSwrd,
        ActionId::ElecSwrd,
        ActionId::FtrSwrd,
        ActionId::KngtSwrd,
        ActionId::HeroSwrd,
        // Cannons
        ActionId::Cannon,
        ActionId::HiCannon,
        ActionId::MCannon,
        // Bombs
        ActionId::MiniBomb,
        ActionId::LilBomb,
        ActionId::CrosBomb,
        ActionId::BigBomb,
        // Waves
        ActionId::ShokWave,
        ActionId::SoniWave,
        ActionId::DynaWave,
        // Spread
        ActionId::Shotgun,
        ActionId::Spreader,
        ActionId::Bubbler,
        // Towers
        ActionId::FireTowr,
        ActionId::AquaTowr,
        ActionId::WoodTowr,
        // Quake
        ActionId::Quake1,
        ActionId::Quake2,
        ActionId::Quake3,
        // Thunder
        ActionId::Thunder1,
        ActionId::Thunder2,
        ActionId::Thunder3,
        // Misc
        ActionId::Ratton1,
        ActionId::Ratton2,
        ActionId::Ratton3,
        ActionId::Dash,
        ActionId::GutsPnch,
        ActionId::IcePunch,
        // Panel
        ActionId::Steal,
        ActionId::Geddon1,
        ActionId::Geddon2,
        ActionId::Repair,
    ]
}

// ============================================================================
// Setup System
// ============================================================================

pub fn setup_loadout(mut commands: Commands, loadout: Res<PlayerLoadout>) {
    // Initialize state
    commands.insert_resource(LoadoutState::default());

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(BG_COLOR),
            LoadoutMenu,
            CleanupOnStateExit(GameState::Loadout),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("LOADOUT"),
                TextFont::from_font_size(60.0),
                TextColor(TEXT_TITLE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Equip your Battle Chips"),
                TextFont::from_font_size(20.0),
                TextColor(TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Main content area (slots + details)
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    column_gap: Val::Px(40.0),
                    ..default()
                },))
                .with_children(|parent| {
                    // Left side: Action slots
                    parent
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(15.0),
                            ..default()
                        },))
                        .with_children(|parent| {
                            // Slots label
                            parent.spawn((
                                Text::new("Action Slots"),
                                TextFont::from_font_size(24.0),
                                TextColor(TEXT_NORMAL),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // 4 action slots
                            for i in 0..4 {
                                spawn_slot(parent, i, loadout.slots[i]);
                            }
                        });

                    // Right side: Details panel
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(350.0),
                                min_height: Val::Px(300.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(20.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(PANEL_BG),
                            BorderColor::all(PANEL_BORDER),
                            DetailsPanel,
                        ))
                        .with_children(|parent| {
                            // Details title
                            parent.spawn((
                                Text::new("Details"),
                                TextFont::from_font_size(22.0),
                                TextColor(TEXT_NORMAL),
                                Node {
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                },
                            ));

                            // Action name
                            parent.spawn((
                                Text::new("Select a slot"),
                                TextFont::from_font_size(28.0),
                                TextColor(TEXT_HIGHLIGHT),
                                DetailsName,
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Element indicator
                            parent.spawn((
                                Text::new(""),
                                TextFont::from_font_size(18.0),
                                TextColor(TEXT_MUTED),
                                DetailsElement,
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Description
                            parent.spawn((
                                Text::new("Navigate with Arrow Keys/D-Pad"),
                                TextFont::from_font_size(16.0),
                                TextColor(TEXT_MUTED),
                                DetailsDescription,
                                Node {
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                },
                            ));

                            // Stats
                            parent.spawn((
                                Text::new(""),
                                TextFont::from_font_size(14.0),
                                TextColor(TEXT_NORMAL),
                                DetailsStats,
                            ));
                        });
                });

            // Instructions at bottom
            parent.spawn((
                Text::new("[Arrow Keys/D-Pad] Navigate  |  [Enter/A] Select  |  [Esc/B] Back"),
                TextFont::from_font_size(16.0),
                TextColor(TEXT_MUTED),
                Node {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                },
            ));
        });

    // Spawn inventory panel (initially hidden)
    spawn_inventory_panel(&mut commands, &loadout);
}

/// Spawn a single action slot
fn spawn_slot(parent: &mut ChildSpawnerCommands, index: usize, action: Option<ActionId>) {
    let (bg_color, display_text, icon_color) = if let Some(action_id) = action {
        let blueprint = ActionBlueprint::get(action_id);
        (
            SLOT_BG_FILLED,
            format!("[{}] {}", index + 1, blueprint.name),
            blueprint.visuals.icon_color,
        )
    } else {
        (
            SLOT_BG_EMPTY,
            format!("[{}] Empty", index + 1),
            Color::srgb(0.3, 0.3, 0.4),
        )
    };

    let border_color = if index == 0 {
        SLOT_BORDER_SELECTED
    } else {
        SLOT_BORDER_NORMAL
    };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(280.0),
                height: Val::Px(70.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(15.0)),
                border: UiRect::all(Val::Px(3.0)),
                column_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor::all(border_color),
            LoadoutSlot { index },
        ))
        .with_children(|parent| {
            // Icon/color indicator
            parent.spawn((
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(icon_color),
                BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.3)),
            ));

            // Text
            parent.spawn((
                Text::new(display_text),
                TextFont::from_font_size(20.0),
                TextColor(TEXT_NORMAL),
            ));
        });
}

/// Spawn the inventory panel (hidden initially)
fn spawn_inventory_panel(commands: &mut Commands, loadout: &PlayerLoadout) {
    let all_actions = get_all_actions();

    // Create a full-screen overlay container for proper centering
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Dim background
            GlobalZIndex(100),                                 // Render on top
            InventoryPanel,
            Visibility::Hidden,
            CleanupOnStateExit(GameState::Loadout),
        ))
        .with_children(|overlay| {
            // Actual inventory panel (centered in overlay) - horizontal layout
            overlay
                .spawn((
                    Node {
                        width: Val::Px(800.0),
                        height: Val::Px(500.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(15.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(INVENTORY_BG),
                    BorderColor::all(Color::srgb(0.5, 0.6, 0.8)),
                ))
                .with_children(|panel| {
                    // Left side: Action list
                    panel
                        .spawn((Node {
                            width: Val::Px(450.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },))
                        .with_children(|left| {
                            // Title bar
                            left.spawn((Node {
                                width: Val::Percent(100.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },))
                                .with_children(|title_bar| {
                                    title_bar.spawn((
                                        Text::new("Select Action"),
                                        TextFont::from_font_size(22.0),
                                        TextColor(TEXT_TITLE),
                                    ));

                                    title_bar.spawn((
                                        Text::new("[Esc/B] Cancel"),
                                        TextFont::from_font_size(12.0),
                                        TextColor(TEXT_MUTED),
                                    ));
                                });

                            // Scrollable inventory list
                            left.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::scroll_y(),
                                    row_gap: Val::Px(4.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                                InventoryList,
                            ))
                            .with_children(|list| {
                                // Add "Clear Slot" option first (index 0)
                                spawn_inventory_clear_option(list, 0);

                                // Add all actions (index 1+)
                                for (i, action_id) in all_actions.iter().enumerate() {
                                    let is_equipped = loadout.is_equipped(*action_id);
                                    spawn_inventory_item(list, *action_id, is_equipped, i + 1);
                                }
                            });
                        });

                    // Right side: Details panel
                    panel
                        .spawn((
                            Node {
                                width: Val::Px(280.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(15.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            BackgroundColor(PANEL_BG),
                            BorderColor::all(PANEL_BORDER),
                        ))
                        .with_children(|details| {
                            // Details header
                            details.spawn((
                                Text::new("Details"),
                                TextFont::from_font_size(18.0),
                                TextColor(TEXT_MUTED),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));

                            // Action name
                            details.spawn((
                                Text::new("Select an action"),
                                TextFont::from_font_size(22.0),
                                TextColor(TEXT_HIGHLIGHT),
                                InventoryDetailsName,
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));

                            // Description
                            details.spawn((
                                Text::new(""),
                                TextFont::from_font_size(14.0),
                                TextColor(TEXT_NORMAL),
                                InventoryDetailsDesc,
                                Node {
                                    margin: UiRect::bottom(Val::Px(15.0)),
                                    ..default()
                                },
                            ));

                            // Stats
                            details.spawn((
                                Text::new(""),
                                TextFont::from_font_size(13.0),
                                TextColor(TEXT_MUTED),
                                InventoryDetailsStats,
                            ));
                        });
                });
        });
}

/// Spawn the "Clear Slot" option in inventory
fn spawn_inventory_clear_option(parent: &mut ChildSpawnerCommands, index: usize) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(INVENTORY_ITEM_BG),
            BorderColor::all(Color::NONE),
            InventoryItem {
                action_id: ActionId::default(), // Placeholder, handled specially
                index,
            },
        ))
        .with_children(|parent| {
            // X icon
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.5, 0.2, 0.2)),
            ));

            parent.spawn((
                Text::new("-- Clear Slot --"),
                TextFont::from_font_size(16.0),
                TextColor(TEXT_MUTED),
                InventoryItemText,
            ));
        });
}

/// Spawn a single inventory item
fn spawn_inventory_item(
    parent: &mut ChildSpawnerCommands,
    action_id: ActionId,
    is_equipped: bool,
    index: usize,
) {
    let blueprint = ActionBlueprint::get(action_id);

    let bg = if is_equipped {
        INVENTORY_ITEM_EQUIPPED
    } else {
        INVENTORY_ITEM_BG
    };

    let text_color = if is_equipped { TEXT_MUTED } else { TEXT_NORMAL };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(Color::NONE),
            InventoryItem { action_id, index },
        ))
        .with_children(|parent| {
            // Icon
            parent.spawn((
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                BackgroundColor(blueprint.visuals.icon_color),
            ));

            // Name + rarity
            parent.spawn((
                Text::new(format!(
                    "{} {}",
                    blueprint.name,
                    rarity_stars(blueprint.rarity)
                )),
                TextFont::from_font_size(16.0),
                TextColor(text_color),
                InventoryItemText,
            ));

            // Element indicator (if any)
            if blueprint.element != Element::None {
                parent.spawn((
                    Text::new(format!("{:?}", blueprint.element)),
                    TextFont::from_font_size(12.0),
                    TextColor(element_color(blueprint.element)),
                ));
            }

            // Equipped indicator
            if is_equipped {
                parent.spawn((
                    Text::new("[EQUIPPED]"),
                    TextFont::from_font_size(12.0),
                    TextColor(Color::srgb(0.8, 0.5, 0.2)),
                ));
            }
        });
}

// ============================================================================
// Update Systems
// ============================================================================

/// Handle input for loadout navigation
pub fn update_loadout_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    mut state: ResMut<LoadoutState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut inventory_visibility: Query<&mut Visibility, With<InventoryPanel>>,
) {
    // Gather gamepad input
    let mut gp_up = false;
    let mut gp_down = false;
    let mut gp_confirm = false;
    let mut gp_back = false;
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            gp_up = true;
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            gp_down = true;
        }
        if gamepad.just_pressed(GamepadButton::South) {
            gp_confirm = true;
        }
        if gamepad.just_pressed(GamepadButton::East) {
            gp_back = true;
        }
    }

    let up =
        keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) || gp_up;
    let down = keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::KeyS)
        || gp_down;
    let confirm = keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Space)
        || gp_confirm;
    let back = keyboard.just_pressed(KeyCode::Escape) || gp_back;

    let all_actions = get_all_actions();
    let total_inventory_items = all_actions.len() + 1; // +1 for "Clear Slot"

    // Input cooldown
    state.input_cooldown -= time.delta_secs();
    let can_navigate = state.input_cooldown <= 0.0;

    if state.inventory_open {
        // Inventory navigation
        if up && can_navigate {
            if state.inventory_cursor > 0 {
                state.inventory_cursor -= 1;
            }
            state.input_cooldown = 0.12;
        }
        if down && can_navigate {
            if state.inventory_cursor < total_inventory_items - 1 {
                state.inventory_cursor += 1;
            }
            state.input_cooldown = 0.12;
        }
        if back {
            // Close inventory (always responsive)
            state.inventory_open = false;
            state.editing_slot = None;
            if let Ok(mut vis) = inventory_visibility.single_mut() {
                *vis = Visibility::Hidden;
            }
            state.input_cooldown = 0.15;
        }
        if confirm && can_navigate {
            // Select action - handled in separate system
            state.input_cooldown = 0.15;
        }
    } else {
        // Slot navigation (inventory closed)
        if up && can_navigate {
            if state.selected_slot > 0 {
                state.selected_slot -= 1;
            }
            state.input_cooldown = 0.12;
        }
        if down && can_navigate {
            if state.selected_slot < 3 {
                state.selected_slot += 1;
            }
            state.input_cooldown = 0.12;
        }
        if confirm && can_navigate {
            // Open inventory for this slot
            state.inventory_open = true;
            state.editing_slot = Some(state.selected_slot);
            state.inventory_cursor = 0;
            state.just_opened_inventory = true; // Prevent same-frame selection
            if let Ok(mut vis) = inventory_visibility.single_mut() {
                *vis = Visibility::Inherited;
            }
            state.input_cooldown = 0.15;
        }

        // Quick slot selection with number keys
        if keyboard.just_pressed(KeyCode::Digit1) {
            state.selected_slot = 0;
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            state.selected_slot = 1;
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            state.selected_slot = 2;
        }
        if keyboard.just_pressed(KeyCode::Digit4) {
            state.selected_slot = 3;
        }
    }

    // Handle back to menu - ALWAYS check this, like campaign does
    if keyboard.just_pressed(KeyCode::Escape) && !state.inventory_open {
        next_state.set(GameState::MainMenu);
    }
}

/// Handle selecting an action from inventory
pub fn handle_inventory_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut state: ResMut<LoadoutState>,
    mut loadout: ResMut<PlayerLoadout>,
    mut inventory_visibility: Query<&mut Visibility, With<InventoryPanel>>,
) {
    if !state.inventory_open {
        return;
    }

    // Skip processing if inventory was just opened this frame
    if state.just_opened_inventory {
        state.just_opened_inventory = false;
        return;
    }

    let mut confirm =
        keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);

    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::South) {
            confirm = true;
        }
    }

    if confirm {
        if let Some(slot) = state.editing_slot {
            let all_actions = get_all_actions();

            if state.inventory_cursor == 0 {
                // "Clear Slot" selected
                loadout.clear_slot(slot);
            } else {
                // Action selected
                let action_index = state.inventory_cursor - 1;
                if action_index < all_actions.len() {
                    let action_id = all_actions[action_index];
                    // Only equip if not already equipped elsewhere
                    if !loadout.is_equipped(action_id) {
                        loadout.slots[slot] = Some(action_id);
                    }
                }
            }

            // Close inventory
            state.inventory_open = false;
            state.editing_slot = None;
            if let Ok(mut vis) = inventory_visibility.single_mut() {
                *vis = Visibility::Hidden;
            }
        }
    }
}

/// Update slot visuals based on selection
pub fn update_slot_visuals(
    state: Res<LoadoutState>,
    loadout: Res<PlayerLoadout>,
    mut slot_query: Query<(
        &LoadoutSlot,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
) {
    for (slot, mut bg, mut border, children) in &mut slot_query {
        let is_selected = slot.index == state.selected_slot && !state.inventory_open;

        // Update border
        *border = BorderColor::all(if is_selected {
            SLOT_BORDER_SELECTED
        } else {
            SLOT_BORDER_NORMAL
        });

        // Update background and text based on content
        if let Some(action_id) = loadout.slots[slot.index] {
            let blueprint = ActionBlueprint::get(action_id);
            bg.0 = SLOT_BG_FILLED;

            // Update text (second child, first is icon)
            if children.len() > 1 {
                if let Ok(mut text) = text_query.get_mut(children[1]) {
                    text.0 = format!("[{}] {}", slot.index + 1, blueprint.name);
                }
            }
            // Update icon color (first child)
            // Note: Icon is a Node, not directly modifiable here - would need separate query
        } else {
            bg.0 = SLOT_BG_EMPTY;
            if children.len() > 1 {
                if let Ok(mut text) = text_query.get_mut(children[1]) {
                    text.0 = format!("[{}] Empty", slot.index + 1);
                }
            }
        }
    }
}

/// Update details panel based on selection
pub fn update_details_panel(
    state: Res<LoadoutState>,
    loadout: Res<PlayerLoadout>,
    mut name_query: Query<(&mut Text, &mut TextColor), With<DetailsName>>,
    mut desc_query: Query<&mut Text, (With<DetailsDescription>, Without<DetailsName>)>,
    mut stats_query: Query<
        &mut Text,
        (
            With<DetailsStats>,
            Without<DetailsName>,
            Without<DetailsDescription>,
        ),
    >,
    mut elem_query: Query<
        (&mut Text, &mut TextColor),
        (
            With<DetailsElement>,
            Without<DetailsName>,
            Without<DetailsDescription>,
            Without<DetailsStats>,
        ),
    >,
) {
    let action_opt = if state.inventory_open {
        // Show details for inventory selection
        if state.inventory_cursor == 0 {
            None // Clear slot
        } else {
            let all_actions = get_all_actions();
            all_actions.get(state.inventory_cursor - 1).copied()
        }
    } else {
        // Show details for selected slot
        loadout.slots[state.selected_slot]
    };

    if let Some(action_id) = action_opt {
        let blueprint = ActionBlueprint::get(action_id);

        // Name
        if let Ok((mut text, mut color)) = name_query.single_mut() {
            text.0 = format!("{} {}", blueprint.name, rarity_stars(blueprint.rarity));
            color.0 = rarity_color(blueprint.rarity);
        }

        // Element
        if let Ok((mut text, mut color)) = elem_query.single_mut() {
            if blueprint.element != Element::None {
                text.0 = format!("Element: {:?}", blueprint.element);
                color.0 = element_color(blueprint.element);
            } else {
                text.0 = "Element: None".to_string();
                color.0 = TEXT_MUTED;
            }
        }

        // Description
        if let Ok(mut text) = desc_query.single_mut() {
            text.0 = blueprint.description.to_string();
        }

        // Stats
        if let Ok(mut text) = stats_query.single_mut() {
            text.0 = format!(
                "Cooldown: {:.1}s\nCharge: {:.1}s",
                blueprint.cooldown, blueprint.charge_time
            );
        }
    } else {
        // Empty slot or Clear option
        if let Ok((mut text, mut color)) = name_query.single_mut() {
            text.0 = if state.inventory_open && state.inventory_cursor == 0 {
                "Clear Slot".to_string()
            } else {
                "Empty Slot".to_string()
            };
            color.0 = TEXT_MUTED;
        }

        if let Ok((mut text, mut color)) = elem_query.single_mut() {
            text.0 = "".to_string();
            color.0 = TEXT_MUTED;
        }

        if let Ok(mut text) = desc_query.single_mut() {
            text.0 = if state.inventory_open && state.inventory_cursor == 0 {
                "Remove the equipped action from this slot".to_string()
            } else {
                "Press Enter/A to open inventory".to_string()
            };
        }

        if let Ok(mut text) = stats_query.single_mut() {
            text.0 = "".to_string();
        }
    }
}

/// Update inventory item visuals and handle scrolling
pub fn update_inventory_visuals(
    state: Res<LoadoutState>,
    loadout: Res<PlayerLoadout>,
    mut item_query: Query<(
        &InventoryItem,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    )>,
    mut text_query: Query<&mut TextColor, With<InventoryItemText>>,
    mut list_query: Query<&mut ScrollPosition, With<InventoryList>>,
) {
    if !state.inventory_open {
        return;
    }

    // Update scroll position to keep selected item visible
    if let Ok(mut scroll) = list_query.single_mut() {
        let item_height = 50.0; // 45px height + 5px gap
        let visible_height = 450.0; // Approximate visible area
        let selected_top = state.inventory_cursor as f32 * item_height;
        let selected_bottom = selected_top + item_height;

        // Scroll down if selection is below visible area
        if selected_bottom > scroll.y + visible_height {
            scroll.y = selected_bottom - visible_height;
        }
        // Scroll up if selection is above visible area
        if selected_top < scroll.y {
            scroll.y = selected_top;
        }
    }

    for (item, mut bg, mut border, children) in item_query.iter_mut() {
        let is_selected = item.index == state.inventory_cursor;

        // Check if this is the Clear option (index 0) or a real action
        let is_equipped = if item.index == 0 {
            false
        } else {
            loadout.is_equipped(item.action_id)
        };

        // Update colors
        if is_selected {
            bg.0 = INVENTORY_ITEM_SELECTED;
            *border = BorderColor::all(SLOT_BORDER_SELECTED);
        } else if is_equipped {
            bg.0 = INVENTORY_ITEM_EQUIPPED;
            *border = BorderColor::all(Color::NONE);
        } else {
            bg.0 = INVENTORY_ITEM_BG;
            *border = BorderColor::all(Color::NONE);
        }

        // Update text color
        for child in children.iter() {
            if let Ok(mut text_color) = text_query.get_mut(child) {
                text_color.0 = if is_equipped && !is_selected {
                    TEXT_MUTED
                } else {
                    TEXT_NORMAL
                };
            }
        }
    }
}

/// Update the details panel inside the inventory overlay
pub fn update_inventory_details(
    state: Res<LoadoutState>,
    loadout: Res<PlayerLoadout>,
    mut name_query: Query<(&mut Text, &mut TextColor), With<InventoryDetailsName>>,
    mut desc_query: Query<&mut Text, (With<InventoryDetailsDesc>, Without<InventoryDetailsName>)>,
    mut stats_query: Query<
        &mut Text,
        (
            With<InventoryDetailsStats>,
            Without<InventoryDetailsName>,
            Without<InventoryDetailsDesc>,
        ),
    >,
) {
    if !state.inventory_open {
        return;
    }

    let action_opt = if state.inventory_cursor == 0 {
        None // Clear slot
    } else {
        let all_actions = get_all_actions();
        all_actions.get(state.inventory_cursor - 1).copied()
    };

    if let Some(action_id) = action_opt {
        let blueprint = ActionBlueprint::get(action_id);
        let is_equipped = loadout.is_equipped(action_id);

        // Name with rarity and equipped status
        if let Ok((mut text, mut color)) = name_query.single_mut() {
            let equipped_tag = if is_equipped { " [EQUIPPED]" } else { "" };
            text.0 = format!(
                "{} {}{}",
                blueprint.name,
                rarity_stars(blueprint.rarity),
                equipped_tag
            );
            color.0 = if is_equipped {
                TEXT_MUTED
            } else {
                rarity_color(blueprint.rarity)
            };
        }

        // Description
        if let Ok(mut text) = desc_query.single_mut() {
            text.0 = blueprint.description.to_string();
        }

        // Stats
        if let Ok(mut text) = stats_query.single_mut() {
            let element_str = if blueprint.element != Element::None {
                format!("Element: {:?}\n", blueprint.element)
            } else {
                String::new()
            };
            text.0 = format!(
                "{}Cooldown: {:.1}s\nCharge: {:.1}s",
                element_str, blueprint.cooldown, blueprint.charge_time
            );
        }
    } else {
        // Clear slot option
        if let Ok((mut text, mut color)) = name_query.single_mut() {
            text.0 = "Clear Slot".to_string();
            color.0 = Color::srgb(0.8, 0.4, 0.4);
        }

        if let Ok(mut text) = desc_query.single_mut() {
            text.0 = "Remove the equipped action from this slot.".to_string();
        }

        if let Ok(mut text) = stats_query.single_mut() {
            text.0 = "".to_string();
        }
    }
}

// ============================================================================
// Cleanup
// ============================================================================

pub fn cleanup_loadout(mut commands: Commands) {
    commands.remove_resource::<LoadoutState>();
}
