use bevy::asset::RenderAssetUsages;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Justify;

use crate::assets::{FighterSprites, SlimeSprites};
use crate::components::{
    ActionBar, ActionChargeBar, ActionCooldownOverlay, ActionKeyText, ActionSlot, ActionSlotUI,
    ActionType, ArenaConfig, BaseColor, CleanupOnStateExit, Enemy, EnemyAI, EnemyConfig, EnemyType,
    FighterAnim, FighterAnimState, GameState, GridPosition, Health, HealthText, Player,
    PlayerHealthText, RenderConfig, SlimeAnim, SlimeAnimState, TilePanel,
};
use crate::constants::*;
use crate::systems::grid_utils::{tile_center_world, tile_floor_world};

/// Simple rectangle mesh
fn rect_mesh(w: f32, h: f32) -> Mesh {
    let half_w = w / 2.0;
    let half_h = h / 2.0;

    let positions = vec![
        [-half_w, -half_h, 0.0],
        [half_w, -half_h, 0.0],
        [half_w, half_h, 0.0],
        [-half_w, half_h, 0.0],
    ];

    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    mesh
}

/// Creates a rounded rectangle mesh (approximated with beveled corners)
fn rounded_rect_mesh(w: f32, h: f32, corner: f32) -> Mesh {
    let half_w = w / 2.0;
    let half_h = h / 2.0;
    let c = corner.min(half_w).min(half_h);

    // 8 vertices for beveled corners
    let positions = vec![
        // Bottom edge (left to right)
        [-half_w + c, -half_h, 0.0], // 0
        [half_w - c, -half_h, 0.0],  // 1
        // Right edge (bottom to top)
        [half_w, -half_h + c, 0.0], // 2
        [half_w, half_h - c, 0.0],  // 3
        // Top edge (right to left)
        [half_w - c, half_h, 0.0],  // 4
        [-half_w + c, half_h, 0.0], // 5
        // Left edge (top to bottom)
        [-half_w, half_h - c, 0.0],  // 6
        [-half_w, -half_h + c, 0.0], // 7
        // Center
        [0.0, 0.0, 0.0], // 8
    ];

    let uvs = vec![
        [c / w, 0.0],
        [1.0 - c / w, 0.0],
        [1.0, c / h],
        [1.0, 1.0 - c / h],
        [1.0 - c / w, 1.0],
        [c / w, 1.0],
        [0.0, 1.0 - c / h],
        [0.0, c / h],
        [0.5, 0.5],
    ];

    // Fan from center
    let indices = Indices::U32(vec![
        8, 0, 1, 8, 1, 2, 8, 2, 3, 8, 3, 4, 8, 4, 5, 8, 5, 6, 8, 6, 7, 8, 7, 0,
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    mesh
}

/// Creates the inner border/frame mesh (hollow rounded rectangle)
fn frame_mesh(outer_w: f32, outer_h: f32, inner_w: f32, inner_h: f32, corner: f32) -> Mesh {
    let ohw = outer_w / 2.0;
    let ohh = outer_h / 2.0;
    let ihw = inner_w / 2.0;
    let ihh = inner_h / 2.0;
    let oc = corner.min(ohw).min(ohh);
    let ic = (corner * 0.6).min(ihw).min(ihh);

    // Outer vertices (0-7)
    // Inner vertices (8-15)
    let positions = vec![
        // Outer ring
        [-ohw + oc, -ohh, 0.0],
        [ohw - oc, -ohh, 0.0],
        [ohw, -ohh + oc, 0.0],
        [ohw, ohh - oc, 0.0],
        [ohw - oc, ohh, 0.0],
        [-ohw + oc, ohh, 0.0],
        [-ohw, ohh - oc, 0.0],
        [-ohw, -ohh + oc, 0.0],
        // Inner ring
        [-ihw + ic, -ihh, 0.0],
        [ihw - ic, -ihh, 0.0],
        [ihw, -ihh + ic, 0.0],
        [ihw, ihh - ic, 0.0],
        [ihw - ic, ihh, 0.0],
        [-ihw + ic, ihh, 0.0],
        [-ihw, ihh - ic, 0.0],
        [-ihw, -ihh + ic, 0.0],
    ];

    let uvs: Vec<[f32; 2]> = positions.iter().map(|_| [0.5, 0.5]).collect();

    // Connect outer to inner with quads
    let indices = Indices::U32(vec![
        // Bottom
        0, 1, 9, 0, 9, 8, // Bottom-right corner
        1, 2, 10, 1, 10, 9, // Right
        2, 3, 11, 2, 11, 10, // Top-right corner
        3, 4, 12, 3, 12, 11, // Top
        4, 5, 13, 4, 13, 12, // Top-left corner
        5, 6, 14, 5, 14, 13, // Left
        6, 7, 15, 6, 15, 14, // Bottom-left corner
        7, 0, 8, 7, 8, 15,
    ]);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    mesh
}

// ============================================================================
// Global Setup (runs once at app startup)
// ============================================================================

/// Setup that runs once at app start - camera only
pub fn setup_global(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ============================================================================
// Arena Setup (runs when entering Playing state)
// ============================================================================

/// Setup the arena background, grid, BGM, and spawn entities based on ArenaConfig
pub fn setup_arena(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<ArenaConfig>,
) {
    // ========================================================================
    // Background - Deep cyber void
    // ========================================================================
    commands.spawn((
        Sprite {
            color: COLOR_BACKGROUND,
            custom_size: Some(Vec2::new(SCREEN_WIDTH + 200.0, SCREEN_HEIGHT + 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
        CleanupOnStateExit(GameState::Playing),
    ));

    // ========================================================================
    // Cyber grid lines in background
    // ========================================================================
    let grid_line_h_mesh = meshes.add(Rectangle::new(SCREEN_WIDTH + 100.0, 1.5));
    let grid_line_v_mesh = meshes.add(Rectangle::new(1.5, SCREEN_HEIGHT + 100.0));
    let grid_line_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE));
    let grid_line_bright_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE_BRIGHT));

    // Horizontal lines - spaced to cover screen
    for i in -12..=12 {
        let y = i as f32 * 40.0 + ARENA_Y_OFFSET;
        let mat = if i % 3 == 0 {
            grid_line_bright_mat.clone()
        } else {
            grid_line_mat.clone()
        };
        commands.spawn((
            Mesh2d(grid_line_h_mesh.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(0.0, y, Z_GRID_LINES),
            CleanupOnStateExit(GameState::Playing),
        ));
    }

    // Vertical lines - spaced to cover screen
    for i in -16..=16 {
        let x = i as f32 * 50.0;
        let mat = if i % 3 == 0 {
            grid_line_bright_mat.clone()
        } else {
            grid_line_mat.clone()
        };
        commands.spawn((
            Mesh2d(grid_line_v_mesh.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(x, ARENA_Y_OFFSET, Z_GRID_LINES),
            CleanupOnStateExit(GameState::Playing),
        ));
    }

    // ========================================================================
    // BGM
    // ========================================================================
    let bgm: Handle<AudioSource> = asset_server.load("audio/bgm/battle.mp3");
    commands.spawn((
        AudioPlayer::new(bgm),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.45)),
        CleanupOnStateExit(GameState::Playing),
    ));

    // ========================================================================
    // MMBN-style Panel Components
    // ========================================================================

    // Panel dimensions (scaled for 1280x800)
    let panel_w = TILE_W;
    let panel_h = TILE_H;
    let frame_thickness = 10.0;
    let inner_w = panel_w - frame_thickness * 2.0;
    let inner_h = panel_h - frame_thickness * 2.0;
    let corner_radius = 12.0;

    // Meshes
    let outer_frame_mesh = meshes.add(rounded_rect_mesh(panel_w, panel_h, corner_radius));
    let inner_frame_mesh = meshes.add(frame_mesh(
        panel_w - 2.0,
        panel_h - 2.0,
        inner_w,
        inner_h,
        corner_radius,
    ));
    let inner_panel_mesh = meshes.add(rounded_rect_mesh(
        inner_w - 4.0,
        inner_h - 4.0,
        corner_radius * 0.5,
    ));
    let highlight_mesh = meshes.add(rect_mesh(inner_w - 20.0, 8.0));
    let front_face_mesh = meshes.add(rect_mesh(panel_w, PANEL_DEPTH));

    // ========================================================================
    // Materials
    // ========================================================================

    // Player panel colors (red/orange like MMBN)
    let player_outer_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_DARK));
    let player_frame_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_FRAME));
    let player_front_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_SIDE));

    // Enemy panel colors (blue like MMBN)
    let enemy_outer_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_DARK));
    let enemy_frame_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_FRAME));
    let enemy_front_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_SIDE));

    // Shared
    let highlight_mat = materials.add(ColorMaterial::from(COLOR_PANEL_HIGHLIGHT));

    // ========================================================================
    // Spawn grid panels (MMBN style)
    // ========================================================================
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let is_player = x < PLAYER_AREA_WIDTH;

            // Create unique material for inner panel (so it can be highlighted individually)
            let inner_color = if is_player {
                COLOR_PLAYER_PANEL_TOP
            } else {
                COLOR_ENEMY_PANEL_TOP
            };
            let unique_inner_mat = materials.add(ColorMaterial::from(inner_color));

            let (outer_mat, frame_mat, front_mat) = if is_player {
                (
                    player_outer_mat.clone(),
                    player_frame_mat.clone(),
                    player_front_mat.clone(),
                )
            } else {
                (
                    enemy_outer_mat.clone(),
                    enemy_frame_mat.clone(),
                    enemy_front_mat.clone(),
                )
            };

            let floor = tile_floor_world(x, y);
            let world = tile_center_world(x, y);
            let z_offset = -floor.y * DEPTH_Y_TO_Z;

            // 1. Front face (3D depth effect) - positioned below the panel
            commands.spawn((
                Mesh2d(front_face_mesh.clone()),
                MeshMaterial2d(front_mat),
                Transform::from_xyz(
                    world.x,
                    world.y - panel_h / 2.0 - PANEL_DEPTH / 2.0,
                    Z_PANEL_SIDE + z_offset,
                ),
                CleanupOnStateExit(GameState::Playing),
            ));

            // 2. Outer frame background (darkest)
            commands.spawn((
                Mesh2d(outer_frame_mesh.clone()),
                MeshMaterial2d(outer_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + z_offset),
                CleanupOnStateExit(GameState::Playing),
            ));

            // 3. Inner frame border (medium tone - creates the grid lines)
            commands.spawn((
                Mesh2d(inner_frame_mesh.clone()),
                MeshMaterial2d(frame_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + 0.1 + z_offset),
                CleanupOnStateExit(GameState::Playing),
            ));

            // 4. Inner panel surface (brightest - the actual walkable area)
            // Each tile gets its own unique material for individual highlighting
            commands.spawn((
                Mesh2d(inner_panel_mesh.clone()),
                MeshMaterial2d(unique_inner_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + 0.2 + z_offset),
                TilePanel { x, y },
                CleanupOnStateExit(GameState::Playing),
            ));

            // 5. Highlight strip at top of inner panel
            commands.spawn((
                Mesh2d(highlight_mesh.clone()),
                MeshMaterial2d(highlight_mat.clone()),
                Transform::from_xyz(
                    world.x,
                    world.y + inner_h / 2.0 - 12.0,
                    Z_PANEL_TOP + 0.3 + z_offset,
                ),
                CleanupOnStateExit(GameState::Playing),
            ));
        }
    }

    // ========================================================================
    // Fighter sprite sheets
    // ========================================================================
    let fighter_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(128, 128),
        10,
        1,
        None,
        None,
    ));

    let fighter_idle = asset_server.load("characters/fighter/male_hero-idle.png");
    let fighter_walk = asset_server.load("characters/fighter/male_hero-walk.png");
    let fighter_shoot = asset_server.load("characters/fighter/male_hero-combo_1.png");

    commands.insert_resource(FighterSprites {
        layout: fighter_layout.clone(),
        idle: fighter_idle.clone(),
        walk: fighter_walk.clone(),
        shoot: fighter_shoot.clone(),
        idle_frames: 10,
        walk_frames: 10,
        shoot_frames: 3,
    });

    // ========================================================================
    // Player (from config)
    // ========================================================================
    let fighter_config = &config.fighter;
    commands.spawn((
        Sprite {
            image: fighter_idle,
            texture_atlas: Some(fighter_layout.into()),
            color: Color::WHITE,
            custom_size: Some(FIGHTER_DRAW_SIZE),
            ..default()
        },
        Anchor(FIGHTER_ANCHOR),
        Transform::default(),
        GridPosition {
            x: fighter_config.start_x,
            y: fighter_config.start_y,
        },
        RenderConfig {
            offset: CHARACTER_OFFSET,
            base_z: Z_CHARACTER,
        },
        FighterAnim {
            state: FighterAnimState::Idle,
            frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        Player,
        Health {
            current: fighter_config.max_hp,
            max: fighter_config.max_hp,
        },
        BaseColor(Color::WHITE),
        CleanupOnStateExit(GameState::Playing),
    ));

    // Player HP display (top-left area, above arena)
    commands.spawn((
        Text2d::new(format!("HP: {}", fighter_config.max_hp)),
        TextLayout::new_with_justify(Justify::Left),
        TextFont::from_font_size(28.0),
        TextColor(COLOR_TEXT),
        Transform::from_xyz(-580.0, 360.0, Z_UI),
        PlayerHealthText,
        CleanupOnStateExit(GameState::Playing),
    ));

    // ========================================================================
    // Slime sprite sheets (16x16 frames, 3 columns per row)
    // ========================================================================
    // IDLE - WALK: 48x48 = 3x3 grid, 7 frames used
    // DEAD: 48x48 = 3x3 grid, 7 frames used
    // SHOOTING: 48x64 = 3x4 grid, 10 frames used
    let slime_idle_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        3, // 3 columns
        3, // 3 rows
        None,
        None,
    ));

    let slime_shoot_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        3, // 3 columns
        4, // 4 rows
        None,
        None,
    ));

    let slime_idle = asset_server.load("enemies/slime/IDLE - WALK.png");
    let slime_shoot = asset_server.load("enemies/slime/SHOOTING.png");
    let slime_dead = asset_server.load("enemies/slime/DEAD.png");

    commands.insert_resource(SlimeSprites {
        layout: slime_idle_layout.clone(),
        shoot_layout: slime_shoot_layout,
        idle: slime_idle.clone(),
        shoot: slime_shoot.clone(),
        dead: slime_dead.clone(),
        idle_frames: 7,
        shoot_frames: 10,
        dead_frames: 7,
    });

    // ========================================================================
    // Enemies (from config)
    // ========================================================================
    for enemy_config in &config.enemies {
        match enemy_config.enemy_type {
            EnemyType::Slime => {
                spawn_slime(
                    &mut commands,
                    slime_idle.clone(),
                    slime_idle_layout.clone(),
                    enemy_config,
                );
            }
        }
    }
}

/// Spawn a slime enemy
fn spawn_slime(
    commands: &mut Commands,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    config: &EnemyConfig,
) {
    let enemy_entity = commands
        .spawn((
            Sprite {
                image: texture,
                texture_atlas: Some(layout.into()),
                color: Color::WHITE,
                custom_size: Some(SLIME_DRAW_SIZE),
                flip_x: true, // Mirror to face left (toward player)
                ..default()
            },
            Anchor(SLIME_ANCHOR),
            Transform::default(),
            GridPosition {
                x: config.start_x,
                y: config.start_y,
            },
            RenderConfig {
                offset: SLIME_OFFSET,
                base_z: Z_CHARACTER,
            },
            SlimeAnim {
                state: SlimeAnimState::Idle,
                frame: 0,
                timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            },
            Enemy,
            Health {
                current: config.max_hp,
                max: config.max_hp,
            },
            EnemyAI {
                move_timer: Timer::from_seconds(ENEMY_MOVE_COOLDOWN, TimerMode::Repeating),
                shoot_timer: Timer::from_seconds(ENEMY_SHOOT_COOLDOWN, TimerMode::Repeating),
            },
            BaseColor(Color::WHITE),
            CleanupOnStateExit(GameState::Playing),
        ))
        .id();

    commands.entity(enemy_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: COLOR_HP_PLATE,
                custom_size: Some(Vec2::new(64.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 80.0, 0.0),
        ));

        parent.spawn((
            Text2d::new(config.max_hp.to_string()),
            TextLayout::new_with_justify(Justify::Center),
            TextFont::from_font_size(20.0),
            TextColor(COLOR_TEXT_SHADOW),
            Transform::from_xyz(1.5, 78.5, 0.1),
            HealthText,
        ));

        parent.spawn((
            Text2d::new(config.max_hp.to_string()),
            TextLayout::new_with_justify(Justify::Center),
            TextFont::from_font_size(20.0),
            TextColor(COLOR_TEXT),
            Transform::from_xyz(0.0, 80.0, 0.2),
            HealthText,
        ));
    });
}

// ============================================================================
// Action Bar Setup (runs when entering Playing state)
// ============================================================================

/// Spawns the action bar UI at the bottom of the screen
pub fn setup_action_bar(mut commands: Commands, config: Res<ArenaConfig>) {
    let actions = &config.fighter.actions;
    let slot_count = actions.len() as f32;

    if slot_count == 0.0 {
        return;
    }

    let total_width = (ACTION_SLOT_SIZE * slot_count) + (ACTION_SLOT_SPACING * (slot_count - 1.0));
    let start_x = -total_width / 2.0 + ACTION_SLOT_SIZE / 2.0;

    // Pre-calculate all slot data
    let slot_data: Vec<ActionSlotData> = actions
        .iter()
        .enumerate()
        .map(|(i, action_type)| ActionSlotData {
            slot_index: i,
            x_offset: start_x + (ACTION_SLOT_SIZE + ACTION_SLOT_SPACING) * i as f32,
            key_label: format!("{}", i + 1),
            icon_color: get_action_icon_color(action_type),
        })
        .collect();

    // Spawn action bar container
    commands
        .spawn((
            Transform::from_xyz(0.0, ACTION_BAR_Y, Z_UI),
            Visibility::Visible,
            ActionBar,
            CleanupOnStateExit(GameState::Playing),
        ))
        .with_children(|parent| {
            for data in &slot_data {
                parent
                    .spawn((
                        Sprite {
                            color: COLOR_ACTION_SLOT_BG,
                            custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(data.x_offset, 0.0, 0.0),
                        ActionSlotUI {
                            slot_index: data.slot_index,
                        },
                    ))
                    .with_children(|slot| {
                        let slot_index = data.slot_index;
                        let icon_color = data.icon_color;
                        let key_label = data.key_label.clone();

                        // Border
                        slot.spawn((
                            Sprite {
                                color: COLOR_ACTION_SLOT_BORDER,
                                custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE + 4.0)),
                                ..default()
                            },
                            Transform::from_xyz(0.0, 0.0, -0.1),
                        ));

                        // Action icon
                        slot.spawn((
                            Sprite {
                                color: icon_color,
                                custom_size: Some(Vec2::splat(ACTION_SLOT_SIZE * 0.6)),
                                ..default()
                            },
                            Transform::from_xyz(0.0, 2.0, 0.1),
                        ));

                        // Cooldown overlay
                        slot.spawn((
                            Sprite {
                                color: COLOR_ACTION_COOLDOWN,
                                custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 0.0)),
                                ..default()
                            },
                            Transform::from_xyz(0.0, 0.0, 0.2),
                            ActionCooldownOverlay { slot_index },
                        ));

                        // Charge bar
                        slot.spawn((
                            Sprite {
                                color: COLOR_ACTION_CHARGE,
                                custom_size: Some(Vec2::new(ACTION_SLOT_SIZE - 4.0, 4.0)),
                                ..default()
                            },
                            Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 + 6.0, 0.3),
                            Visibility::Hidden,
                            ActionChargeBar { slot_index },
                        ));

                        // Key label
                        slot.spawn((
                            Text2d::new(key_label),
                            TextColor(COLOR_ACTION_KEY_TEXT),
                            TextFont::from_font_size(14.0),
                            Transform::from_xyz(0.0, -ACTION_SLOT_SIZE / 2.0 - 12.0, 0.1),
                            ActionKeyText { slot_index },
                        ));

                        // Ready indicator
                        slot.spawn((
                            Sprite {
                                color: COLOR_ACTION_SLOT_READY,
                                custom_size: Some(Vec2::splat(8.0)),
                                ..default()
                            },
                            Transform::from_xyz(
                                ACTION_SLOT_SIZE / 2.0 - 8.0,
                                ACTION_SLOT_SIZE / 2.0 - 8.0,
                                0.3,
                            ),
                            ActionReadyIndicator { slot_index },
                        ));
                    });
            }
        });
}

/// Get the icon color for an action type
fn get_action_icon_color(action_type: &ActionType) -> Color {
    match action_type {
        ActionType::ChargedShot => COLOR_CHARGED_SHOT_ICON,
        ActionType::Heal => COLOR_HEAL_ICON,
        ActionType::Shield => COLOR_SHIELD_ICON,
        ActionType::WideSword => COLOR_WIDESWORD_ICON,
    }
}

/// Helper struct to hold action slot spawn data
struct ActionSlotData {
    slot_index: usize,
    x_offset: f32,
    key_label: String,
    icon_color: Color,
}

/// Marker for the ready indicator dot
#[derive(Component)]
pub struct ActionReadyIndicator {
    pub slot_index: usize,
}

/// Spawn the actual ActionSlot components based on config
pub fn spawn_player_actions(mut commands: Commands, config: Res<ArenaConfig>) {
    for (i, action_type) in config.fighter.actions.iter().enumerate() {
        let (cooldown, charge_time) = get_action_timings(action_type);
        commands.spawn((
            ActionSlot::new(i, *action_type, cooldown, charge_time),
            CleanupOnStateExit(GameState::Playing),
        ));
    }
}

/// Get cooldown and charge time for an action type
fn get_action_timings(action_type: &ActionType) -> (f32, f32) {
    match action_type {
        ActionType::ChargedShot => (CHARGED_SHOT_COOLDOWN, CHARGED_SHOT_CHARGE_TIME),
        ActionType::Heal => (HEAL_COOLDOWN, HEAL_CHARGE_TIME),
        ActionType::Shield => (SHIELD_COOLDOWN, SHIELD_CHARGE_TIME),
        ActionType::WideSword => (WIDESWORD_COOLDOWN, WIDESWORD_CHARGE_TIME),
    }
}

// ============================================================================
// Cleanup
// ============================================================================

/// Cleanup for when leaving Playing state
pub fn cleanup_arena(mut commands: Commands, query: Query<(Entity, &CleanupOnStateExit)>) {
    for (entity, scoped) in &query {
        if scoped.0 == GameState::Playing {
            commands.entity(entity).despawn();
        }
    }
}

/// Cleanup for when leaving Splash state
pub fn cleanup_splash_entities(
    mut commands: Commands,
    query: Query<(Entity, &CleanupOnStateExit)>,
) {
    for (entity, scoped) in &query {
        if scoped.0 == GameState::Splash {
            commands.entity(entity).despawn();
        }
    }
}

/// Cleanup for when leaving MainMenu state
pub fn cleanup_menu_entities(mut commands: Commands, query: Query<(Entity, &CleanupOnStateExit)>) {
    for (entity, scoped) in &query {
        if scoped.0 == GameState::MainMenu {
            commands.entity(entity).despawn();
        }
    }
}
