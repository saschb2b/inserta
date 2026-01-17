use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::image::TextureAtlas;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::Justify;

use crate::assets::{FighterSprites, ProjectileSprites};
use crate::components::{
    ActionBar, ActionChargeBar, ActionCooldownOverlay, ActionKeyText, ActionSlot, ActionSlotUI,
    ActionType, ArenaConfig, BaseColor, CleanupOnStateExit, Enemy, EnemyConfig, FighterAnim,
    FighterAnimState, GameState, GridPosition, Health, HealthText, Player, PlayerHealthText,
    RenderConfig, SlimeAnim, SlimeAnimState,
};
use crate::constants::*;
use crate::enemies::{
    BehaviorEnemy, EnemyAnimState, EnemyAttack, EnemyBlueprint, EnemyMovement, EnemyStats,
    EnemyTraitContainer,
};
use crate::resources::{ArenaLayout, PlayerUpgrades, WaveState};
use crate::systems::arena::spawn_arena_visuals;
use crate::weapons::{EquippedWeapon, WeaponState, WeaponType};

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
    upgrades: Res<PlayerUpgrades>,
    mut wave_state: ResMut<WaveState>,
    windows: Query<&Window>,
) {
    *wave_state = WaveState::Spawning;

    // ========================================================================
    // Compute Arena Layout from window size
    // ========================================================================
    let layout = windows
        .iter()
        .next()
        .map(|window| ArenaLayout::from_screen_size(window.width(), window.height()))
        .unwrap_or_default();
    commands.insert_resource(layout.clone());

    // ========================================================================
    // Arena Visuals (background, grid lines, tile panels)
    // ========================================================================
    spawn_arena_visuals(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &layout,
    );

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

    // Create equipped weapon and its state
    let mut equipped_weapon = EquippedWeapon::new(WeaponType::Blaster);
    equipped_weapon.stats.apply_upgrades(&upgrades);

    let weapon_state = WeaponState::new(equipped_weapon.stats.fire_cooldown);

    let max_hp = upgrades.get_max_hp();

    commands.spawn((
        Sprite {
            image: fighter_idle,
            texture_atlas: Some(fighter_layout.into()),
            color: Color::WHITE,
            custom_size: Some(layout.scale_vec2(FIGHTER_DRAW_SIZE)),
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
            current: max_hp,
            max: max_hp,
        },
        BaseColor(Color::WHITE),
        // Weapon system components
        equipped_weapon,
        weapon_state,
        CleanupOnStateExit(GameState::Playing),
    ));

    // Player HP display (top-left area, above arena)
    commands.spawn((
        Text2d::new(format!("HP: {}", max_hp)),
        TextLayout::new_with_justify(Justify::Left),
        TextFont::from_font_size(28.0),
        TextColor(COLOR_TEXT),
        Transform::from_xyz(-580.0, 360.0, Z_UI),
        PlayerHealthText,
        CleanupOnStateExit(GameState::Playing),
    ));

    // ========================================================================
    // Projectile sprites
    // ========================================================================
    let blaster_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16), // Each frame is 16x16
        4,                  // 4 columns (launch, travel, impact, finish)
        1,                  // 1 row
        None,
        None,
    ));

    let blaster_charged_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16), // Each frame is 16x16
        4,                  // 4 columns (launch, travel, impact, finish)
        1,                  // 1 row
        None,
        None,
    ));

    let blaster_projectile = asset_server.load("battle/attacks/projectile/blaster.png");
    let blaster_charged_projectile =
        asset_server.load("battle/attacks/projectile/blaster_charged.png");

    commands.insert_resource(ProjectileSprites {
        blaster_image: blaster_projectile,
        blaster_layout,
        blaster_charged_image: blaster_charged_projectile,
        blaster_charged_layout,
    });

    // ========================================================================
    // Enemies (from config) - using the new blueprint system
    // ========================================================================
    for enemy_config in &config.enemies {
        spawn_enemy(
            &mut commands,
            &asset_server,
            &mut atlas_layouts,
            enemy_config,
            0, // TODO: Pass wave level for HP scaling
            &layout,
        );
    }
}

/// Spawn an enemy using the blueprint system
/// This is the unified spawn function for all enemy types
fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
    config: &EnemyConfig,
    wave_level: i32,
    arena_layout: &ArenaLayout,
) {
    // Get the blueprint for this enemy type
    let blueprint = EnemyBlueprint::get(config.enemy_id);

    // Calculate HP (use override or scaled from blueprint)
    let hp = config
        .hp_override
        .unwrap_or_else(|| blueprint.scaled_hp(wave_level));

    // Get visuals from blueprint
    let visuals = &blueprint.visuals;
    let anims = &visuals.animations;

    // Load sprite from blueprint path
    let sprite_path = format!("{}/{}", visuals.sprite_path, anims.idle_file);
    let texture: Handle<Image> = asset_server.load(&sprite_path);

    // Create atlas layout from blueprint animation config
    let atlas_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16), // Frame size (TODO: make configurable in blueprint)
        anims.idle_grid.0,
        anims.idle_grid.1,
        None,
        None,
    ));

    // Calculate FPS from blueprint
    let frame_duration = 1.0 / anims.idle_fps;

    let enemy_entity = commands
        .spawn((
            // Sprite setup from blueprint visuals (scaled to arena)
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout,
                    index: 0,
                }),
                color: Color::WHITE,
                custom_size: Some(arena_layout.scale_vec2(visuals.draw_size)),
                flip_x: visuals.flip_x,
                ..default()
            },
            Anchor(visuals.anchor),
            Transform::default(),
            GridPosition {
                x: config.start_x,
                y: config.start_y,
            },
            RenderConfig {
                offset: visuals.offset,
                base_z: Z_CHARACTER,
            },
            // Legacy animation component (for backward compatibility)
            SlimeAnim {
                state: SlimeAnimState::Idle,
                frame: 0,
                timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            },
            // Core enemy markers
            Enemy,
            BehaviorEnemy, // Mark as using new behavior system
            Health {
                current: hp,
                max: hp,
            },
            BaseColor(Color::WHITE),
            CleanupOnStateExit(GameState::Playing),
        ))
        .id();

    // Add behavior components separately (to avoid tuple size limits)
    commands.entity(enemy_entity).insert((
        EnemyStats {
            base_hp: blueprint.stats.base_hp,
            contact_damage: blueprint.stats.contact_damage,
            move_speed: blueprint.stats.move_speed,
            attack_speed: blueprint.stats.attack_speed,
        },
        EnemyMovement::new(blueprint.movement.clone(), blueprint.stats.move_speed),
        EnemyAttack::new(blueprint.attack.clone(), blueprint.stats.attack_speed),
        EnemyTraitContainer::new(blueprint.traits.clone()),
        EnemyAnimState::default(),
    ));

    // Spawn HP display as children
    commands.entity(enemy_entity).with_children(|parent| {
        // HP plate background
        parent.spawn((
            Sprite {
                color: COLOR_HP_PLATE,
                custom_size: Some(Vec2::new(64.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 80.0, 0.0),
        ));

        // HP text shadow
        parent.spawn((
            Text2d::new(hp.to_string()),
            TextLayout::new_with_justify(Justify::Center),
            TextFont::from_font_size(20.0),
            TextColor(COLOR_TEXT_SHADOW),
            Transform::from_xyz(1.5, 78.5, 0.1),
            HealthText,
        ));

        // HP text
        parent.spawn((
            Text2d::new(hp.to_string()),
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

/// Cleanup for when leaving Campaign state
pub fn cleanup_campaign_entities(
    mut commands: Commands,
    query: Query<(Entity, &CleanupOnStateExit)>,
) {
    for (entity, scoped) in &query {
        if scoped.0 == GameState::Campaign {
            commands.entity(entity).despawn();
        }
    }
}
