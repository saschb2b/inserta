// ============================================================================
// Post-Battle Outro Systems (Victory & Defeat)
// ============================================================================

use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;

use crate::components::{
    CleanupOnStateExit, DefeatContinueText, DefeatGameOverText, DefeatNoRewardText, DefeatOutro,
    DefeatPhase, DefeatStatsPanel, DefeatTimeText, GameState, OutroPhase, VictoryClearText,
    VictoryContinueText, VictoryOutro, VictoryRewardText, VictoryStatsPanel, VictoryTimeText,
};
use crate::constants::Z_UI;
use crate::resources::{CampaignProgress, SelectedBattle};

// Timing constants (in seconds)
const HITSTOP_DURATION: f32 = 0.1;
const CLEAR_START: f32 = 0.1;
const CLEAR_DURATION: f32 = 0.4;
const STATS_START: f32 = 0.5;
const STATS_DURATION: f32 = 1.0;
const WAIT_CONFIRM_START: f32 = 1.5;

/// Marker to track if outro UI has been spawned
#[derive(Component)]
struct OutroUISpawned;

// ============================================================================
// Setup System - Called when victory outro resource is added
// ============================================================================

/// Setup the victory outro UI elements (runs when VictoryOutro resource exists but UI not yet spawned)
pub fn setup_outro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    outro: Option<Res<VictoryOutro>>,
    existing_ui: Query<(), With<VictoryClearText>>,
) {
    // Only run if outro is active but UI not yet spawned
    if outro.is_none() || !existing_ui.is_empty() {
        return;
    }
    // Play victory sound with slight BGM ducking effect
    let victory_sound: Handle<AudioSource> = asset_server.load("audio/sound/victory.mp3");
    commands.spawn((
        AudioPlayer::new(victory_sound),
        PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
        CleanupOnStateExit(GameState::Playing),
    ));

    // Spawn "CLEAR!" text (starts invisible, will animate in)
    commands.spawn((
        Text2d::new("CLEAR!"),
        TextFont::from_font_size(100.0),
        TextColor(Color::srgba(1.0, 0.9, 0.2, 0.0)), // Start invisible
        Transform::from_xyz(0.0, 80.0, Z_UI + 50.0),
        VictoryClearText,
        CleanupOnStateExit(GameState::Playing),
    ));

    // Stats panel background
    commands
        .spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0), // Start invisible
                custom_size: Some(Vec2::new(400.0, 200.0)),
                ..default()
            },
            Transform::from_xyz(0.0, -40.0, Z_UI + 49.0),
            VictoryStatsPanel,
            CleanupOnStateExit(GameState::Playing),
        ))
        .with_children(|parent| {
            // Time label and value
            parent.spawn((
                Text2d::new("TIME: --:--"),
                TextFont::from_font_size(32.0),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)), // Start invisible
                Transform::from_xyz(0.0, 40.0, 1.0),
                VictoryTimeText,
            ));

            // Reward label and value
            parent.spawn((
                Text2d::new("REWARD: 0 Z"),
                TextFont::from_font_size(32.0),
                TextColor(Color::srgba(1.0, 0.9, 0.2, 0.0)), // Start invisible
                Transform::from_xyz(0.0, -10.0, 1.0),
                VictoryRewardText,
            ));

            // Continue prompt
            parent.spawn((
                Text2d::new("Press SPACE to continue"),
                TextFont::from_font_size(20.0),
                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.0)), // Start invisible
                Transform::from_xyz(0.0, -70.0, 1.0),
                VictoryContinueText,
            ));
        });
}

// ============================================================================
// Update System
// ============================================================================

/// Update the victory outro sequence
pub fn update_outro(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut outro: ResMut<VictoryOutro>,
    mut clear_text: Query<
        (&mut TextColor, &mut Transform),
        (
            With<VictoryClearText>,
            Without<VictoryStatsPanel>,
            Without<VictoryTimeText>,
            Without<VictoryRewardText>,
            Without<VictoryContinueText>,
        ),
    >,
    mut stats_panel: Query<
        &mut Sprite,
        (
            With<VictoryStatsPanel>,
            Without<VictoryClearText>,
            Without<VictoryTimeText>,
            Without<VictoryRewardText>,
            Without<VictoryContinueText>,
        ),
    >,
    mut time_text: Query<
        (&mut Text2d, &mut TextColor),
        (
            With<VictoryTimeText>,
            Without<VictoryClearText>,
            Without<VictoryRewardText>,
            Without<VictoryContinueText>,
        ),
    >,
    mut reward_text: Query<
        (&mut Text2d, &mut TextColor),
        (
            With<VictoryRewardText>,
            Without<VictoryClearText>,
            Without<VictoryTimeText>,
            Without<VictoryContinueText>,
        ),
    >,
    mut continue_text: Query<
        &mut TextColor,
        (
            With<VictoryContinueText>,
            Without<VictoryClearText>,
            Without<VictoryTimeText>,
            Without<VictoryRewardText>,
        ),
    >,
) {
    outro.elapsed += time.delta_secs();

    // Update phase based on elapsed time
    let new_phase = if outro.elapsed < HITSTOP_DURATION {
        OutroPhase::HitStop
    } else if outro.elapsed < STATS_START {
        OutroPhase::Clear
    } else if outro.elapsed < WAIT_CONFIRM_START {
        OutroPhase::Stats
    } else {
        OutroPhase::WaitConfirm
    };

    outro.phase = new_phase;

    // Handle "CLEAR!" text animation
    for (mut color, mut transform) in &mut clear_text {
        match outro.phase {
            OutroPhase::HitStop => {
                // Stay invisible
                color.0 = Color::srgba(1.0, 0.9, 0.2, 0.0);
            }
            OutroPhase::Clear => {
                // Fade in and scale down (bounce effect)
                let phase_progress = (outro.elapsed - CLEAR_START) / CLEAR_DURATION;
                let alpha = phase_progress.min(1.0);
                color.0 = Color::srgba(1.0, 0.9, 0.2, alpha);

                // Start big, shrink to normal with overshoot
                let scale = if phase_progress < 0.6 {
                    1.5 - phase_progress * 0.8 // 1.5 -> 1.02
                } else {
                    1.0 + (1.0 - phase_progress) * 0.05 // Settle to 1.0
                };
                transform.scale = Vec3::splat(scale);
            }
            _ => {
                // Stay visible at normal scale
                color.0 = Color::srgba(1.0, 0.9, 0.2, 1.0);
                transform.scale = Vec3::splat(1.0);
            }
        }
    }

    // Handle stats panel animation
    for mut sprite in &mut stats_panel {
        match outro.phase {
            OutroPhase::HitStop | OutroPhase::Clear => {
                // Stay invisible
                sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
            _ => {
                // Fade in
                let phase_progress = ((outro.elapsed - STATS_START) / 0.3).min(1.0);
                sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.7 * phase_progress);
            }
        }
    }

    // Handle time text
    for (mut text, mut color) in &mut time_text {
        if outro.phase == OutroPhase::Stats || outro.phase == OutroPhase::WaitConfirm {
            let phase_progress = ((outro.elapsed - STATS_START) / STATS_DURATION).min(1.0);
            color.0 = Color::srgba(1.0, 1.0, 1.0, phase_progress);

            // Count up effect for time
            let displayed_time = outro.battle_time * phase_progress;
            let minutes = (displayed_time / 60.0) as u32;
            let seconds = (displayed_time % 60.0) as u32;
            let centis = ((displayed_time % 1.0) * 100.0) as u32;
            text.0 = format!("TIME: {:02}:{:02}.{:02}", minutes, seconds, centis);
        }
    }

    // Handle reward text
    for (mut text, mut color) in &mut reward_text {
        if outro.phase == OutroPhase::Stats || outro.phase == OutroPhase::WaitConfirm {
            let phase_progress =
                ((outro.elapsed - STATS_START - 0.2) / (STATS_DURATION - 0.2)).clamp(0.0, 1.0);
            color.0 = Color::srgba(1.0, 0.9, 0.2, phase_progress);

            // Count up effect for reward
            let displayed_reward = (outro.reward as f32 * phase_progress) as u64;
            text.0 = format!("REWARD: {} Z", displayed_reward);
        }
    }

    // Handle continue prompt (blink effect when waiting)
    for mut color in &mut continue_text {
        if outro.phase == OutroPhase::WaitConfirm {
            // Blink effect
            let blink = (outro.elapsed * 2.0).sin() * 0.3 + 0.7;
            color.0 = Color::srgba(0.7, 0.7, 0.7, blink);
        }
    }

    // Check for confirm input
    if outro.phase == OutroPhase::WaitConfirm {
        let keyboard_confirm =
            keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);

        let gamepad_confirm = gamepads
            .iter()
            .any(|gp| gp.just_pressed(GamepadButton::South)); // X on PlayStation, A on Xbox

        if keyboard_confirm || gamepad_confirm {
            outro.confirmed = true;
        }
    }
}

// ============================================================================
// Transition System - Handle state change after outro
// ============================================================================

/// Check if outro is complete and transition to next state
pub fn check_outro_complete(
    outro: Option<Res<VictoryOutro>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut campaign_progress: ResMut<CampaignProgress>,
    selected_battle: Option<Res<SelectedBattle>>,
) {
    let Some(outro) = outro else { return };

    if outro.is_done() {
        // Mark battle complete and transition
        if let Some(selected) = selected_battle {
            campaign_progress.complete_battle(selected.arc, selected.battle);
            info!(
                "Battle {} of Arc {} completed!",
                selected.battle + 1,
                selected.arc + 1
            );
            next_state.set(GameState::Campaign);
        } else {
            next_state.set(GameState::Shop);
        }
    }
}

// ============================================================================
// Cleanup System
// ============================================================================

/// Cleanup outro resources when leaving Playing state
pub fn cleanup_outro(mut commands: Commands) {
    commands.remove_resource::<VictoryOutro>();
    commands.remove_resource::<DefeatOutro>();
}

// ============================================================================
// Run Conditions
// ============================================================================

/// Run condition: only run if any outro is active (victory or defeat)
pub fn outro_active(victory: Option<Res<VictoryOutro>>, defeat: Option<Res<DefeatOutro>>) -> bool {
    victory.is_some() || defeat.is_some()
}

/// Run condition: only run if NO outro is active (normal gameplay)
pub fn outro_not_active(
    victory: Option<Res<VictoryOutro>>,
    defeat: Option<Res<DefeatOutro>>,
) -> bool {
    victory.is_none() && defeat.is_none()
}

/// Run condition: only run if VICTORY outro is active
pub fn victory_outro_active(victory: Option<Res<VictoryOutro>>) -> bool {
    victory.is_some()
}

/// Run condition: only run if DEFEAT outro is active
pub fn defeat_outro_active(defeat: Option<Res<DefeatOutro>>) -> bool {
    defeat.is_some()
}

// ============================================================================
// ============================================================================
// DEFEAT OUTRO SYSTEMS
// ============================================================================
// ============================================================================

// Defeat outro timing constants (slightly longer hitstop for dramatic effect)
const DEFEAT_HITSTOP_DURATION: f32 = 0.3;
const DEFEAT_GAMEOVER_START: f32 = 0.3;
const DEFEAT_GAMEOVER_DURATION: f32 = 0.5;
const DEFEAT_STATS_START: f32 = 0.8;
const DEFEAT_STATS_DURATION: f32 = 0.7;
const DEFEAT_WAIT_CONFIRM_START: f32 = 1.5;

// ============================================================================
// Defeat Setup System
// ============================================================================

/// Setup the defeat outro UI elements
pub fn setup_defeat_outro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    outro: Option<Res<DefeatOutro>>,
    existing_ui: Query<(), With<DefeatGameOverText>>,
) {
    // Only run if defeat outro is active but UI not yet spawned
    if outro.is_none() || !existing_ui.is_empty() {
        return;
    }

    // Play game over sound
    let gameover_sound: Handle<AudioSource> = asset_server.load("audio/sound/game-over.mp3");
    commands.spawn((
        AudioPlayer::new(gameover_sound),
        PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
        CleanupOnStateExit(GameState::Playing),
    ));

    // Spawn "GAME OVER" text (starts invisible, will animate in)
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont::from_font_size(90.0),
        TextColor(Color::srgba(1.0, 0.2, 0.2, 0.0)), // Start invisible, red color
        Transform::from_xyz(0.0, 80.0, Z_UI + 50.0),
        DefeatGameOverText,
        CleanupOnStateExit(GameState::Playing),
    ));

    // Stats panel background
    commands
        .spawn((
            Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.0), // Start invisible
                custom_size: Some(Vec2::new(400.0, 180.0)),
                ..default()
            },
            Transform::from_xyz(0.0, -40.0, Z_UI + 49.0),
            DefeatStatsPanel,
            CleanupOnStateExit(GameState::Playing),
        ))
        .with_children(|parent| {
            // Time label and value
            parent.spawn((
                Text2d::new("TIME: --:--"),
                TextFont::from_font_size(32.0),
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.0)), // Start invisible
                Transform::from_xyz(0.0, 30.0, 1.0),
                DefeatTimeText,
            ));

            // No reward message
            parent.spawn((
                Text2d::new("NO REWARD"),
                TextFont::from_font_size(28.0),
                TextColor(Color::srgba(0.6, 0.6, 0.6, 0.0)), // Start invisible, gray
                Transform::from_xyz(0.0, -15.0, 1.0),
                DefeatNoRewardText,
            ));

            // Continue prompt
            parent.spawn((
                Text2d::new("Press SPACE to continue"),
                TextFont::from_font_size(20.0),
                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.0)), // Start invisible
                Transform::from_xyz(0.0, -60.0, 1.0),
                DefeatContinueText,
            ));
        });
}

// ============================================================================
// Defeat Update System
// ============================================================================

/// Update the defeat outro sequence
pub fn update_defeat_outro(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut outro: ResMut<DefeatOutro>,
    mut gameover_text: Query<
        (&mut TextColor, &mut Transform),
        (
            With<DefeatGameOverText>,
            Without<DefeatStatsPanel>,
            Without<DefeatTimeText>,
            Without<DefeatNoRewardText>,
            Without<DefeatContinueText>,
        ),
    >,
    mut stats_panel: Query<
        &mut Sprite,
        (
            With<DefeatStatsPanel>,
            Without<DefeatGameOverText>,
            Without<DefeatTimeText>,
            Without<DefeatNoRewardText>,
            Without<DefeatContinueText>,
        ),
    >,
    mut time_text: Query<
        (&mut Text2d, &mut TextColor),
        (
            With<DefeatTimeText>,
            Without<DefeatGameOverText>,
            Without<DefeatNoRewardText>,
            Without<DefeatContinueText>,
        ),
    >,
    mut no_reward_text: Query<
        &mut TextColor,
        (
            With<DefeatNoRewardText>,
            Without<DefeatGameOverText>,
            Without<DefeatTimeText>,
            Without<DefeatContinueText>,
        ),
    >,
    mut continue_text: Query<
        &mut TextColor,
        (
            With<DefeatContinueText>,
            Without<DefeatGameOverText>,
            Without<DefeatTimeText>,
            Without<DefeatNoRewardText>,
        ),
    >,
) {
    outro.elapsed += time.delta_secs();

    // Update phase based on elapsed time
    let new_phase = if outro.elapsed < DEFEAT_HITSTOP_DURATION {
        DefeatPhase::HitStop
    } else if outro.elapsed < DEFEAT_STATS_START {
        DefeatPhase::GameOver
    } else if outro.elapsed < DEFEAT_WAIT_CONFIRM_START {
        DefeatPhase::Stats
    } else {
        DefeatPhase::WaitConfirm
    };

    outro.phase = new_phase;

    // Handle "GAME OVER" text animation
    for (mut color, mut transform) in &mut gameover_text {
        match outro.phase {
            DefeatPhase::HitStop => {
                // Stay invisible
                color.0 = Color::srgba(1.0, 0.2, 0.2, 0.0);
            }
            DefeatPhase::GameOver => {
                // Fade in with shake effect
                let phase_progress =
                    (outro.elapsed - DEFEAT_GAMEOVER_START) / DEFEAT_GAMEOVER_DURATION;
                let alpha = phase_progress.min(1.0);
                color.0 = Color::srgba(1.0, 0.2, 0.2, alpha);

                // Shake effect that settles
                let shake_intensity = (1.0 - phase_progress).max(0.0) * 10.0;
                let shake_x = (outro.elapsed * 50.0).sin() * shake_intensity;
                let shake_y = (outro.elapsed * 47.0).cos() * shake_intensity;
                transform.translation.x = shake_x;
                transform.translation.y = 80.0 + shake_y;

                // Start big, shrink to normal
                let scale = 1.3 - phase_progress * 0.3;
                transform.scale = Vec3::splat(scale.max(1.0));
            }
            _ => {
                // Stay visible at normal position and scale
                color.0 = Color::srgba(1.0, 0.2, 0.2, 1.0);
                transform.translation = Vec3::new(0.0, 80.0, Z_UI + 50.0);
                transform.scale = Vec3::splat(1.0);
            }
        }
    }

    // Handle stats panel animation
    for mut sprite in &mut stats_panel {
        match outro.phase {
            DefeatPhase::HitStop | DefeatPhase::GameOver => {
                // Stay invisible
                sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
            _ => {
                // Fade in
                let phase_progress = ((outro.elapsed - DEFEAT_STATS_START) / 0.3).min(1.0);
                sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.7 * phase_progress);
            }
        }
    }

    // Handle time text
    for (mut text, mut color) in &mut time_text {
        if outro.phase == DefeatPhase::Stats || outro.phase == DefeatPhase::WaitConfirm {
            let phase_progress =
                ((outro.elapsed - DEFEAT_STATS_START) / DEFEAT_STATS_DURATION).min(1.0);
            color.0 = Color::srgba(1.0, 1.0, 1.0, phase_progress);

            // Show final time (no count-up for defeat, just reveal)
            let displayed_time = outro.battle_time;
            let minutes = (displayed_time / 60.0) as u32;
            let seconds = (displayed_time % 60.0) as u32;
            let centis = ((displayed_time % 1.0) * 100.0) as u32;
            text.0 = format!("TIME: {:02}:{:02}.{:02}", minutes, seconds, centis);
        }
    }

    // Handle no reward text
    for mut color in &mut no_reward_text {
        if outro.phase == DefeatPhase::Stats || outro.phase == DefeatPhase::WaitConfirm {
            let phase_progress = ((outro.elapsed - DEFEAT_STATS_START - 0.1)
                / (DEFEAT_STATS_DURATION - 0.1))
                .clamp(0.0, 1.0);
            color.0 = Color::srgba(0.6, 0.6, 0.6, phase_progress);
        }
    }

    // Handle continue prompt (blink effect when waiting)
    for mut color in &mut continue_text {
        if outro.phase == DefeatPhase::WaitConfirm {
            // Blink effect
            let blink = (outro.elapsed * 2.0).sin() * 0.3 + 0.7;
            color.0 = Color::srgba(0.7, 0.7, 0.7, blink);
        }
    }

    // Check for confirm input
    if outro.phase == DefeatPhase::WaitConfirm {
        let keyboard_confirm =
            keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);

        let gamepad_confirm = gamepads
            .iter()
            .any(|gp| gp.just_pressed(GamepadButton::South));

        if keyboard_confirm || gamepad_confirm {
            outro.confirmed = true;
        }
    }
}

// ============================================================================
// Defeat Transition System
// ============================================================================

/// Check if defeat outro is complete and transition to campaign (no battle marked complete)
pub fn check_defeat_outro_complete(
    outro: Option<Res<DefeatOutro>>,
    mut next_state: ResMut<NextState<GameState>>,
    selected_battle: Option<Res<SelectedBattle>>,
) {
    let Some(outro) = outro else { return };

    if outro.is_done() {
        // Don't mark battle complete - player lost!
        if selected_battle.is_some() {
            info!("Returning to campaign after defeat...");
            next_state.set(GameState::Campaign);
        } else {
            // Fallback for non-campaign battles
            next_state.set(GameState::MainMenu);
        }
    }
}
