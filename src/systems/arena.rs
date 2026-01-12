//! Arena rendering system
//!
//! This module handles all arena visual rendering:
//! - Background and cyber grid lines
//! - MMBN-style tile panels (outer frame, inner frame, surface, highlights)
//!
//! Tile styling can be customized by modifying the panel colors in constants.rs
//! or by adjusting the mesh generation functions here.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::components::{CleanupOnStateExit, GameState, TilePanel};
use crate::constants::*;
use crate::systems::grid_utils::{tile_center_world, tile_floor_world};

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
pub fn spawn_background(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: COLOR_BACKGROUND,
            custom_size: Some(Vec2::new(SCREEN_WIDTH + 200.0, SCREEN_HEIGHT + 200.0)),
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
) {
    let grid_line_h_mesh = meshes.add(Rectangle::new(SCREEN_WIDTH + 100.0, 1.5));
    let grid_line_v_mesh = meshes.add(Rectangle::new(1.5, SCREEN_HEIGHT + 100.0));
    let grid_line_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE));
    let grid_line_bright_mat = materials.add(ColorMaterial::from(COLOR_GRID_LINE_BRIGHT));

    // Horizontal lines
    for i in -10..=10 {
        let y = i as f32 * 60.0 + ARENA_Y_OFFSET;
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
            Transform::from_xyz(x, ARENA_Y_OFFSET, Z_GRID_LINES),
            CleanupOnStateExit(GameState::Playing),
        ));
    }
}

/// Spawns all MMBN-style tile panels for the arena grid
pub fn spawn_tile_panels(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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

    // Spawn grid panels (MMBN style)
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            spawn_single_tile_panel(
                commands,
                materials,
                x,
                y,
                panel_w,
                panel_h,
                inner_w,
                inner_h,
                &outer_frame_mesh,
                &inner_frame_mesh,
                &inner_panel_mesh,
                &highlight_mesh,
                &front_face_mesh,
                &player_outer_mat,
                &player_frame_mat,
                &player_front_mat,
                &enemy_outer_mat,
                &enemy_frame_mat,
                &enemy_front_mat,
                &highlight_mat,
            );
        }
    }
}

/// Spawns a single tile panel with all its visual layers
#[allow(clippy::too_many_arguments)]
fn spawn_single_tile_panel(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: i32,
    y: i32,
    _panel_w: f32,
    panel_h: f32,
    _inner_w: f32,
    inner_h: f32,
    outer_frame_mesh: &Handle<Mesh>,
    inner_frame_mesh: &Handle<Mesh>,
    inner_panel_mesh: &Handle<Mesh>,
    highlight_mesh: &Handle<Mesh>,
    front_face_mesh: &Handle<Mesh>,
    player_outer_mat: &Handle<ColorMaterial>,
    player_frame_mat: &Handle<ColorMaterial>,
    player_front_mat: &Handle<ColorMaterial>,
    enemy_outer_mat: &Handle<ColorMaterial>,
    enemy_frame_mat: &Handle<ColorMaterial>,
    enemy_front_mat: &Handle<ColorMaterial>,
    highlight_mat: &Handle<ColorMaterial>,
) {
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
) {
    spawn_background(commands);
    spawn_grid_lines(commands, meshes, materials);
    spawn_tile_panels(commands, meshes, materials);
}
