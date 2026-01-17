use bevy::image::TextureAtlas;
use bevy::prelude::*;

use crate::assets::{FighterSprites, SlimeSprites};
use crate::components::{
    Enemy, FighterAnim, FighterAnimState, FlashTimer, Player, SlimeAnim, SlimeAnimState,
};
use crate::enemies::ChargingTelegraph;
use crate::systems::actions::HealFlashTimer;

fn movement_pressed(keys: &ButtonInput<KeyCode>) -> bool {
    keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::KeyS)
        || keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::ArrowUp)
        || keys.pressed(KeyCode::ArrowLeft)
        || keys.pressed(KeyCode::ArrowDown)
        || keys.pressed(KeyCode::ArrowRight)
}

fn animation_for_state(sprites: &FighterSprites, state: FighterAnimState) -> Handle<Image> {
    match state {
        FighterAnimState::Idle => sprites.idle.clone(),
        FighterAnimState::Walk => sprites.walk.clone(),
        FighterAnimState::Shoot => sprites.shoot.clone(),
    }
}

fn frames_for_state(sprites: &FighterSprites, state: FighterAnimState) -> usize {
    match state {
        FighterAnimState::Idle => sprites.idle_frames,
        FighterAnimState::Walk => sprites.walk_frames,
        FighterAnimState::Shoot => sprites.shoot_frames,
    }
}

fn fps_for_state(state: FighterAnimState) -> f32 {
    match state {
        FighterAnimState::Idle => 10.0,
        FighterAnimState::Walk => 14.0,
        FighterAnimState::Shoot => 18.0,
    }
}

pub fn animate_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    sprites: Option<Res<FighterSprites>>,
    mut query: Query<(&mut Sprite, &mut FighterAnim), (With<Player>, Without<HealFlashTimer>)>,
) {
    let Some(sprites) = sprites else {
        return;
    };

    let desired_state = if keys.pressed(KeyCode::Space) {
        FighterAnimState::Shoot
    } else if movement_pressed(&keys) {
        FighterAnimState::Walk
    } else {
        FighterAnimState::Idle
    };

    for (mut sprite, mut anim) in &mut query {
        if anim.state != desired_state {
            anim.state = desired_state;
            anim.frame = 0;
            anim.timer =
                Timer::from_seconds(1.0 / fps_for_state(desired_state), TimerMode::Repeating);

            sprite.image = animation_for_state(&sprites, desired_state);
            sprite.texture_atlas = Some(TextureAtlas {
                layout: sprites.layout.clone(),
                index: 0,
            });
        }

        anim.timer.tick(time.delta());
        if anim.timer.is_finished() {
            let frame_count = frames_for_state(&sprites, anim.state);
            anim.frame = (anim.frame + 1) % frame_count;
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = anim.frame;
            }
        }
    }
}

// ============================================================================
// Slime Animation
// ============================================================================

fn slime_animation_for_state(sprites: &SlimeSprites, state: SlimeAnimState) -> Handle<Image> {
    match state {
        SlimeAnimState::Idle => sprites.idle.clone(),
        SlimeAnimState::Shoot => sprites.shoot.clone(),
        SlimeAnimState::Dead => sprites.dead.clone(),
    }
}

fn slime_layout_for_state(
    sprites: &SlimeSprites,
    state: SlimeAnimState,
) -> Handle<TextureAtlasLayout> {
    match state {
        SlimeAnimState::Idle | SlimeAnimState::Dead => sprites.layout.clone(),
        SlimeAnimState::Shoot => sprites.shoot_layout.clone(),
    }
}

fn slime_frames_for_state(sprites: &SlimeSprites, state: SlimeAnimState) -> usize {
    match state {
        SlimeAnimState::Idle => sprites.idle_frames,
        SlimeAnimState::Shoot => sprites.shoot_frames,
        SlimeAnimState::Dead => sprites.dead_frames,
    }
}

fn slime_fps_for_state(state: SlimeAnimState) -> f32 {
    match state {
        SlimeAnimState::Idle => 8.0,
        SlimeAnimState::Shoot => 12.0,
        SlimeAnimState::Dead => 10.0,
    }
}

pub fn animate_slime(
    time: Res<Time>,
    sprites: Option<Res<SlimeSprites>>,
    mut query: Query<
        (&mut Sprite, &mut SlimeAnim),
        (With<Enemy>, Without<ChargingTelegraph>, Without<FlashTimer>),
    >,
) {
    let Some(sprites) = sprites else {
        return;
    };

    for (mut sprite, mut anim) in &mut query {
        // For now, slime just idles. State changes can be triggered by combat system later.
        let desired_state = anim.state; // Keep current state

        if anim.state != desired_state {
            anim.state = desired_state;
            anim.frame = 0;
            anim.timer = Timer::from_seconds(
                1.0 / slime_fps_for_state(desired_state),
                TimerMode::Repeating,
            );

            sprite.image = slime_animation_for_state(&sprites, desired_state);
            sprite.texture_atlas = Some(TextureAtlas {
                layout: slime_layout_for_state(&sprites, desired_state),
                index: 0,
            });
        }

        anim.timer.tick(time.delta());
        if anim.timer.is_finished() {
            let frame_count = slime_frames_for_state(&sprites, anim.state);
            anim.frame = (anim.frame + 1) % frame_count;
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = anim.frame;
            }
        }
    }
}
