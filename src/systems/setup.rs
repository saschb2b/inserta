use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{Anchor, MeshMaterial2d};

use crate::assets::FighterSprites;
use crate::components::*;
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // ========================================================================
    // Background - Deep cyber void
    // ========================================================================
    commands.spawn((
        Sprite {
            color: COLOR_BACKGROUND,
            custom_size: Some(Vec2::new(2000.0, 1200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
    ));

    // ========================================================================
    // Cyber grid lines in background
    // ========================================================================
    let grid_line_h_mesh = meshes.add(Rectangle::new(900.0, 1.5));
    let grid_line_v_mesh = meshes.add(Rectangle::new(1.5, 600.0));
    let grid_line_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE));
    let grid_line_bright_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE_BRIGHT));

    for i in -10..=10 {
        let y = i as f32 * 35.0;
        let mat = if i % 3 == 0 {
            grid_line_bright_mat.clone()
        } else {
            grid_line_mat.clone()
        };
        commands.spawn((
            Mesh2d(grid_line_h_mesh.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(0.0, y, Z_GRID_LINES),
        ));
    }

    for i in -12..=12 {
        let x = i as f32 * 45.0;
        let mat = if i % 3 == 0 {
            grid_line_bright_mat.clone()
        } else {
            grid_line_mat.clone()
        };
        commands.spawn((
            Mesh2d(grid_line_v_mesh.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(x, 0.0, Z_GRID_LINES),
        ));
    }

    // ========================================================================
    // BGM
    // ========================================================================
    let bgm: Handle<AudioSource> = asset_server.load("audio/bgm/battle.mp3");
    commands.spawn((
        AudioPlayer::new(bgm),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.45)),
    ));

    // ========================================================================
    // MMBN-style Panel Components
    // ========================================================================

    // Panel dimensions
    let panel_w = TILE_W;
    let panel_h = TILE_H;
    let frame_thickness = 6.0;
    let inner_w = panel_w - frame_thickness * 2.0;
    let inner_h = panel_h - frame_thickness * 2.0;
    let corner_radius = 8.0;

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
    let highlight_mesh = meshes.add(rect_mesh(inner_w - 12.0, 6.0));
    let front_face_mesh = meshes.add(rect_mesh(panel_w, PANEL_DEPTH));

    // ========================================================================
    // Materials
    // ========================================================================

    // Player panel colors (red/orange like MMBN)
    let player_outer_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_DARK));
    let player_frame_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_FRAME));
    let player_inner_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_TOP));
    let player_front_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL_SIDE));

    // Enemy panel colors (blue like MMBN)
    let enemy_outer_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_DARK));
    let enemy_frame_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_FRAME));
    let enemy_inner_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_TOP));
    let enemy_front_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL_SIDE));

    // Shared
    let highlight_mat = materials.add(ColorMaterial::from(COLOR_PANEL_HIGHLIGHT));

    // ========================================================================
    // Spawn grid panels (MMBN style)
    // ========================================================================
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let is_player = x < PLAYER_AREA_WIDTH;

            let (outer_mat, frame_mat, inner_mat, front_mat) = if is_player {
                (
                    player_outer_mat.clone(),
                    player_frame_mat.clone(),
                    player_inner_mat.clone(),
                    player_front_mat.clone(),
                )
            } else {
                (
                    enemy_outer_mat.clone(),
                    enemy_frame_mat.clone(),
                    enemy_inner_mat.clone(),
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
            ));

            // 2. Outer frame background (darkest)
            commands.spawn((
                Mesh2d(outer_frame_mesh.clone()),
                MeshMaterial2d(outer_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + z_offset),
            ));

            // 3. Inner frame border (medium tone - creates the grid lines)
            commands.spawn((
                Mesh2d(inner_frame_mesh.clone()),
                MeshMaterial2d(frame_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + 0.1 + z_offset),
            ));

            // 4. Inner panel surface (brightest - the actual walkable area)
            commands.spawn((
                Mesh2d(inner_panel_mesh.clone()),
                MeshMaterial2d(inner_mat),
                Transform::from_xyz(world.x, world.y, Z_PANEL_TOP + 0.2 + z_offset),
            ));

            // 5. Highlight strip at top of inner panel
            commands.spawn((
                Mesh2d(highlight_mesh.clone()),
                MeshMaterial2d(highlight_mat.clone()),
                Transform::from_xyz(
                    world.x,
                    world.y + inner_h / 2.0 - 8.0,
                    Z_PANEL_TOP + 0.3 + z_offset,
                ),
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
    // Player
    // ========================================================================
    commands.spawn((
        Sprite {
            image: fighter_idle,
            texture_atlas: Some(fighter_layout.into()),
            color: Color::WHITE,
            anchor: Anchor::Custom(FIGHTER_ANCHOR),
            custom_size: Some(FIGHTER_DRAW_SIZE),
            ..default()
        },
        Transform::default(),
        GridPosition { x: 1, y: 1 },
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
    ));

    // ========================================================================
    // Enemy
    // ========================================================================
    let enemy_entity = commands
        .spawn((
            Sprite {
                color: COLOR_ENEMY,
                anchor: Anchor::BottomCenter,
                custom_size: Some(Vec2::new(70.0, 90.0)),
                ..default()
            },
            Transform::default(),
            GridPosition { x: 4, y: 1 },
            RenderConfig {
                offset: CHARACTER_OFFSET,
                base_z: Z_CHARACTER,
            },
            BaseColor(COLOR_ENEMY),
            Enemy,
            Health {
                current: 100,
                max: 100,
            },
        ))
        .id();

    commands.entity(enemy_entity).with_children(|parent| {
        parent.spawn((
            Sprite {
                color: COLOR_HP_PLATE,
                custom_size: Some(Vec2::new(48.0, 22.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 96.0, 0.0),
        ));

        parent.spawn((
            Text2d::new("100"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(COLOR_TEXT_SHADOW),
            Transform::from_xyz(1.5, 94.5, 0.1),
            HealthText,
        ));

        parent.spawn((
            Text2d::new("100"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(COLOR_TEXT),
            Transform::from_xyz(0.0, 96.0, 0.2),
            HealthText,
        ));
    });
}
