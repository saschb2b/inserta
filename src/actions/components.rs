// ============================================================================
// Action Components - ECS components for the action/chip system
// ============================================================================

use bevy::prelude::*;

/// Unique identifier for action types (like Battle Chip IDs)
/// Add new actions here!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActionId {
    // Recovery chips
    #[default]
    Recov10,
    Recov30,
    Recov50,
    Recov80,
    Recov120,
    Recov150,
    Recov200,
    Recov300,

    // Defense chips
    Barrier,
    Shield, // Our current shield
    MetGuard,
    Invis1,
    Invis2,
    Invis3,
    LifeAura,

    // Sword chips
    Sword,
    WideSwrd,
    LongSwrd,
    FireSwrd,
    AquaSwrd,
    ElecSwrd,
    FtrSwrd,
    KngtSwrd,
    HeroSwrd,

    // Cannon chips
    Cannon,
    HiCannon,
    MCannon,

    // Bomb chips
    MiniBomb,
    LilBomb,
    CrosBomb,
    BigBomb,

    // Wave/Ground attacks
    ShokWave,
    SoniWave,
    DynaWave,

    // Spread attacks
    Shotgun,
    Spreader,
    Bubbler,

    // Tower attacks (hit column, can move up/down)
    FireTowr,
    AquaTowr,
    WoodTowr,

    // Area attacks
    Quake1,
    Quake2,
    Quake3,

    // Thunder (rolling ball)
    Thunder1,
    Thunder2,
    Thunder3,

    // Misc attacks
    Ratton1,
    Ratton2,
    Ratton3,
    Dash,
    GutsPnch,
    IcePunch,

    // Panel manipulation
    Steal,
    Geddon1,
    Geddon2,
    Repair,
}

/// Element type for actions (affects damage and weaknesses)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Element {
    #[default]
    None,
    Fire,
    Aqua,
    Elec,
    Wood,
}

impl Element {
    /// Get element that this element is strong against
    pub fn strong_against(&self) -> Option<Element> {
        match self {
            Element::Fire => Some(Element::Wood),
            Element::Aqua => Some(Element::Fire),
            Element::Elec => Some(Element::Aqua),
            Element::Wood => Some(Element::Elec),
            Element::None => None,
        }
    }

    /// Get element that this element is weak to
    pub fn weak_to(&self) -> Option<Element> {
        match self {
            Element::Fire => Some(Element::Aqua),
            Element::Aqua => Some(Element::Elec),
            Element::Elec => Some(Element::Wood),
            Element::Wood => Some(Element::Fire),
            Element::None => None,
        }
    }
}

/// Rarity of an action (affects availability/power)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Rarity {
    #[default]
    Common, // *
    Uncommon,  // **
    Rare,      // ***
    SuperRare, // ****
    UltraRare, // *****
}

/// State of an action slot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionState {
    #[default]
    Ready,
    Charging,
    OnCooldown,
}

/// An action slot component - represents one equipped action
#[derive(Component)]
pub struct ActionSlot {
    /// Which slot this is (0, 1, 2, ...)
    pub slot_index: usize,
    /// The action equipped in this slot
    pub action_id: ActionId,
    /// Current state
    pub state: ActionState,
    /// Cooldown timer
    pub cooldown_timer: Timer,
    /// Charge timer (for charged actions)
    pub charge_timer: Option<Timer>,
    /// Cached cooldown duration
    pub cooldown_duration: f32,
    /// Cached charge duration
    pub charge_duration: f32,
}

impl ActionSlot {
    pub fn new(slot_index: usize, action_id: ActionId, cooldown: f32, charge: f32) -> Self {
        Self {
            slot_index,
            action_id,
            state: ActionState::Ready,
            cooldown_timer: Timer::from_seconds(cooldown, TimerMode::Once),
            charge_timer: None,
            cooldown_duration: cooldown,
            charge_duration: charge,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == ActionState::Ready
    }

    pub fn start_charging(&mut self) {
        if self.charge_duration > 0.0 {
            self.state = ActionState::Charging;
            self.charge_timer = Some(Timer::from_seconds(self.charge_duration, TimerMode::Once));
        }
    }

    pub fn start_cooldown(&mut self) {
        self.state = ActionState::OnCooldown;
        self.cooldown_timer = Timer::from_seconds(self.cooldown_duration, TimerMode::Once);
        self.charge_timer = None;
    }

    pub fn cooldown_progress(&self) -> f32 {
        if self.state == ActionState::OnCooldown {
            self.cooldown_timer.fraction()
        } else {
            1.0
        }
    }

    pub fn charge_progress(&self) -> f32 {
        if let Some(ref timer) = self.charge_timer {
            timer.fraction()
        } else {
            0.0
        }
    }
}

/// Marker for a pending action execution
#[derive(Component)]
pub struct PendingAction {
    pub action_id: ActionId,
    pub source_entity: Entity,
    pub source_position: (i32, i32),
}

/// Active shield effect on an entity
#[derive(Component)]
pub struct ActiveShield {
    /// Duration remaining
    pub duration_timer: Timer,
    /// Damage threshold (for auras)
    pub damage_threshold: Option<i32>,
    /// Shield type for visuals
    pub shield_type: ShieldType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShieldType {
    /// Basic shield - blocks all damage
    Basic,
    /// Barrier - blocks one hit
    Barrier,
    /// Aura - blocks damage under threshold
    Aura,
    /// Invisibility - complete invincibility
    Invis,
}

/// Marker for action visual effects (slashes, projectiles, etc.)
#[derive(Component)]
pub struct ActionVisual {
    /// Lifetime timer for auto-despawn
    pub lifetime: Timer,
    /// Entity that created this visual
    pub source: Option<Entity>,
}

/// Component for damage zones (sword slashes, explosions, etc.)
#[derive(Component)]
pub struct DamageZone {
    pub damage: i32,
    pub element: Element,
    /// Tiles that will be hit
    pub hit_tiles: Vec<(i32, i32)>,
    /// Whether damage has been applied (prevents double-hit)
    pub applied: bool,
}

/// Component for projectiles spawned by actions
#[derive(Component)]
pub struct ActionProjectile {
    pub damage: i32,
    pub element: Element,
    /// Speed in tiles per second
    pub speed: f32,
    /// Direction of travel
    pub direction: ProjectileDirection,
    /// Whether it pierces enemies
    pub piercing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectileDirection {
    /// Travels horizontally toward enemy side
    Forward,
    /// Travels horizontally toward player side
    Backward,
    /// Travels along the ground (shockwave style)
    Ground,
    /// Homes toward nearest enemy
    Homing,
}

/// Marker for heal flash effect
#[derive(Component)]
pub struct HealFlash {
    pub timer: Timer,
    pub heal_amount: i32,
}
