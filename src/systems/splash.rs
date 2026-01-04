use bevy::prelude::*;

use crate::components::{CleanupOnStateExit, GameState};
use crate::constants::*;

/// Marker for the splash screen container
#[derive(Component)]
pub struct SplashScreen;

/// Timer for auto-advancing from splash
#[derive(Resource)]
pub struct SplashTimer(pub Timer);

/// Setup the splash screen
pub fn setup_splash(mut commands: Commands) {
    // Splash background - darker than game background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.02, 0.02, 0.08),
            custom_size: Some(Vec2::new(SCREEN_WIDTH + 200.0, SCREEN_HEIGHT + 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        SplashScreen,
        CleanupOnStateExit(GameState::Splash),
    ));

    // Game title
    commands.spawn((
        Text2d::new("INSERTA"),
        TextFont::from_font_size(120.0),
        TextColor(Color::srgb(0.9, 0.4, 0.3)),
        Transform::from_xyz(0.0, 80.0, 1.0),
        SplashScreen,
        CleanupOnStateExit(GameState::Splash),
    ));

    // Subtitle
    commands.spawn((
        Text2d::new("Battle Network"),
        TextFont::from_font_size(36.0),
        TextColor(Color::srgb(0.5, 0.7, 0.9)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        SplashScreen,
        CleanupOnStateExit(GameState::Splash),
    ));

    // Press any key prompt
    commands.spawn((
        Text2d::new("Press SPACE or ENTER to continue"),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
        Transform::from_xyz(0.0, -150.0, 1.0),
        SplashScreen,
        CleanupOnStateExit(GameState::Splash),
    ));

    // Decorative cyber lines
    for i in 0..5 {
        let y_offset = (i as f32 - 2.0) * 60.0;
        let alpha = 0.15 - (i as f32 - 2.0).abs() * 0.03;
        commands.spawn((
            Sprite {
                color: Color::srgba(0.3, 0.5, 0.8, alpha),
                custom_size: Some(Vec2::new(SCREEN_WIDTH * 1.5, 2.0)),
                ..default()
            },
            Transform::from_xyz(0.0, y_offset - 200.0, 0.5),
            SplashScreen,
            CleanupOnStateExit(GameState::Splash),
        ));
    }

    // Insert splash timer (auto-advance after delay, or skip with input)
    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

/// Handle splash screen input and timing
pub fn update_splash(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    // Advance on timer completion or key press
    if timer.0.is_finished()
        || keyboard.just_pressed(KeyCode::Space)
        || keyboard.just_pressed(KeyCode::Enter)
        || keyboard.just_pressed(KeyCode::Escape)
    {
        next_state.set(GameState::MainMenu);
    }
}

/// Animate the splash screen elements (pulsing text, etc.)
pub fn animate_splash(
    time: Res<Time>,
    mut query: Query<(&mut TextColor, &Transform), With<SplashScreen>>,
) {
    let t = time.elapsed_secs();

    for (mut color, transform) in &mut query {
        // Only animate the "press any key" text (at y = -150)
        if (transform.translation.y - (-150.0)).abs() < 10.0 {
            let alpha = 0.5 + 0.3 * (t * 2.0).sin();
            color.0 = Color::srgba(0.7, 0.7, 0.7, alpha);
        }
    }
}

/// Cleanup splash screen resources
pub fn cleanup_splash(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}
