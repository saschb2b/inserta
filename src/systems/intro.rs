// ============================================================================
// Pre-Battle Intro Systems
// ============================================================================

use bevy::prelude::*;

use crate::components::{CountdownText, FadeOverlay, IntroPhase, Player, PreBattleIntro};
use crate::constants::Z_UI;

// Timing constants (in seconds)
const FADE_DURATION: f32 = 0.15;
const DROP_IN_START: f32 = 0.15;
const DROP_IN_DURATION: f32 = 0.35;
const COUNTDOWN_3_START: f32 = 0.5;
const COUNTDOWN_2_START: f32 = 0.7;
const COUNTDOWN_1_START: f32 = 0.9;
const ENGAGE_START: f32 = 1.1;
const COMPLETE_TIME: f32 = 1.3;

/// Setup the pre-battle intro (spawn overlay, countdown text)
pub fn setup_intro(mut commands: Commands) {
    // Initialize the intro resource
    commands.insert_resource(PreBattleIntro::default());

    // Spawn black fade overlay (full screen)
    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(2000.0, 2000.0)), // Cover entire screen
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_UI + 100.0), // Above everything
        FadeOverlay,
    ));

    // Spawn countdown text (initially invisible)
    commands.spawn((
        Text2d::new(""),
        TextFont::from_font_size(120.0),
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, Z_UI + 101.0), // Above overlay
        CountdownText,
    ));
}

/// Update the pre-battle intro sequence
pub fn update_intro(
    mut commands: Commands,
    time: Res<Time>,
    mut intro: ResMut<PreBattleIntro>,
    mut overlay_query: Query<(Entity, &mut Sprite), With<FadeOverlay>>,
    mut text_query: Query<
        (Entity, &mut Text2d, &mut TextColor, &mut Transform),
        With<CountdownText>,
    >,
    mut player_query: Query<&mut Transform, (With<Player>, Without<CountdownText>)>,
) {
    intro.elapsed += time.delta_secs();

    // Update phase based on elapsed time
    let new_phase = if intro.elapsed < FADE_DURATION {
        IntroPhase::FadeIn
    } else if intro.elapsed < COUNTDOWN_3_START {
        IntroPhase::DropIn
    } else if intro.elapsed < COUNTDOWN_2_START {
        IntroPhase::Countdown3
    } else if intro.elapsed < COUNTDOWN_1_START {
        IntroPhase::Countdown2
    } else if intro.elapsed < ENGAGE_START {
        IntroPhase::Countdown1
    } else if intro.elapsed < COMPLETE_TIME {
        IntroPhase::Engage
    } else {
        IntroPhase::Complete
    };

    intro.phase = new_phase;

    // Handle fade overlay
    for (entity, mut sprite) in &mut overlay_query {
        if intro.phase == IntroPhase::FadeIn {
            // Fade from black to transparent
            let progress = intro.elapsed / FADE_DURATION;
            let alpha = 1.0 - progress.min(1.0);
            sprite.color = Color::srgba(0.0, 0.0, 0.0, alpha);
        } else if intro.phase != IntroPhase::Complete {
            // Keep transparent during countdown
            sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
        } else {
            // Despawn overlay when complete
            commands.entity(entity).despawn();
        }
    }

    // Handle countdown text
    for (entity, mut text, mut color, mut transform) in &mut text_query {
        match intro.phase {
            IntroPhase::Countdown3 => {
                text.0 = "3".to_string();
                color.0 = Color::WHITE;
                // Pulse effect: scale based on phase progress
                let phase_progress =
                    (intro.elapsed - COUNTDOWN_3_START) / (COUNTDOWN_2_START - COUNTDOWN_3_START);
                let scale = 1.0 + (1.0 - phase_progress) * 0.3; // Start big, shrink
                transform.scale = Vec3::splat(scale);
            }
            IntroPhase::Countdown2 => {
                text.0 = "2".to_string();
                color.0 = Color::WHITE;
                let phase_progress =
                    (intro.elapsed - COUNTDOWN_2_START) / (COUNTDOWN_1_START - COUNTDOWN_2_START);
                let scale = 1.0 + (1.0 - phase_progress) * 0.3;
                transform.scale = Vec3::splat(scale);
            }
            IntroPhase::Countdown1 => {
                text.0 = "1".to_string();
                color.0 = Color::WHITE;
                let phase_progress =
                    (intro.elapsed - COUNTDOWN_1_START) / (ENGAGE_START - COUNTDOWN_1_START);
                let scale = 1.0 + (1.0 - phase_progress) * 0.3;
                transform.scale = Vec3::splat(scale);
            }
            IntroPhase::Engage => {
                text.0 = "ENGAGE!".to_string();
                color.0 = Color::srgb(1.0, 0.9, 0.2); // Yellow/gold
                let phase_progress =
                    (intro.elapsed - ENGAGE_START) / (COMPLETE_TIME - ENGAGE_START);
                let scale = 1.0 + (1.0 - phase_progress) * 0.2;
                transform.scale = Vec3::splat(scale);
            }
            IntroPhase::Complete => {
                // Despawn text when complete
                commands.entity(entity).despawn();
            }
            _ => {
                text.0 = "".to_string();
            }
        }
    }

    // Handle player drop-in animation
    if intro.phase == IntroPhase::DropIn || intro.phase == IntroPhase::FadeIn {
        for mut transform in &mut player_query {
            if intro.elapsed < DROP_IN_START {
                // Player not visible yet (scale 0)
                transform.scale = Vec3::splat(0.0);
            } else {
                // Animate scale from 0.0 to 1.0
                let drop_progress = ((intro.elapsed - DROP_IN_START) / DROP_IN_DURATION).min(1.0);
                // Ease out curve for bounce effect
                let eased = 1.0 - (1.0 - drop_progress).powi(2);
                transform.scale = Vec3::splat(eased);
            }
        }
    } else if intro.phase == IntroPhase::Countdown3 {
        // Ensure player is fully visible after drop-in
        for mut transform in &mut player_query {
            transform.scale = Vec3::splat(1.0);
        }
    }

    // Unlock input when complete
    if intro.phase == IntroPhase::Complete && !intro.input_unlocked {
        intro.input_unlocked = true;
    }
}

/// Cleanup intro resources when leaving Playing state
pub fn cleanup_intro(mut commands: Commands) {
    commands.remove_resource::<PreBattleIntro>();
}

/// Run condition: only run if intro is complete (input unlocked)
pub fn intro_complete(intro: Option<Res<PreBattleIntro>>) -> bool {
    intro.map(|i| i.is_complete()).unwrap_or(true)
}
