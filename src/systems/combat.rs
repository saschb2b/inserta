use crate::components::{
    BaseColor, Bullet, DefeatOutro, Enemy, EnemyBullet, FlashTimer, GridPosition, Health, Lifetime,
    MoveTimer, MuzzleFlash, Player, PlayerHealthText, TargetsTiles, TileAssets, TileHighlightState,
    TilePanel, VictoryOutro,
};
use crate::constants::*;
use crate::resources::{BattleTimer, GameProgress, PlayerCurrency, WaveState};

/// Speed of highlight fade in/out (intensity units per second)
const HIGHLIGHT_FADE_SPEED: f32 = 8.0;
use crate::assets::{ProjectileAnimation, ProjectileSprites};
use bevy::image::TextureAtlas;
use bevy::prelude::*;

/// Player bullets move right
pub fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut GridPosition, &mut MoveTimer),
        (
            With<Bullet>,
            Without<EnemyBullet>,
            Without<crate::components::ProjectileImmobile>,
        ),
    >,
) {
    for (entity, mut pos, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            pos.x += 1;
            if pos.x >= GRID_WIDTH {
                // Despawn off-screen projectiles (but not hit projectiles in animation)
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Enemy bullets move left
pub fn enemy_bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut GridPosition, &mut MoveTimer),
        (
            With<EnemyBullet>,
            Without<crate::components::ProjectileImmobile>,
        ),
    >,
) {
    for (entity, mut pos, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            pos.x -= 1;
            if pos.x < 0 {
                // Despawn off-screen projectiles (but not hit projectiles in animation)
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn muzzle_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime), With<MuzzleFlash>>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.0.tick(time.delta());
        if lifetime.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Enemy bullets hit player
pub fn enemy_bullet_hit_player(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GridPosition, &EnemyBullet)>,
    mut player_query: Query<(Entity, &GridPosition, &mut Health), With<Player>>,
    mut hp_text_query: Query<&mut Text2d, With<PlayerHealthText>>,
) {
    for (bullet_entity, bullet_pos, enemy_bullet) in &bullet_query {
        for (player_entity, player_pos, mut health) in &mut player_query {
            if bullet_pos == player_pos {
                // Use damage from the bullet (defined in enemy blueprint)
                health.current -= enemy_bullet.damage;
                commands.entity(bullet_entity).despawn();

                // Update player HP text
                for mut text in &mut hp_text_query {
                    text.0 = format!("HP: {}", health.current.max(0));
                }

                if health.current <= 0 {
                    // Player defeated - could trigger game over
                    commands.entity(player_entity).despawn();
                } else {
                    // Flash feedback only if still alive
                    commands
                        .entity(player_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }
            }
        }
    }
}

/// Flash effect for any entity with FlashTimer
pub fn entity_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &BaseColor, &mut FlashTimer)>,
) {
    for (entity, mut sprite, base, mut flash) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.is_finished() {
            sprite.color = base.0;
            commands.entity(entity).remove::<FlashTimer>();
        } else {
            sprite.color = Color::srgb(1.0, 0.3, 0.3); // Red flash for damage
        }
    }
}

/// Highlights tiles that are being targeted by attacks with smooth fade transitions
///
/// This system:
/// 1. Collects all targeted tiles from entities with TargetsTiles component
/// 2. Sets target highlight intensity based on tile being targeted
/// 3. Smoothly transitions intensity toward target
/// 4. Swaps between normal/highlighted textures based on intensity
pub fn tile_attack_highlight(
    time: Res<Time>,
    tile_assets: Option<Res<TileAssets>>,
    targeting_query: Query<(&TargetsTiles, Option<&GridPosition>)>,
    mut tile_query: Query<(&TilePanel, &mut TileHighlightState, &mut Sprite)>,
) {
    // Skip if tile assets aren't loaded yet
    let Some(assets) = tile_assets else {
        return;
    };

    // Collect all targeted tile positions from entities with TargetsTiles
    let mut targeted_positions: Vec<(i32, i32)> = Vec::new();

    for (targets, grid_pos) in &targeting_query {
        if targets.use_grid_position {
            // Use entity's GridPosition (for bullets, single-target attacks)
            if let Some(pos) = grid_pos {
                targeted_positions.push((pos.x, pos.y));
            }
        } else {
            // Use explicit tiles list (for multi-tile attacks like WideSword)
            targeted_positions.extend(targets.tiles.iter().copied());
        }
    }

    let dt = time.delta_secs();

    // Update each tile's highlight state and texture
    for (tile, mut highlight, mut sprite) in &mut tile_query {
        let is_targeted = targeted_positions.contains(&(tile.x, tile.y));

        // Set target based on whether tile is being attacked
        highlight.target = if is_targeted { 1.0 } else { 0.0 };

        // Smoothly transition intensity toward target
        if highlight.intensity != highlight.target {
            let direction = if highlight.target > highlight.intensity {
                1.0
            } else {
                -1.0
            };
            highlight.intensity += direction * HIGHLIGHT_FADE_SPEED * dt;
            highlight.intensity = highlight.intensity.clamp(0.0, 1.0);

            // Snap to target if close enough
            if (highlight.intensity - highlight.target).abs() < 0.01 {
                highlight.intensity = highlight.target;
            }
        }

        // Choose texture based on intensity threshold (swap at 50%)
        let use_highlighted = highlight.intensity > 0.5;

        let (normal_tex, highlighted_tex) = if highlight.is_player_side {
            (&assets.red_normal, &assets.red_highlighted)
        } else {
            (&assets.blue_normal, &assets.blue_highlighted)
        };

        let desired_texture = if use_highlighted {
            highlighted_tex
        } else {
            normal_tex
        };

        // Only update if texture changed
        if sprite.image != *desired_texture {
            sprite.image = desired_texture.clone();
        }

        // Apply fade effect via alpha for smooth transition
        // When transitioning, fade alpha based on how close we are to the swap point
        let alpha = if highlight.intensity > 0.0 && highlight.intensity < 1.0 {
            // During transition: pulse slightly for visual interest
            if use_highlighted {
                // Fading in highlighted: start dimmer, get brighter
                0.7 + 0.3 * (highlight.intensity - 0.5).abs() * 2.0
            } else {
                // Fading out to normal: start slightly dim at boundary
                0.85 + 0.15 * (0.5 - highlight.intensity).max(0.0) * 2.0
            }
        } else {
            1.0
        };

        sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

// ============================================================================
// Game Loop Systems
// ============================================================================

/// Transition wave state from Spawning to Active once enemies exist
pub fn update_wave_state(
    mut wave_state: ResMut<WaveState>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if *wave_state == WaveState::Spawning && !enemy_query.is_empty() {
        *wave_state = WaveState::Active;
        info!("Wave Active!");
    }
}

/// Check if all enemies are defeated to win the wave
pub fn check_victory_condition(
    mut commands: Commands,
    mut wave_state: ResMut<WaveState>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut currency: ResMut<PlayerCurrency>,
    mut progress: ResMut<GameProgress>,
    battle_timer: Res<BattleTimer>,
) {
    if *wave_state == WaveState::Active && enemy_query.is_empty() {
        // Victory!
        *wave_state = WaveState::Cleared;

        // Award currency (base + scaling)
        let reward = 100 + (progress.current_level as u64 * 50);
        currency.zenny += reward;
        info!("Wave Cleared! Reward: {} Zenny", reward);

        // Advance level
        progress.next_level();

        // Trigger the victory outro instead of immediate state transition
        // The outro system will detect this resource and set up the UI
        commands.insert_resource(VictoryOutro::new(battle_timer.elapsed, reward));
    }
}

// ============================================================================
// Projectile Animation System
// ============================================================================

/// Animate projectiles based on their state (launch, travel, impact, finish)
pub fn projectile_animation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut ProjectileAnimation), With<Bullet>>,
    projectiles: Option<Res<ProjectileSprites>>,
    time: Res<Time>,
) {
    let Some(projectiles) = projectiles else {
        return;
    };

    for (entity, mut sprite, mut anim) in &mut query {
        // Transition from Launch to Travel immediately (launch frame is just visual startup)
        if anim.state == crate::assets::ProjectileAnimationState::Launch {
            anim.state = crate::assets::ProjectileAnimationState::Travel;
            anim.timer = Timer::from_seconds(0.0, TimerMode::Once);
        }

        // Update timer for state transitions
        anim.timer.tick(time.delta());

        // Transition to Finish if we've shown Impact long enough
        if anim.state == crate::assets::ProjectileAnimationState::Impact && anim.timer.is_finished()
        {
            anim.state = crate::assets::ProjectileAnimationState::Finish;
            anim.timer = Timer::from_seconds(0.1, TimerMode::Once); // Brief show of finish frame
        }

        // Despawn after Finish state animation completes
        if anim.state == crate::assets::ProjectileAnimationState::Finish && anim.timer.is_finished()
        {
            commands.entity(entity).despawn();
            continue;
        }

        // Get the current frame index based on animation state
        let frame_index = anim.frame_indices[anim.state as usize];

        // Set the sprite to show the correct frame from the 4-frame spritesheet
        // Choose between normal and charged sprite based on projectile type
        let (sprite_image, sprite_layout) = if anim.is_charged {
            (
                projectiles.blaster_charged_image.clone(),
                projectiles.blaster_charged_layout.clone(),
            )
        } else {
            (
                projectiles.blaster_image.clone(),
                projectiles.blaster_layout.clone(),
            )
        };

        sprite.image = sprite_image;
        sprite.custom_size = Some(BULLET_DRAW_SIZE);
        sprite.texture_atlas = Some(TextureAtlas {
            layout: sprite_layout,
            index: frame_index,
        });
    }
}

/// Check if player is defeated to trigger game over
pub fn check_defeat_condition(
    mut commands: Commands,
    mut wave_state: ResMut<WaveState>,
    player_query: Query<&Health, With<Player>>,
    battle_timer: Res<BattleTimer>,
) {
    // Only check during active battle
    if *wave_state != WaveState::Active {
        return;
    }

    // Check if player is dead (entity still exists but HP <= 0) or player entity is gone
    let player_dead = player_query
        .iter()
        .next()
        .map(|h| h.current <= 0)
        .unwrap_or(true); // No player entity = dead

    if player_dead {
        // Defeat!
        *wave_state = WaveState::Cleared; // Reuse Cleared state to stop gameplay

        info!("Player Defeated! No reward earned.");

        // Trigger the defeat outro
        commands.insert_resource(DefeatOutro::new(battle_timer.elapsed));
    }
}
