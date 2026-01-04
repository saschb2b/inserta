use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GridPosition, &mut MoveTimer), With<Bullet>>,
) {
    for (entity, mut pos, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            pos.x += 1;
            if pos.x >= GRID_WIDTH {
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
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity, &GridPosition), With<Bullet>>,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    // Tile-based collision: a hit occurs if a bullet occupies the same tile as the enemy.
    for (bullet_entity, bullet_pos) in &bullet_query {
        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            if bullet_pos == enemy_pos {
                health.current -= 1;
                commands.entity(bullet_entity).despawn();

                // Update HP text children (shadow + main)
                for i in 0..children.len() {
                    let child = children[i];
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

pub fn enemy_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Sprite, &BaseColor, &mut FlashTimer), With<Enemy>>,
) {
    for (entity, mut sprite, base, mut flash) in &mut query {
        flash.0.tick(time.delta());

        if flash.0.finished() {
            sprite.color = base.0;
            commands.entity(entity).remove::<FlashTimer>();
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
