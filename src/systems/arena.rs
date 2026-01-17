//! Arena rendering system
//!
//! This module handles all arena visual rendering:
//! - Background and cyber grid lines
//! - MMBN-style tile panels (sprites with responsive scaling)
//!
//! Tile styling can be customized by modifying the panel colors in constants.rs
//! or by adjusting the mesh generation functions here.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::components::{CleanupOnStateExit, GameState, TileAssets, TileHighlightState, TilePanel};
use crate::constants::*;
use crate::resources::ArenaLayout;

// ============================================================================
// Mesh Helpers
// ============================================================================

/// Simple rectangle mesh
pub fn rect_mesh(w: f32, h: f32) -> Mesh {
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
pub fn rounded_rect_mesh(w: f32, h: f32, corner: f32) -> Mesh {
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
pub fn frame_mesh(outer_w: f32, outer_h: f32, inner_w: f32, inner_h: f32, corner: f32) -> Mesh {
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
// Arena Rendering
// ============================================================================

/// Spawns the arena background (deep cyber void)
pub fn spawn_background(commands: &mut Commands, layout: &ArenaLayout) {
    commands.spawn((
        Sprite {
            color: COLOR_BACKGROUND,
            custom_size: Some(Vec2::new(
                layout.screen_width + 200.0,
                layout.screen_height + 200.0,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
        CleanupOnStateExit(GameState::Playing),
    ));
}

/// Spawns the cyber grid lines in the background
pub fn spawn_grid_lines(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    layout: &ArenaLayout,
) {
    let grid_line_h_mesh = meshes.add(Rectangle::new(layout.screen_width + 100.0, 1.5));
    let grid_line_v_mesh = meshes.add(Rectangle::new(1.5, layout.screen_height + 100.0));
    let grid_line_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE));
    let grid_line_bright_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE_BRIGHT));

    // Horizontal lines
    for i in -10..=10 {
        let y = i as f32 * 60.0 + layout.arena_y_offset;
        let mat = if i % 4 == 0 {
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

    // Vertical lines
    for i in -14..=14 {
        let x = i as f32 * 70.0;
        let mat = if i % 4 == 0 {
            grid_line_bright_mat.clone()
        } else {
            grid_line_mat.clone()
        };
        commands.spawn((
            Mesh2d(grid_line_v_mesh.clone()),
            MeshMaterial2d(mat),
            Transform::from_xyz(x, layout.arena_y_offset, Z_GRID_LINES),
            CleanupOnStateExit(GameState::Playing),
        ));
    }
}

/// Spawns all MMBN-style tile panels for the arena grid using sprite assets
pub fn spawn_tile_panels(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    layout: &ArenaLayout,
) {
    // Load all tile sprite assets (normal and highlighted variants)
    let tile_assets = TileAssets {
        red_normal: asset_server.load("battle/arena/tile_red.png"),
        red_highlighted: asset_server.load("battle/arena/tile_red_highlighted.png"),
        blue_normal: asset_server.load("battle/arena/tile_blue.png"),
        blue_highlighted: asset_server.load("battle/arena/tile_blue_highlighted.png"),
    };

    // Spawn grid panels - render from back row (y=2) to front row (y=0)
    // so that front rows overlap back rows correctly
    for y in (0..GRID_HEIGHT).rev() {
        for x in 0..GRID_WIDTH {
            let is_player = x < PLAYER_AREA_WIDTH;
            let tile_texture = if is_player {
                tile_assets.red_normal.clone()
            } else {
                tile_assets.blue_normal.clone()
            };

            let sprite_pos = layout.tile_sprite_world(x, y);

            // Z-ordering: back rows behind front rows
            // Higher y = further back = lower z
            let z = Z_PANEL_TOP - (y as f32) * 0.1;

            commands.spawn((
                Sprite {
                    image: tile_texture,
                    custom_size: Some(layout.tile_size()),
                    ..default()
                },
                Transform::from_xyz(sprite_pos.x, sprite_pos.y, z),
                TilePanel { x, y },
                TileHighlightState::new(is_player),
                CleanupOnStateExit(GameState::Playing),
            ));
        }
    }

    // Insert tile assets as a resource for the highlight system
    commands.insert_resource(tile_assets);
}

// ============================================================================
// Main Arena Setup System
// ============================================================================

/// Spawns all arena visuals: background, grid lines, and tile panels
///
/// This is the main entry point for arena rendering. Call this from setup_arena.
pub fn spawn_arena_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    layout: &ArenaLayout,
) {
    spawn_background(commands, layout);
    spawn_grid_lines(commands, meshes, materials, layout);
    spawn_tile_panels(commands, asset_server, layout);
}
