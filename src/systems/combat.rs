use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

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

/// Player bullets hit enemies
pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GridPosition), (With<Bullet>, Without<EnemyBullet>)>,
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
                        text.0 = health.current.to_string();
                    }
                }

                // Flash feedback
                commands
                    .entity(enemy_entity)
                    .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
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

                // Flash feedback
                commands
                    .entity(player_entity)
                    .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));

                if health.current <= 0 {
                    // Player defeated - could trigger game over
                    commands.entity(player_entity).despawn();
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

/// Highlights tiles that have bullets on them
pub fn bullet_tile_highlight(
    player_bullet_query: Query<&GridPosition, (With<Bullet>, Without<EnemyBullet>)>,
    enemy_bullet_query: Query<&GridPosition, With<EnemyBullet>>,
    mut tile_query: Query<(&TilePanel, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Collect all tile positions that have bullets (both player and enemy)
    let mut bullet_positions: Vec<(i32, i32)> = player_bullet_query
        .iter()
        .map(|pos| (pos.x, pos.y))
        .collect();
    bullet_positions.extend(enemy_bullet_query.iter().map(|pos| (pos.x, pos.y)));

    // Update tile materials based on bullet presence
    for (tile, material_handle) in &mut tile_query {
        let has_bullet = bullet_positions.contains(&(tile.x, tile.y));

        if let Some(material) = materials.get_mut(&material_handle.0) {
            let is_player_side = tile.x < PLAYER_AREA_WIDTH;
            let base_color = if is_player_side {
                COLOR_PLAYER_PANEL_TOP
            } else {
                COLOR_ENEMY_PANEL_TOP
            };

            if has_bullet {
                // Blend with yellow highlight
                material.color = blend_colors(base_color, COLOR_BULLET_HIGHLIGHT, 0.6);
            } else {
                // Restore base color
                material.color = base_color;
            }
        }
    }
}

/// Blends two colors together
fn blend_colors(base: Color, overlay: Color, amount: f32) -> Color {
    let base_srgba = base.to_srgba();
    let overlay_srgba = overlay.to_srgba();

    Color::srgba(
        base_srgba.red + (overlay_srgba.red - base_srgba.red) * amount * overlay_srgba.alpha,
        base_srgba.green + (overlay_srgba.green - base_srgba.green) * amount * overlay_srgba.alpha,
        base_srgba.blue + (overlay_srgba.blue - base_srgba.blue) * amount * overlay_srgba.alpha,
        base_srgba.alpha,
    )
}
