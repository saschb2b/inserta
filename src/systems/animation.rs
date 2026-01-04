use bevy::image::TextureAtlas;
use bevy::prelude::*;

use crate::assets::FighterSprites;
use crate::components::{FighterAnim, FighterAnimState, Player};

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
    mut query: Query<(&mut Sprite, &mut FighterAnim), With<Player>>,
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
        if anim.timer.finished() {
            anim.frame = (anim.frame + 1) % sprites.frames;
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = anim.frame;
            }
        }
    }
}
