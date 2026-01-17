use crate::components::{
    BaseColor, Bullet, ChargedShot, Enemy, EnemyBullet, FlashTimer, GameState, GridPosition,
    Health, HealthText, Lifetime, MoveTimer, MuzzleFlash, Player, PlayerHealthText, TargetsTiles,
    TileAssets, TileHighlightState, TilePanel,
};
use crate::constants::*;
use crate::resources::{GameProgress, PlayerCurrency, WaveState};

/// Speed of highlight fade in/out (intensity units per second)
const HIGHLIGHT_FADE_SPEED: f32 = 8.0;
use crate::weapons::Projectile;
use bevy::prelude::*;

/// Player bullets move right
pub fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut GridPosition, &mut MoveTimer),
        (With<Bullet>, Without<EnemyBullet>),
    >,
) {
    for (entity, mut pos, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            pos.x += 1;
            if pos.x >= GRID_WIDTH {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Enemy bullets move left
pub fn enemy_bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GridPosition, &mut MoveTimer), With<EnemyBullet>>,
) {
    for (entity, mut pos, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            pos.x -= 1;
            if pos.x < 0 {
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

/// Legacy bullet hit system - only for non-weapon bullets (e.g., old system bullets without Projectile)
/// Player projectiles with the Projectile component are handled by weapon::projectile_hit_system
pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<
        (Entity, &GridPosition),
        (
            With<Bullet>,
            Without<EnemyBullet>,
            Without<ChargedShot>,
            Without<Projectile>,
        ),
    >,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    for (bullet_entity, bullet_pos) in &bullet_query {
        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            if bullet_pos == enemy_pos {
                health.current -= PLAYER_DAMAGE;
                commands.entity(bullet_entity).despawn();

                // Update HP text children (shadow + main)
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = health.current.max(0).to_string();
                    }
                }

                if health.current <= 0 {
                    // Despawn enemy and all children (despawn is recursive in Bevy 0.17+)
                    commands.entity(enemy_entity).despawn();
                } else {
                    // Flash feedback only if still alive
                    commands
                        .entity(enemy_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }
            }
        }
    }
}

/// Enemy bullets hit player
pub fn enemy_bullet_hit_player(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GridPosition), With<EnemyBullet>>,
    mut player_query: Query<(Entity, &GridPosition, &mut Health), With<Player>>,
    mut hp_text_query: Query<&mut Text2d, With<PlayerHealthText>>,
) {
    for (bullet_entity, bullet_pos) in &bullet_query {
        for (player_entity, player_pos, mut health) in &mut player_query {
            if bullet_pos == player_pos {
                health.current -= ENEMY_DAMAGE;
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
    mut wave_state: ResMut<WaveState>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut currency: ResMut<PlayerCurrency>,
    mut progress: ResMut<GameProgress>,
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

        // Go to shop
        next_state.set(GameState::Shop);
    }
}
