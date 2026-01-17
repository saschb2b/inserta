//! Weapon system for equippable weapons with unique characteristics.
//!
//! Each weapon has stats defining its behavior:
//! - Damage: Base damage dealt (can have multiple damage types)
//! - Charge Time: How quickly a weapon can be charged for heavy attacks
//! - Critical Chance/Multiplier: Chance and damage bonus for critical hits
//! - Fire Rate: How quickly the weapon fires (cooldown between shots)
//! - Damage Falloff: Range where damage starts decreasing
//! - Range: Maximum distance the weapon can hit

pub mod blaster;

use crate::assets::{ProjectileAnimation, ProjectileSprites};
use crate::resources::PlayerUpgrades;
use bevy::image::TextureAtlas;
use bevy::prelude::*;

// ============================================================================
// Weapon Stats & Types
// ============================================================================

/// Damage types that weapons can deal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DamageType {
    #[default]
    Physical,
    Fire,
    Ice,
    Electric,
    Void,
}

/// Damage configuration for a weapon
#[derive(Debug, Clone)]
pub struct DamageConfig {
    /// Base damage amount
    pub amount: i32,
    /// Type of damage dealt
    pub damage_type: DamageType,
}

impl Default for DamageConfig {
    fn default() -> Self {
        Self {
            amount: 1,
            damage_type: DamageType::Physical,
        }
    }
}

/// Critical hit configuration
#[derive(Debug, Clone, Copy)]
pub struct CriticalConfig {
    /// Base critical chance (0.0 - 1.0+)
    /// Above 1.0 = guaranteed crit with chance for orange crit
    /// Above 2.0 = guaranteed orange crit with chance for red crit
    pub chance: f32,
    /// Damage multiplier on critical hit (e.g., 2.0 = double damage)
    pub multiplier: f32,
    /// Orange crit multiplier (when chance > 100%)
    pub orange_multiplier: f32,
    /// Red crit multiplier (when chance > 200%)
    pub red_multiplier: f32,
}

impl Default for CriticalConfig {
    fn default() -> Self {
        Self {
            chance: 0.05,    // 5% base crit chance
            multiplier: 1.5, // 1.5x damage on crit
            orange_multiplier: 2.0,
            red_multiplier: 3.0,
        }
    }
}

/// Result of a critical hit roll
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CritResult {
    Normal,
    Critical,       // Yellow/white crit
    OrangeCritical, // Higher tier crit
    RedCritical,    // Highest tier crit
}

impl CriticalConfig {
    /// Roll for a critical hit and return the result
    pub fn roll(&self) -> CritResult {
        let roll: f32 = rand::random();

        if self.chance >= 2.0 {
            // Guaranteed orange crit, chance for red
            let red_chance = self.chance - 2.0;
            if roll < red_chance {
                CritResult::RedCritical
            } else {
                CritResult::OrangeCritical
            }
        } else if self.chance >= 1.0 {
            // Guaranteed crit, chance for orange
            let orange_chance = self.chance - 1.0;
            if roll < orange_chance {
                CritResult::OrangeCritical
            } else {
                CritResult::Critical
            }
        } else if roll < self.chance {
            CritResult::Critical
        } else {
            CritResult::Normal
        }
    }

    /// Get the damage multiplier for a crit result
    pub fn get_multiplier(&self, result: CritResult) -> f32 {
        match result {
            CritResult::Normal => 1.0,
            CritResult::Critical => self.multiplier,
            CritResult::OrangeCritical => self.orange_multiplier,
            CritResult::RedCritical => self.red_multiplier,
        }
    }
}

/// Damage falloff configuration
#[derive(Debug, Clone, Copy)]
pub struct FalloffConfig {
    /// Distance (in tiles) where falloff begins
    pub start_range: i32,
    /// Distance (in tiles) where minimum damage is reached
    pub end_range: i32,
    /// Minimum damage multiplier at max range (0.0 - 1.0)
    pub min_multiplier: f32,
}

impl Default for FalloffConfig {
    fn default() -> Self {
        Self {
            start_range: 4,      // Falloff starts at 4 tiles
            end_range: 6,        // Minimum at 6 tiles (full arena width)
            min_multiplier: 0.5, // 50% damage at max range
        }
    }
}

impl FalloffConfig {
    /// No falloff - full damage at all ranges
    pub fn none() -> Self {
        Self {
            start_range: 999,
            end_range: 999,
            min_multiplier: 1.0,
        }
    }

    /// Calculate damage multiplier based on distance traveled
    pub fn get_multiplier(&self, distance: i32) -> f32 {
        if distance <= self.start_range {
            1.0
        } else if distance >= self.end_range {
            self.min_multiplier
        } else {
            let range = (self.end_range - self.start_range) as f32;
            let progress = (distance - self.start_range) as f32 / range;
            1.0 - (1.0 - self.min_multiplier) * progress
        }
    }
}

/// Complete weapon statistics
#[derive(Debug, Clone)]
pub struct WeaponStats {
    /// Display name of the weapon
    pub name: String,
    /// Normal shot damage
    pub damage: DamageConfig,
    /// Charged shot damage (if weapon supports charging)
    pub charged_damage: Option<DamageConfig>,
    /// Time to fully charge (0.0 = no charging)
    pub charge_time: f32,
    /// Critical hit configuration
    pub critical: CriticalConfig,
    /// Time between shots (fire rate)
    pub fire_cooldown: f32,
    /// Damage falloff over distance
    pub falloff: FalloffConfig,
    /// Maximum range in tiles (projectile despawns after this)
    pub range: i32,
    /// Projectile speed (tiles per second)
    pub projectile_speed: f32,
    /// Visual: projectile size
    pub projectile_size: Vec2,
    /// Visual: projectile color (normal shot)
    pub projectile_color: Color,
    /// Visual: projectile color (charged shot)
    pub charged_projectile_color: Color,
    /// Visual: charged projectile size
    pub charged_projectile_size: Vec2,
}

impl WeaponStats {
    /// Apply player upgrades to the base weapon stats
    pub fn apply_upgrades(&mut self, upgrades: &PlayerUpgrades) {
        // Apply damage
        self.damage.amount += upgrades.get_bonus_damage();
        if let Some(ref mut charged) = self.charged_damage {
            // Charged shots get double the bonus
            charged.amount += upgrades.get_bonus_damage() * 2;
        }

        // Apply crit chance
        self.critical.chance += upgrades.get_crit_chance_bonus();

        // Apply fire rate (cooldown reduction)
        self.fire_cooldown *= upgrades.get_cooldown_modifier();
    }
}

impl Default for WeaponStats {
    fn default() -> Self {
        Self {
            name: "Default Weapon".to_string(),
            damage: DamageConfig::default(),
            charged_damage: None,
            charge_time: 0.0,
            critical: CriticalConfig::default(),
            fire_cooldown: 0.35,
            falloff: FalloffConfig::default(),
            range: 6,
            projectile_speed: 8.33, // tiles per second (matches 0.12s move timer)
            projectile_size: Vec2::new(18.0, 18.0),
            projectile_color: Color::srgb(1.0, 0.95, 0.2), // Yellow
            charged_projectile_color: Color::srgb(1.0, 0.5, 0.1), // Orange
            charged_projectile_size: Vec2::new(32.0, 32.0),
        }
    }
}

// ============================================================================
// Weapon Types & Registry
// ============================================================================

/// Enum of all available weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WeaponType {
    #[default]
    Blaster,
    // Future weapons:
    // Spreader,     // Multiple projectiles in a cone
    // Railgun,      // Instant hit, high damage, long charge
    // PlasmaCannon, // Area damage, slow projectile
    // etc.
}

impl WeaponType {
    /// Get the stats for this weapon type
    pub fn stats(&self) -> WeaponStats {
        match self {
            WeaponType::Blaster => blaster::blaster_stats(),
        }
    }
}

// ============================================================================
// Weapon Components
// ============================================================================

/// Component for an equipped weapon on an entity
#[derive(Component, Debug)]
pub struct EquippedWeapon {
    pub weapon_type: WeaponType,
    pub stats: WeaponStats,
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        let weapon_type = WeaponType::default();
        Self {
            stats: weapon_type.stats(),
            weapon_type,
        }
    }
}

impl EquippedWeapon {
    pub fn new(weapon_type: WeaponType) -> Self {
        Self {
            stats: weapon_type.stats(),
            weapon_type,
        }
    }
}

/// State of weapon firing/charging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WeaponFiringState {
    #[default]
    Ready,
    Charging,
    OnCooldown,
}

/// Component tracking weapon state (cooldowns, charging, etc.)
#[derive(Component, Debug)]
pub struct WeaponState {
    pub firing_state: WeaponFiringState,
    /// Timer for fire cooldown
    pub cooldown_timer: Timer,
    /// Timer for charging (when holding fire button)
    pub charge_timer: Option<Timer>,
    /// Whether the fire button is currently held
    pub fire_held: bool,
    /// Whether a charged shot is ready to release
    pub charge_ready: bool,
}

impl Default for WeaponState {
    fn default() -> Self {
        Self {
            firing_state: WeaponFiringState::Ready,
            cooldown_timer: Timer::from_seconds(0.35, TimerMode::Once),
            charge_timer: None,
            fire_held: false,
            charge_ready: false,
        }
    }
}

impl WeaponState {
    pub fn new(fire_cooldown: f32) -> Self {
        let mut timer = Timer::from_seconds(fire_cooldown, TimerMode::Once);
        timer.tick(std::time::Duration::from_secs_f32(fire_cooldown)); // Start ready
        Self {
            firing_state: WeaponFiringState::Ready,
            cooldown_timer: timer,
            charge_timer: None,
            fire_held: false,
            charge_ready: false,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.firing_state == WeaponFiringState::Ready && self.cooldown_timer.is_finished()
    }

    pub fn start_cooldown(&mut self, duration: f32) {
        self.firing_state = WeaponFiringState::OnCooldown;
        self.cooldown_timer = Timer::from_seconds(duration, TimerMode::Once);
        self.charge_timer = None;
        self.charge_ready = false;
    }

    pub fn start_charging(&mut self, charge_time: f32) {
        self.firing_state = WeaponFiringState::Charging;
        self.charge_timer = Some(Timer::from_seconds(charge_time, TimerMode::Once));
        self.charge_ready = false;
    }

    pub fn charge_progress(&self) -> f32 {
        self.charge_timer
            .as_ref()
            .map(|t| t.fraction())
            .unwrap_or(0.0)
    }
}

/// Marker component for projectiles fired from weapons
#[derive(Component, Debug)]
pub struct Projectile {
    /// Base damage this projectile deals
    pub damage: i32,
    /// Damage type
    pub damage_type: DamageType,
    /// Whether this is a charged shot
    pub is_charged: bool,
    /// Starting x position (for falloff calculation)
    pub origin_x: i32,
    /// Critical hit result (rolled on fire)
    pub crit_result: CritResult,
    /// Critical multiplier to apply
    pub crit_multiplier: f32,
    /// Falloff configuration
    pub falloff: FalloffConfig,
    /// Maximum range
    pub max_range: i32,
}

impl Projectile {
    /// Calculate final damage based on distance traveled and crit
    pub fn calculate_damage(&self, current_x: i32) -> i32 {
        let distance = (current_x - self.origin_x).abs();
        let falloff_mult = self.falloff.get_multiplier(distance);
        let base_damage = self.damage as f32 * self.crit_multiplier * falloff_mult;
        base_damage.round() as i32
    }
}

// ============================================================================
// Weapon Plugin
// ============================================================================

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                weapon_input_system,
                weapon_cooldown_system,
                projectile_hit_system,
            )
                .run_if(in_state(crate::components::GameState::Playing)),
        );
    }
}

// ============================================================================
// Weapon Systems
// ============================================================================

use crate::components::{
    Bullet, Enemy, EnemyBullet, FlashTimer, GridPosition, Health, HealthText, Lifetime, MoveTimer,
    MuzzleFlash, Player, ProjectileHit, ProjectileImmobile, RenderConfig, TargetsTiles,
};
use crate::constants::*;

/// Handle weapon input (fire button press/hold/release)
pub fn weapon_input_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
    projectiles: Res<ProjectileSprites>,
    mut query: Query<(&GridPosition, &EquippedWeapon, &mut WeaponState), With<Player>>,
) {
    for (player_pos, weapon, mut state) in &mut query {
        let mut fire_pressed = keyboard.just_pressed(KeyCode::Space);
        let mut fire_held = keyboard.pressed(KeyCode::Space);
        let mut fire_released = keyboard.just_released(KeyCode::Space);

        // Gamepad Input
        for gamepad in gamepads.iter() {
            if gamepad.just_pressed(GamepadButton::South)
                || gamepad.just_pressed(GamepadButton::RightTrigger2)
            {
                fire_pressed = true;
            }
            if gamepad.pressed(GamepadButton::South)
                || gamepad.pressed(GamepadButton::RightTrigger2)
            {
                fire_held = true;
            }
            if gamepad.just_released(GamepadButton::South)
                || gamepad.just_released(GamepadButton::RightTrigger2)
            {
                fire_released = true;
            }
        }

        state.fire_held = fire_held;

        // Update cooldown
        if state.firing_state == WeaponFiringState::OnCooldown {
            state.cooldown_timer.tick(time.delta());
            if state.cooldown_timer.is_finished() {
                state.firing_state = WeaponFiringState::Ready;
            }
        }

        // Update charging
        if state.firing_state == WeaponFiringState::Charging {
            if let Some(ref mut timer) = state.charge_timer {
                timer.tick(time.delta());
                if timer.is_finished() {
                    state.charge_ready = true;
                }
            }
        }

        // Handle fire button press - immediate shot for blaster
        if fire_pressed && state.is_ready() {
            // Fire normal shot immediately
            spawn_projectile(&mut commands, player_pos, weapon, false, &projectiles);

            // Start charging if weapon supports it
            if weapon.stats.charge_time > 0.0 {
                state.start_charging(weapon.stats.charge_time);
            } else {
                state.start_cooldown(weapon.stats.fire_cooldown);
            }
        }

        // Handle fire button release - charged shot if ready
        if fire_released && state.firing_state == WeaponFiringState::Charging {
            if state.charge_ready {
                // Fire charged shot
                spawn_projectile(&mut commands, player_pos, weapon, true, &projectiles);
            }
            // Start cooldown regardless
            state.start_cooldown(weapon.stats.fire_cooldown);
        }

        // Handle holding without charging complete - cancel on release
        if fire_released && state.firing_state == WeaponFiringState::Charging && !state.charge_ready
        {
            state.start_cooldown(weapon.stats.fire_cooldown * 0.5); // Shorter cooldown for cancelled charge
        }
    }
}

/// Spawn a projectile from a weapon
fn spawn_projectile(
    commands: &mut Commands,
    player_pos: &GridPosition,
    weapon: &EquippedWeapon,
    is_charged: bool,
    projectiles: &ProjectileSprites,
) {
    let stats = &weapon.stats;

    let damage = if is_charged {
        let charged = stats.charged_damage.as_ref().unwrap_or(&stats.damage);
        charged.amount
    } else {
        stats.damage.amount
    };

    // Roll for crit
    let crit_result = stats.critical.roll();
    let crit_multiplier = stats.critical.get_multiplier(crit_result);

    // Spawn projectile entity with sprite animation
    // The blaster projectile is 64x16 with 4 frames: launch, travel, impact, finish
    // Choose sprite based on whether it's charged
    let (sprite_image, sprite_layout) = if is_charged {
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

    commands.spawn((
        Sprite {
            image: sprite_image,
            texture_atlas: Some(TextureAtlas {
                layout: sprite_layout,
                index: 1, // Start at travel frame
            }),
            custom_size: Some(BULLET_DRAW_SIZE),
            ..default()
        },
        Transform::default(),
        GridPosition {
            x: player_pos.x,
            y: player_pos.y,
        },
        RenderConfig {
            offset: BULLET_OFFSET,
            base_z: Z_BULLET,
        },
        Bullet,
        Projectile {
            damage,
            damage_type: stats.damage.damage_type,
            is_charged,
            origin_x: player_pos.x,
            crit_result,
            crit_multiplier,
            falloff: stats.falloff,
            max_range: stats.range,
        },
        ProjectileAnimation::blaster(is_charged),
        MoveTimer(Timer::from_seconds(BULLET_MOVE_TIMER, TimerMode::Repeating)),
        TargetsTiles::single(), // Highlight tile at bullet's position
    ));

    // Muzzle flash
    commands.spawn((
        Sprite {
            color: COLOR_MUZZLE,
            custom_size: Some(Vec2::new(22.0, 12.0)),
            ..default()
        },
        Transform::default(),
        GridPosition {
            x: player_pos.x,
            y: player_pos.y,
        },
        RenderConfig {
            offset: MUZZLE_OFFSET,
            base_z: Z_BULLET + 1.0,
        },
        MuzzleFlash,
        Lifetime(Timer::from_seconds(MUZZLE_TIME, TimerMode::Once)),
    ));
}

/// Update weapon cooldowns
pub fn weapon_cooldown_system(time: Res<Time>, mut query: Query<&mut WeaponState>) {
    for mut state in &mut query {
        if state.firing_state == WeaponFiringState::OnCooldown {
            state.cooldown_timer.tick(time.delta());
            if state.cooldown_timer.is_finished() {
                state.firing_state = WeaponFiringState::Ready;
            }
        }
    }
}

/// Handle projectiles hitting enemies (with proper damage calculation)
pub fn projectile_hit_system(
    mut commands: Commands,
    projectile_query: Query<
        (
            Entity,
            &GridPosition,
            &Projectile,
            &crate::assets::ProjectileAnimation,
        ),
        (With<Bullet>, Without<EnemyBullet>, Without<ProjectileHit>),
    >,
    mut enemy_query: Query<(Entity, &GridPosition, &mut Health, &Children), With<Enemy>>,
    mut text_query: Query<&mut Text2d, With<HealthText>>,
) {
    for (bullet_entity, bullet_pos, projectile, anim) in &projectile_query {
        for (enemy_entity, enemy_pos, mut health, children) in &mut enemy_query {
            if bullet_pos == enemy_pos {
                // Calculate damage with falloff and crit
                let final_damage = projectile.calculate_damage(bullet_pos.x);

                health.current -= final_damage;

                // Transition projectile to impact state instead of despawning immediately
                // Preserve the is_charged flag from the original animation
                commands.entity(bullet_entity).insert((
                    crate::assets::ProjectileAnimation {
                        frame_indices: [0, 1, 2, 3],
                        state: crate::assets::ProjectileAnimationState::Impact,
                        timer: Timer::from_seconds(0.1, TimerMode::Once), // Short duration for impact
                        is_charged: anim.is_charged,
                    },
                    ProjectileHit, // Mark as hit so it will despawn after finish state
                    ProjectileImmobile, // Stop moving during animation
                ));

                // Update HP text
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = health.current.max(0).to_string();
                    }
                }

                if health.current <= 0 {
                    commands.entity(enemy_entity).despawn();
                } else {
                    commands
                        .entity(enemy_entity)
                        .insert(FlashTimer(Timer::from_seconds(FLASH_TIME, TimerMode::Once)));
                }

                break; // Bullet hit one enemy, stop checking
            }
        }
    }
}
