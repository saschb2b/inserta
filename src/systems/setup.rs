use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::{Anchor, MeshMaterial2d};

use crate::assets::FighterSprites;
use crate::components::*;
use crate::constants::*;
use crate::systems::grid_utils::{tile_center_world, tile_floor_world};

fn trapezoid_mesh(w: f32, h: f32, top_inset: f32, top_skew: f32) -> Mesh {
    let half_w = w / 2.0;
    let half_h = h / 2.0;

    let top_w = (w - top_inset * 2.0).max(1.0);
    let top_half_w = top_w / 2.0;

    let positions = vec![
        [-half_w, -half_h, 0.0],
        [half_w, -half_h, 0.0],
        [top_half_w + top_skew, half_h, 0.0],
        [-top_half_w + top_skew, half_h, 0.0],
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Background
    commands.spawn((
        Sprite {
            color: COLOR_BACKGROUND,
            custom_size: Some(Vec2::new(2000.0, 1200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_BACKGROUND),
    ));

    // BGM
    let bgm: Handle<AudioSource> = asset_server.load("audio/bgm/battle.mp3");
    commands.spawn((
        AudioPlayer::new(bgm),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.45)),
    ));

    // Shared meshes
    let shadow_mesh = meshes.add(trapezoid_mesh(
        TILE_W + 8.0,
        TILE_H + 8.0,
        TILE_TOP_INSET + 6.0,
        8.0,
    ));
    let border_mesh = meshes.add(trapezoid_mesh(
        TILE_W + 4.0,
        TILE_H + 4.0,
        TILE_TOP_INSET + 4.0,
        8.0,
    ));
    let panel_mesh = meshes.add(trapezoid_mesh(TILE_W, TILE_H, TILE_TOP_INSET, 8.0));
    let shine_mesh = meshes.add(trapezoid_mesh(
        TILE_W * 0.92,
        TILE_H * 0.25,
        TILE_TOP_INSET * 0.8,
        6.0,
    ));

    // Shared materials
    let shadow_mat = materials.add(ColorMaterial::from(COLOR_PANEL_SHADOW));
    let border_mat = materials.add(ColorMaterial::from(COLOR_PANEL_BORDER));
    let player_mat = materials.add(ColorMaterial::from(COLOR_PLAYER_PANEL));
    let enemy_mat = materials.add(ColorMaterial::from(COLOR_ENEMY_PANEL));
    let shine_mat = materials.add(ColorMaterial::from(COLOR_PANEL_SHINE));

    // Grid
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let is_player_territory = x < PLAYER_AREA_WIDTH;
            let panel_mat = if is_player_territory {
                player_mat.clone()
            } else {
                enemy_mat.clone()
            };
            let floor = tile_floor_world(x, y);
            let world = tile_center_world(x, y);

            commands.spawn((
                Mesh2d(shadow_mesh.clone()),
                MeshMaterial2d(shadow_mat.clone()),
                Transform::from_xyz(
                    world.x + 3.0,
                    world.y - 3.0,
                    Z_GRID_SHADOW - floor.y * DEPTH_Y_TO_Z,
                ),
            ));

            commands.spawn((
                Mesh2d(border_mesh.clone()),
                MeshMaterial2d(border_mat.clone()),
                Transform::from_xyz(world.x, world.y, Z_GRID - floor.y * DEPTH_Y_TO_Z),
            ));

            commands.spawn((
                Mesh2d(panel_mesh.clone()),
                MeshMaterial2d(panel_mat),
                Transform::from_xyz(world.x, world.y, (Z_GRID + 0.1) - floor.y * DEPTH_Y_TO_Z),
            ));

            commands.spawn((
                Mesh2d(shine_mesh.clone()),
                MeshMaterial2d(shine_mat.clone()),
                Transform::from_xyz(
                    world.x - 8.0,
                    world.y + (TILE_H * 0.18),
                    (Z_GRID + 0.2) - floor.y * DEPTH_Y_TO_Z,
                ),
            ));
        }
    }

    // Fighter sprite sheets
    // These are 1280x128 (10 frames of 128x128).
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
        frames: 10,
    });

    // Player
    commands.spawn((
        Sprite {
            image: fighter_idle,
            texture_atlas: Some(fighter_layout.into()),
            color: Color::WHITE,
            anchor: Anchor::Custom(FIGHTER_ANCHOR),
            custom_size: Some(FIGHTER_DRAW_SIZE),
            ..default()
        },
        // Keep scale at 1.0; custom_size drives size.
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

    // Enemy
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
                custom_size: Some(Vec2::new(44.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 96.0, 0.0),
        ));

        // Shadow
        parent.spawn((
            Text2d::new("100"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(COLOR_TEXT_SHADOW),
            Transform::from_xyz(1.5, 94.5, 0.1),
            HealthText,
        ));

        // Main
        parent.spawn((
            Text2d::new("100"),
            TextLayout::new_with_justify(JustifyText::Center),
            TextColor(COLOR_TEXT),
            Transform::from_xyz(0.0, 96.0, 0.2),
            HealthText,
        ));
    });
}
