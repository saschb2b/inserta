// ============================================================================
// Action Blueprints - Complete action definitions
// ============================================================================
//
// A blueprint is a complete template for an action.
// It combines stats, targeting, effects, and visuals into one package.
//
// ADDING A NEW ACTION:
// 1. Add variant to ActionId enum in components.rs
// 2. Create blueprint function below (e.g., fn my_chip() -> ActionBlueprint)
// 3. Add match arm in ActionBlueprint::get()

use super::{
    ActionEffect, ActionId, ActionModifiers, ActionTarget, ActionVisuals, Element, Rarity, colors,
};
use bevy::prelude::*;

/// Complete blueprint for an action/chip
#[derive(Debug, Clone)]
pub struct ActionBlueprint {
    /// Unique identifier
    pub id: ActionId,
    /// Display name
    pub name: &'static str,
    /// Description for UI
    pub description: &'static str,
    /// Element type
    pub element: Element,
    /// Rarity (* to *****)
    pub rarity: Rarity,

    // Timing
    /// Cooldown after use (seconds)
    pub cooldown: f32,
    /// Charge time before activation (0 = instant)
    pub charge_time: f32,

    // Behavior
    /// How the action targets
    pub target: ActionTarget,
    /// What the action does
    pub effect: ActionEffect,
    /// Optional modifiers
    pub modifiers: ActionModifiers,

    // Visuals
    pub visuals: ActionVisuals,
}

impl ActionBlueprint {
    /// Get the blueprint for a given action ID
    pub fn get(id: ActionId) -> Self {
        match id {
            // Recovery chips
            ActionId::Recov10 => recov(10, 1, Rarity::Common),
            ActionId::Recov30 => recov(30, 2, Rarity::Common),
            ActionId::Recov50 => recov(50, 3, Rarity::Common),
            ActionId::Recov80 => recov(80, 4, Rarity::Common),
            ActionId::Recov120 => recov(120, 5, Rarity::Uncommon),
            ActionId::Recov150 => recov(150, 6, Rarity::Uncommon),
            ActionId::Recov200 => recov(200, 7, Rarity::Rare),
            ActionId::Recov300 => recov(300, 8, Rarity::SuperRare),

            // Defense chips
            ActionId::Barrier => barrier(),
            ActionId::Shield => shield(),
            ActionId::MetGuard => met_guard(),
            ActionId::Invis1 => invis(1),
            ActionId::Invis2 => invis(2),
            ActionId::Invis3 => invis(3),
            ActionId::LifeAura => life_aura(),

            // Sword chips
            ActionId::Sword => sword(80, Rarity::Common, "Sword", 1),
            ActionId::WideSwrd => wide_sword(),
            ActionId::LongSwrd => long_sword(),
            ActionId::FireSwrd => fire_sword(),
            ActionId::AquaSwrd => aqua_sword(),
            ActionId::ElecSwrd => elec_sword(),
            ActionId::FtrSwrd => fighter_sword(),
            ActionId::KngtSwrd => knight_sword(),
            ActionId::HeroSwrd => hero_sword(),

            // Cannon chips
            ActionId::Cannon => cannon(40, Rarity::Common, "Cannon"),
            ActionId::HiCannon => cannon(80, Rarity::Uncommon, "HiCannon"),
            ActionId::MCannon => cannon(120, Rarity::Rare, "M-Cannon"),

            // Bomb chips
            ActionId::MiniBomb => bomb(50, 1, Rarity::Common, "MiniBomb"),
            ActionId::LilBomb => bomb(80, 1, Rarity::Common, "LilBomb"),
            ActionId::CrosBomb => cross_bomb(),
            ActionId::BigBomb => bomb(90, 2, Rarity::Rare, "BigBomb"),

            // Wave chips
            ActionId::ShokWave => shockwave(60, Rarity::Common, "ShokWave"),
            ActionId::SoniWave => shockwave(80, Rarity::Uncommon, "SoniWave"),
            ActionId::DynaWave => shockwave(100, Rarity::Rare, "DynaWave"),

            // Spread chips
            ActionId::Shotgun => shotgun(),
            ActionId::Spreader => spreader(),
            ActionId::Bubbler => bubbler(),

            // Tower chips
            ActionId::FireTowr => fire_tower(),
            ActionId::AquaTowr => aqua_tower(),
            ActionId::WoodTowr => wood_tower(),

            // Quake chips
            ActionId::Quake1 => quake(90, Rarity::Common, "Quake1"),
            ActionId::Quake2 => quake(120, Rarity::Uncommon, "Quake2"),
            ActionId::Quake3 => quake(150, Rarity::Rare, "Quake3"),

            // Thunder chips
            ActionId::Thunder1 => thunder(90, Rarity::Common, "Thunder1"),
            ActionId::Thunder2 => thunder(120, Rarity::Uncommon, "Thunder2"),
            ActionId::Thunder3 => thunder(150, Rarity::Rare, "Thunder3"),

            // Misc chips
            ActionId::Ratton1 => ratton(90, Rarity::Common, "Ratton1"),
            ActionId::Ratton2 => ratton(110, Rarity::Uncommon, "Ratton2"),
            ActionId::Ratton3 => ratton(130, Rarity::Rare, "Ratton3"),
            ActionId::Dash => dash(),
            ActionId::GutsPnch => guts_punch(),
            ActionId::IcePunch => ice_punch(),

            // Panel chips
            ActionId::Steal => area_steal(),
            ActionId::Geddon1 => geddon(1),
            ActionId::Geddon2 => geddon(2),
            ActionId::Repair => repair(),
        }
    }

    /// Get display name with rarity stars
    pub fn display_name(&self) -> String {
        let stars = match self.rarity {
            Rarity::Common => "*",
            Rarity::Uncommon => "**",
            Rarity::Rare => "***",
            Rarity::SuperRare => "****",
            Rarity::UltraRare => "*****",
        };
        format!("{} {}", self.name, stars)
    }
}

// ============================================================================
// Recovery Chips
// ============================================================================

fn recov(amount: i32, tier: i32, rarity: Rarity) -> ActionBlueprint {
    ActionBlueprint {
        id: match amount {
            10 => ActionId::Recov10,
            30 => ActionId::Recov30,
            50 => ActionId::Recov50,
            80 => ActionId::Recov80,
            120 => ActionId::Recov120,
            150 => ActionId::Recov150,
            200 => ActionId::Recov200,
            _ => ActionId::Recov300,
        },
        name: match amount {
            10 => "Recov10",
            30 => "Recov30",
            50 => "Recov50",
            80 => "Recov80",
            120 => "Recov120",
            150 => "Recov150",
            200 => "Recov200",
            _ => "Recov300",
        },
        description: "Recover HP",
        element: Element::None,
        rarity,
        cooldown: 4.0 + tier as f32 * 1.0, // Higher heals = longer cooldown
        charge_time: 0.0,                  // Instant
        target: ActionTarget::OnSelf,
        effect: ActionEffect::heal(amount),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::heal(colors::HEAL_GREEN, colors::HEAL_GREEN),
    }
}

// ============================================================================
// Defense Chips
// ============================================================================

fn barrier() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Barrier,
        name: "Barrier",
        description: "Nullify 1 enemy attack",
        element: Element::None,
        rarity: Rarity::Uncommon,
        cooldown: 5.0,
        charge_time: 0.0,
        target: ActionTarget::OnSelf,
        effect: ActionEffect::Shield {
            duration: 10.0,     // Lasts until hit
            threshold: Some(0), // Blocks any damage, but breaks after 1 hit
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::shield(colors::BARRIER_CYAN, colors::BARRIER_CYAN),
    }
}

fn shield() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Shield,
        name: "Shield",
        description: "Block all damage for 2 seconds",
        element: Element::None,
        rarity: Rarity::Uncommon,
        cooldown: 6.0,
        charge_time: 0.0,
        target: ActionTarget::OnSelf,
        effect: ActionEffect::shield(2.0),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::shield(colors::SHIELD_BLUE, colors::SHIELD_BLUE),
    }
}

fn met_guard() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::MetGuard,
        name: "MetGuard",
        description: "Hold for 3 sec defense",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 3.0,
        charge_time: 0.0, // Defensive stance handled separately
        target: ActionTarget::OnSelf,
        effect: ActionEffect::shield(3.0),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::shield(colors::WAVE_GRAY, colors::WAVE_GRAY),
    }
}

fn invis(tier: i32) -> ActionBlueprint {
    let duration = 1.5 + tier as f32 * 0.5;
    ActionBlueprint {
        id: match tier {
            1 => ActionId::Invis1,
            2 => ActionId::Invis2,
            _ => ActionId::Invis3,
        },
        name: match tier {
            1 => "Invis1",
            2 => "Invis2",
            _ => "Invis3",
        },
        description: "Temporary immunity",
        element: Element::None,
        rarity: match tier {
            1 => Rarity::Uncommon,
            2 => Rarity::Rare,
            _ => Rarity::SuperRare,
        },
        cooldown: 8.0 + tier as f32 * 2.0,
        charge_time: 0.0,
        target: ActionTarget::OnSelf,
        effect: ActionEffect::Invisibility { duration },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals {
            icon_color: Color::srgba(0.8, 0.8, 1.0, 0.5),
            effect_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
            ..default()
        },
    }
}

fn life_aura() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::LifeAura,
        name: "LifeAura",
        description: "Negate all attacks with damage<100",
        element: Element::None,
        rarity: Rarity::UltraRare,
        cooldown: 20.0,
        charge_time: 0.0,
        target: ActionTarget::OnSelf,
        effect: ActionEffect::aura(15.0, 100),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::shield(colors::AURA_GOLD, colors::AURA_GOLD),
    }
}

// ============================================================================
// Sword Chips
// ============================================================================

fn sword(damage: i32, rarity: Rarity, name: &'static str, range: i32) -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Sword,
        name,
        description: "Cut down enemies",
        element: Element::None,
        rarity,
        cooldown: 3.0,
        charge_time: 0.2, // Quick melee
        target: ActionTarget::SingleTile { range },
        effect: ActionEffect::damage(damage),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::SWORD_WHITE, colors::SWORD_WHITE),
    }
}

fn wide_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::WideSwrd,
        name: "WideSwrd",
        description: "Cut down column! Range=1",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 4.0,
        charge_time: 0.3,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::damage(80),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::SWORD_PINK, colors::SWORD_PINK),
    }
}

fn long_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::LongSwrd,
        name: "LongSwrd",
        description: "Cut down enemies! Range=2",
        element: Element::None,
        rarity: Rarity::Uncommon,
        cooldown: 4.0,
        charge_time: 0.25,
        target: ActionTarget::Pattern {
            tiles: vec![(1, 0), (2, 0)], // Hits 2 tiles forward
        },
        effect: ActionEffect::damage(100),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::SWORD_WHITE, colors::SWORD_WHITE),
    }
}

fn fire_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::FireSwrd,
        name: "FireSwrd",
        description: "Cuts down column Range=1 [Fire]",
        element: Element::Fire,
        rarity: Rarity::Uncommon,
        cooldown: 4.5,
        charge_time: 0.3,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(120, Element::Fire),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::FIRE, colors::SWORD_FIRE),
    }
}

fn aqua_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::AquaSwrd,
        name: "AquaSwrd",
        description: "Cuts down column Range=1 [Aqua]",
        element: Element::Aqua,
        rarity: Rarity::Rare,
        cooldown: 4.5,
        charge_time: 0.3,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(150, Element::Aqua),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::AQUA, colors::SWORD_AQUA),
    }
}

fn elec_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::ElecSwrd,
        name: "ElecSwrd",
        description: "Cuts down column Range=1 [Elec]",
        element: Element::Elec,
        rarity: Rarity::Rare,
        cooldown: 4.5,
        charge_time: 0.3,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(130, Element::Elec),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::ELEC, colors::SWORD_ELEC),
    }
}

fn fighter_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::FtrSwrd,
        name: "FtrSwrd",
        description: "Warrior's sword Range=3",
        element: Element::None,
        rarity: Rarity::SuperRare,
        cooldown: 5.0,
        charge_time: 0.3,
        target: ActionTarget::Pattern {
            tiles: vec![(1, 0), (2, 0), (3, 0)],
        },
        effect: ActionEffect::damage(100),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::SWORD_WHITE, colors::SWORD_WHITE),
    }
}

fn knight_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::KngtSwrd,
        name: "KngtSwrd",
        description: "Knight's sword Range=3",
        element: Element::None,
        rarity: Rarity::SuperRare,
        cooldown: 5.5,
        charge_time: 0.35,
        target: ActionTarget::Pattern {
            tiles: vec![(1, 0), (2, 0), (3, 0)],
        },
        effect: ActionEffect::damage(150),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::SWORD_WHITE, colors::SWORD_WHITE),
    }
}

fn hero_sword() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::HeroSwrd,
        name: "HeroSwrd",
        description: "Legendary sword Range=3",
        element: Element::None,
        rarity: Rarity::UltraRare,
        cooldown: 6.0,
        charge_time: 0.4,
        target: ActionTarget::Pattern {
            tiles: vec![(1, 0), (2, 0), (3, 0)],
        },
        effect: ActionEffect::damage(200),
        modifiers: ActionModifiers {
            guard_break: true,
            ..default()
        },
        visuals: ActionVisuals::sword_slash(colors::AURA_GOLD, colors::AURA_GOLD),
    }
}

// ============================================================================
// Cannon Chips
// ============================================================================

fn cannon(damage: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "Cannon" => ActionId::Cannon,
            "HiCannon" => ActionId::HiCannon,
            _ => ActionId::MCannon,
        },
        name,
        description: "A nice, big cannon!",
        element: Element::None,
        rarity,
        cooldown: 3.0 + (damage as f32 / 40.0),
        charge_time: 0.2,
        target: ActionTarget::Projectile {
            x_offset: 1,
            piercing: false,
        },
        effect: ActionEffect::damage(damage),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::CANNON_YELLOW, colors::CANNON_ORANGE),
    }
}

// ============================================================================
// Bomb Chips
// ============================================================================

fn bomb(damage: i32, radius: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "MiniBomb" => ActionId::MiniBomb,
            "LilBomb" => ActionId::LilBomb,
            _ => ActionId::BigBomb,
        },
        name,
        description: "Throw a bomb!",
        element: Element::None,
        rarity,
        cooldown: 4.0,
        charge_time: 0.3,
        target: ActionTarget::AreaAtPosition {
            x_offset: 3, // Throws 3 tiles forward
            y_offset: 0,
            pattern: generate_radius_pattern(radius),
        },
        effect: ActionEffect::Delayed {
            delay: 0.8,
            effect: Box::new(ActionEffect::damage(damage)),
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::explosion(
            colors::BOMB_RED,
            colors::BOMB_ORANGE,
            Vec2::new(80.0, 80.0),
        ),
    }
}

fn cross_bomb() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::CrosBomb,
        name: "CrosBomb",
        description: "Cross bomb pattern",
        element: Element::None,
        rarity: Rarity::Uncommon,
        cooldown: 4.5,
        charge_time: 0.3,
        target: ActionTarget::AreaAtPosition {
            x_offset: 3,
            y_offset: 0,
            pattern: vec![(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)], // Cross pattern
        },
        effect: ActionEffect::Delayed {
            delay: 0.8,
            effect: Box::new(ActionEffect::damage(80)),
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::explosion(
            colors::BOMB_RED,
            colors::BOMB_ORANGE,
            Vec2::new(100.0, 100.0),
        ),
    }
}

// ============================================================================
// Wave/Shockwave Chips
// ============================================================================

fn shockwave(damage: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "ShokWave" => ActionId::ShokWave,
            "SoniWave" => ActionId::SoniWave,
            _ => ActionId::DynaWave,
        },
        name,
        description: "Piercing ground wave",
        element: Element::None,
        rarity,
        cooldown: 3.5,
        charge_time: 0.25,
        target: ActionTarget::Row {
            x_offset: 1,
            traveling: true, // Travels along ground
        },
        effect: ActionEffect::Damage {
            amount: damage,
            element: Element::None,
            can_crit: false,
            guard_break: false,
        },
        modifiers: ActionModifiers {
            destroys_obstacles: true, // Breaks rocks
            ..default()
        },
        visuals: ActionVisuals::projectile(colors::WAVE_GRAY, colors::WAVE_YELLOW),
    }
}

// ============================================================================
// Spread Chips
// ============================================================================

fn shotgun() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Shotgun,
        name: "Shotgun",
        description: "Hits enemy and keeps going 1pnl",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 3.0,
        charge_time: 0.2,
        target: ActionTarget::ProjectileSpread {
            x_offset: 1,
            spread_rows: vec![0], // Just hits in a line, but continues
        },
        effect: ActionEffect::damage(30),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::CANNON_YELLOW, colors::CANNON_YELLOW),
    }
}

fn spreader() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Spreader,
        name: "Spreader",
        description: "Gun with a 1-panel blast",
        element: Element::None,
        rarity: Rarity::Uncommon,
        cooldown: 3.5,
        charge_time: 0.2,
        target: ActionTarget::AreaAtPosition {
            x_offset: 3,
            y_offset: 0,
            pattern: vec![(0, 0), (0, 1), (0, -1), (-1, 0)], // Splash pattern
        },
        effect: ActionEffect::damage(30),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::CANNON_YELLOW, colors::CANNON_YELLOW),
    }
}

fn bubbler() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Bubbler,
        name: "Bubbler",
        description: "Bubbles w/ a 1-panel blast [Aqua]",
        element: Element::Aqua,
        rarity: Rarity::Common,
        cooldown: 3.5,
        charge_time: 0.2,
        target: ActionTarget::AreaAtPosition {
            x_offset: 3,
            y_offset: 0,
            pattern: vec![(0, 0), (0, 1), (0, -1), (-1, 0)],
        },
        effect: ActionEffect::elemental_damage(50, Element::Aqua),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::AQUA, colors::AQUA),
    }
}

// ============================================================================
// Tower Chips (hit column, travel up/down)
// ============================================================================

fn fire_tower() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::FireTowr,
        name: "FireTowr",
        description: "Fire that can move up & down [Fire]",
        element: Element::Fire,
        rarity: Rarity::Uncommon,
        cooldown: 5.0,
        charge_time: 0.4,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(100, Element::Fire),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::FIRE, colors::FIRE),
    }
}

fn aqua_tower() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::AquaTowr,
        name: "AquaTowr",
        description: "Water that can move up & down [Aqua]",
        element: Element::Aqua,
        rarity: Rarity::Uncommon,
        cooldown: 5.0,
        charge_time: 0.4,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(120, Element::Aqua),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::AQUA, colors::AQUA),
    }
}

fn wood_tower() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::WoodTowr,
        name: "WoodTowr",
        description: "Log that can move up & down [Wood]",
        element: Element::Wood,
        rarity: Rarity::Uncommon,
        cooldown: 5.0,
        charge_time: 0.4,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(140, Element::Wood),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::WOOD, colors::WOOD),
    }
}

// ============================================================================
// Quake Chips
// ============================================================================

fn quake(damage: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "Quake1" => ActionId::Quake1,
            "Quake2" => ActionId::Quake2,
            _ => ActionId::Quake3,
        },
        name,
        description: "Cracks a panel, damages enemies",
        element: Element::None,
        rarity,
        cooldown: 4.0,
        charge_time: 0.3,
        target: ActionTarget::AreaAtPosition {
            x_offset: 3,
            y_offset: 0,
            pattern: vec![(0, 0)],
        },
        effect: ActionEffect::Combo {
            effects: vec![
                ActionEffect::damage(damage),
                ActionEffect::CrackPanel { crack_only: true },
            ],
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::explosion(
            colors::WAVE_GRAY,
            colors::WAVE_GRAY,
            Vec2::new(64.0, 64.0),
        ),
    }
}

// ============================================================================
// Thunder Chips
// ============================================================================

fn thunder(damage: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "Thunder1" => ActionId::Thunder1,
            "Thunder2" => ActionId::Thunder2,
            _ => ActionId::Thunder3,
        },
        name,
        description: "A rolling lightning attack [Elec]",
        element: Element::Elec,
        rarity,
        cooldown: 4.0,
        charge_time: 0.3,
        target: ActionTarget::Projectile {
            x_offset: 1,
            piercing: true, // Thunder goes through enemies
        },
        effect: ActionEffect::elemental_damage(damage, Element::Elec),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::ELEC, colors::ELEC),
    }
}

// ============================================================================
// Ratton Chips (homing missiles)
// ============================================================================

fn ratton(damage: i32, rarity: Rarity, name: &'static str) -> ActionBlueprint {
    ActionBlueprint {
        id: match name {
            "Ratton1" => ActionId::Ratton1,
            "Ratton2" => ActionId::Ratton2,
            _ => ActionId::Ratton3,
        },
        name,
        description: "Missile that can turn once",
        element: Element::None,
        rarity,
        cooldown: 3.5,
        charge_time: 0.2,
        target: ActionTarget::Projectile {
            x_offset: 1,
            piercing: false,
        },
        effect: ActionEffect::damage(damage),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::BOMB_ORANGE, colors::BOMB_ORANGE),
    }
}

// ============================================================================
// Misc Chips
// ============================================================================

fn dash() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Dash,
        name: "Dash",
        description: "Knock over all in your path!",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 4.0,
        charge_time: 0.2,
        target: ActionTarget::Row {
            x_offset: 0,
            traveling: true,
        },
        effect: ActionEffect::Combo {
            effects: vec![
                ActionEffect::damage(50),
                ActionEffect::Knockback { distance: 1 },
            ],
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::projectile(colors::SWORD_WHITE, colors::SWORD_WHITE),
    }
}

fn guts_punch() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::GutsPnch,
        name: "GutsPnch",
        description: "Knocks stuff over Range=1",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 3.0,
        charge_time: 0.3,
        target: ActionTarget::SingleTile { range: 1 },
        effect: ActionEffect::Combo {
            effects: vec![
                ActionEffect::damage(160),
                ActionEffect::Knockback { distance: 2 },
            ],
        },
        modifiers: ActionModifiers {
            guard_break: true,
            ..default()
        },
        visuals: ActionVisuals::sword_slash(colors::CANNON_ORANGE, colors::CANNON_ORANGE),
    }
}

fn ice_punch() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::IcePunch,
        name: "IcePunch",
        description: "Knocks stuff over Range=1 [Aqua]",
        element: Element::Aqua,
        rarity: Rarity::Uncommon,
        cooldown: 3.5,
        charge_time: 0.3,
        target: ActionTarget::SingleTile { range: 1 },
        effect: ActionEffect::Combo {
            effects: vec![
                ActionEffect::elemental_damage(150, Element::Aqua),
                ActionEffect::Knockback { distance: 2 },
            ],
        },
        modifiers: ActionModifiers {
            guard_break: true,
            ..default()
        },
        visuals: ActionVisuals::sword_slash(colors::AQUA, colors::AQUA),
    }
}

// ============================================================================
// Panel Chips
// ============================================================================

fn area_steal() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Steal,
        name: "Steal",
        description: "Steal left column of enemy area",
        element: Element::None,
        rarity: Rarity::Rare,
        cooldown: 10.0,
        charge_time: 0.0,
        target: ActionTarget::Column { x_offset: 3 }, // First enemy column
        effect: ActionEffect::StealPanel { columns: 1 },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals {
            icon_color: Color::srgb(0.8, 0.2, 0.8),
            effect_color: Color::srgb(0.8, 0.2, 0.8),
            ..default()
        },
    }
}

fn geddon(tier: i32) -> ActionBlueprint {
    ActionBlueprint {
        id: if tier == 1 {
            ActionId::Geddon1
        } else {
            ActionId::Geddon2
        },
        name: if tier == 1 { "Geddon1" } else { "Geddon2" },
        description: if tier == 1 {
            "All panels become cracked!"
        } else {
            "Erases all empty panels"
        },
        element: Element::None,
        rarity: if tier == 1 {
            Rarity::Rare
        } else {
            Rarity::SuperRare
        },
        cooldown: 15.0,
        charge_time: 0.5,
        target: ActionTarget::EnemyArea,
        effect: ActionEffect::CrackPanel {
            crack_only: tier == 1,
        },
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::explosion(
            Color::srgb(0.5, 0.1, 0.5),
            Color::srgb(0.5, 0.1, 0.5),
            Vec2::new(200.0, 200.0),
        ),
    }
}

fn repair() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::Repair,
        name: "Repair",
        description: "Repair panels in your area",
        element: Element::None,
        rarity: Rarity::Common,
        cooldown: 5.0,
        charge_time: 0.0,
        target: ActionTarget::AreaAroundSelf { radius: 3 },
        effect: ActionEffect::RepairPanel,
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::heal(colors::HEAL_GREEN, colors::HEAL_GREEN),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Generate a circular pattern of tiles
fn generate_radius_pattern(radius: i32) -> Vec<(i32, i32)> {
    let mut pattern = vec![(0, 0)];
    for dx in -radius..=radius {
        for dy in -radius..=radius {
            if dx == 0 && dy == 0 {
                continue;
            }
            if dx.abs() + dy.abs() <= radius {
                pattern.push((dx, dy));
            }
        }
    }
    pattern
}
