# AGENTS.md

This repo is a small Bevy (Rust) prototype for a Mega Man Battle Network–style arena.

## Quick commands
- Run: `cargo run`
- Compile check: `cargo check` (RUN THIS AFTER EVERY CHANGE)

## Development Workflow
1. **Make Changes**: Edit code as requested.
2. **Verify Compilation**: IMMEDIATELY run `cargo check` to catch syntax errors or type mismatches.
3. **Fix Errors**: If `cargo check` fails, fix the errors before proceeding.
4. **Verify Behavior**: Run `cargo run` to ensure runtime behavior is correct.
5. **Update LEARNINGS.md**: Record any noteworthy discoveries (see below).

## Maintaining LEARNINGS.md
**ALWAYS update `LEARNINGS.md` when something noteworthy happens.** This is a
first-class artifact that captures reusable knowledge for future development.

### When to add entries
- **Decisions**: When choosing an approach over alternatives (add to Decisions)
- **APIs/Types**: When introducing or changing a public API/type (add to Interfaces)
- **Bugs/Edge cases**: When encountering a bug or edge case (add to Gotchas)
- **Patterns**: When establishing a repeatable solution (add to Patterns)
- **Scripts/Config**: When adding tooling or commands (add to Tooling)
- **Test strategies**: When establishing test heuristics (add to Metrics and tests)

### Entry format
- Use stable IDs: `DEC-001`, `PAT-002`, `GCH-003`, etc.
- One-sentence summary first, then details.
- Include file paths or commit refs for traceability.
- Never delete entries; mark as `Status: Superseded by DEC-00X` if outdated.

### Example entry
```markdown
### DEC-005: Use X instead of Y
Status: accepted

Summary: One sentence explaining the decision.

Context:
- Why this decision was needed.

Decision:
- What was chosen.

Alternatives:
- What else was considered.

Consequences:
- Impact of this decision.

Refs:
- src/path/to/file.rs
```

## Project structure
- `src/main.rs`: App wiring (plugins + system scheduling)
- `src/constants.rs`: Gameplay + rendering constants
- `src/components.rs`: ECS components/resources
- `src/assets.rs`: Asset handles/resources (sprite sheets)
- `src/systems/`
  - `setup.rs`: Spawns arena, entities, and BGM
  - `common.rs`: Grid → world transform updates (tile-floor based)
  - `grid_utils.rs`: Tile coordinate mapping helpers
  - `player.rs`: Movement input (shooting moved to weapon system)
  - `combat.rs`: Bullet movement + tile-based hits
  - `animation.rs`: Player sprite-sheet animation
  - `actions.rs`: Legacy action systems (deprecated, use actions/ instead)
  - `action_ui.rs`: Action bar UI at bottom of screen
- `src/actions/` **NEW - Composable Action/Chip System**
  - `mod.rs`: ActionsPlugin registration
  - `components.rs`: ActionId, ActionSlot, Element, Rarity, ActiveShield
  - `behaviors.rs`: ActionTarget and ActionEffect enums
  - `blueprints.rs`: ActionBlueprint definitions (add new actions here!)
  - `visuals.rs`: ActionVisuals config and color presets
  - `systems.rs`: Input handling and effect execution
- `src/enemies/` **Composable Enemy System**
  - `mod.rs`: EnemyPlugin registration
  - `components.rs`: EnemyStats, EnemyMovement, EnemyAttack, EnemyTraitContainer
  - `behaviors.rs`: MovementBehavior and AttackBehavior enums
  - `blueprints.rs`: EnemyBlueprint definitions (add new enemies here!)
  - `visuals.rs`: EnemyVisuals and EnemyAnimations config
  - `systems.rs`: Behavior execution systems
- `src/weapons/`
  - `mod.rs`: Weapon system (stats, components, plugin, systems)
  - `blaster.rs`: Blaster weapon implementation

## Core gameplay rules (keep consistent)
- Arena is `3x6` tiles (`GRID_HEIGHT=3`, `GRID_WIDTH=6`).
- Left `3x3` is player territory; movement is restricted to that area.
- Combat is tile/lane based:
  - Bullets spawn from the player's current `GridPosition`.
  - Bullets travel along the same row.
  - Hits are calculated only when `bullet.GridPosition == enemy.GridPosition`.
  - Sprite sizes must not affect hit detection.

## Weapon System
The fighter equips a weapon that handles primary attacks. Weapons have unique characteristics:

### Weapon Stats
- **Damage**: Base damage dealt (can have multiple damage types: Physical, Fire, Ice, Electric, Void)
- **Charge Time**: How quickly a weapon can be charged for heavy attacks
- **Critical Chance/Multiplier**: Chance for critical hits (yellow/orange/red crits at 100%/200%+ chance)
- **Fire Rate**: Cooldown between shots
- **Damage Falloff**: Range where damage decreases (start range, end range, minimum multiplier)
- **Range**: Maximum distance in tiles

### Current Weapon: Blaster
The default starting weapon - a reliable energy pistol that rewards skilled timing.

| Stat | Normal Shot | Charged Shot |
|------|-------------|--------------|
| Damage | 1 | 5 |
| Charge Time | - | 0.6s |
| Fire Cooldown | 0.25s | 0.25s |
| Crit Chance | 8% | 8% |
| Range | 6 tiles | 6 tiles |
| Falloff | None | None |

**Controls:**
- `Space` (tap): Fire single shot immediately
- `Space` (hold): Charge up, release for charged shot
- Releasing early cancels the charge (shorter cooldown)

**Strategy:**
- Use single shots as filler damage while repositioning
- Master charge timing for burst damage opportunities
- Charged shots deal 5x damage - worth the commitment

### Adding New Weapons
1. Create new file in `src/weapons/` (e.g., `spreader.rs`)
2. Implement `weapon_stats()` function returning `WeaponStats`
3. Add variant to `WeaponType` enum in `weapons/mod.rs`
4. Add match arm in `WeaponType::stats()`

## Enemy System (Composable Behaviors)
Enemies are defined using a **blueprint system** - combine stats, movement, attacks, and traits like LEGO blocks.

### Architecture Overview
```
EnemyBlueprint {
    id: EnemyId,           // Unique identifier
    name: &str,            // Display name
    stats: EnemyStats,     // HP, damage, speed multipliers
    movement: MovementBehavior,  // How it moves
    attack: AttackBehavior,      // How it attacks
    traits: EnemyTraits,         // Optional modifiers
    visuals: EnemyVisuals,       // Sprite config
}
```

### Adding a New Enemy (Step by Step)

**Step 1: Add to `EnemyId` enum** (`src/enemies/components.rs`)
```rust
pub enum EnemyId {
    Slime,
    Mettaur,  // <- Add your new enemy here
}
```

**Step 2: Create blueprint function** (`src/enemies/blueprints.rs`)
```rust
fn mettaur_blueprint() -> EnemyBlueprint {
    EnemyBlueprint {
        id: EnemyId::Mettaur,
        name: "Mettaur",
        stats: EnemyStats {
            base_hp: 40,
            contact_damage: 10,
            move_speed: 0.8,
            attack_speed: 1.0,
        },
        movement: MovementBehavior::HideAndPeek {
            hide_duration: 2.0,
            peek_duration: 1.5,
        },
        attack: AttackBehavior::ShockWave {
            damage: 20,
            speed: 6.0,
            charge_time: 0.3,
        },
        traits: EnemyTraits::default(),
        visuals: EnemyVisuals {
            sprite_path: "enemies/mettaur".into(),
            draw_size: Vec2::new(96.0, 96.0),
            anchor: Vec2::new(0.0, -0.35),
            offset: Vec2::ZERO,
            flip_x: true,
            animations: EnemyAnimations::default(),
        },
    }
}
```

**Step 3: Register in `EnemyBlueprint::get()`** (`src/enemies/blueprints.rs`)
```rust
pub fn get(id: EnemyId) -> Self {
    match id {
        EnemyId::Slime => slime_blueprint(),
        EnemyId::Mettaur => mettaur_blueprint(),  // <- Add match arm
    }
}
```

**Step 4: Add sprite assets** (`assets/enemies/mettaur/`)
- `IDLE.png` - Idle animation sprite sheet
- `ATTACK.png` - Attack animation (optional)
- `DEAD.png` - Death animation (optional)

### Available Movement Behaviors
| Behavior | Description |
|----------|-------------|
| `Stationary` | Doesn't move (turrets) |
| `Random { idle_chance }` | Random movement, chance to stay still |
| `ChaseRow` | Moves to match player's Y position |
| `ChasePlayer` | Moves toward player (stays in territory) |
| `PatrolHorizontal` | Patrols left-right |
| `PatrolVertical` | Patrols up-down |
| `HideAndPeek { hide_duration, peek_duration }` | Mettaur-style hide/attack |
| `Teleport { min_interval, max_interval }` | Random teleportation |
| `BackRowOnly` | Stays at back, moves vertically |
| `MirrorPlayer` | Mirrors player Y position |
| `Advance { max_advance }` | Gradually advances toward player |

### Available Attack Behaviors
| Behavior | Description |
|----------|-------------|
| `None` | No attack (contact damage only) |
| `Projectile { damage, speed, charge_time }` | Single projectile |
| `ProjectileSpread { ..., row_offsets }` | Multiple projectiles |
| `ShockWave { damage, speed, charge_time }` | Ground wave attack |
| `Melee { damage, range, charge_time }` | Close range attack |
| `AreaAttack { damage, charge_time, pattern }` | Multi-tile attack |
| `Bomb { damage, fuse_time, radius }` | Delayed explosion |
| `LaserBeam { damage, charge_time, duration }` | Instant row hit |
| `Summon { summon_id, max_summons, charge_time }` | Spawns minions |

### Available Enemy Traits
| Trait | Description |
|-------|-------------|
| `armor: i32` | Flat damage reduction |
| `hp_regen_per_sec: f32` | HP regeneration |
| `super_armor: bool` | Immune to flinching |
| `elemental_resist: f32` | Elemental damage reduction |
| `death_explosion` | Explodes on death |
| `death_spawn` | Spawns minions on death |
| `enrage` | Gets stronger at low HP |
| `phase_immunity` | Periodic invulnerability |

### Spawning Enemies in Battle
Use `EnemyConfig` in `ArenaConfig`:
```rust
ArenaConfig {
    enemies: vec![
        EnemyConfig::new(EnemyId::Slime, 4, 1),           // Default HP
        EnemyConfig::new(EnemyId::Mettaur, 5, 0).with_hp(80),  // Custom HP
    ],
    ..default()
}
```

### Current Limitations
- **Player position**: Movement behaviors that need player position (ChasePlayer, MirrorPlayer) currently don't track the player to avoid query conflicts. Will be fixed with a shared resource.
- **Animation**: Still uses legacy `SlimeAnim` component. Full animation generalization is TODO.
- **Traits**: HP regen and enrage are defined but systems are disabled to avoid query conflicts.

## Action System (Composable Blueprints)
The fighter has action slots for special abilities (Battle Chips). Actions are defined using a **blueprint system** - combine targeting, effects, and visuals like LEGO blocks.

### Architecture Overview
```
ActionBlueprint {
    id: ActionId,              // Unique identifier
    name: &str,                // Display name
    element: Element,          // None, Fire, Aqua, Elec, Wood
    rarity: Rarity,            // Common to UltraRare (* to *****)
    cooldown: f32,             // Seconds after use
    charge_time: f32,          // 0.0 = instant
    target: ActionTarget,      // How it selects targets
    effect: ActionEffect,      // What it does
    modifiers: ActionModifiers,// Optional modifiers
    visuals: ActionVisuals,    // Icon and effect colors
}
```

### Adding a New Action (Step by Step)

**Step 1: Add to `ActionId` enum** (`src/actions/components.rs`)
```rust
pub enum ActionId {
    Recov50,
    Shield,
    WideSwrd,
    MyNewChip,  // <- Add your new action here
}
```

**Step 2: Create blueprint function** (`src/actions/blueprints.rs`)
```rust
fn my_new_chip() -> ActionBlueprint {
    ActionBlueprint {
        id: ActionId::MyNewChip,
        name: "MyNewChip",
        description: "Does something cool!",
        element: Element::Fire,
        rarity: Rarity::Rare,
        cooldown: 5.0,
        charge_time: 0.3,
        target: ActionTarget::Column { x_offset: 1 },
        effect: ActionEffect::elemental_damage(100, Element::Fire),
        modifiers: ActionModifiers::default(),
        visuals: ActionVisuals::sword_slash(colors::FIRE, colors::FIRE),
    }
}
```

**Step 3: Register in `ActionBlueprint::get()`** (`src/actions/blueprints.rs`)
```rust
pub fn get(id: ActionId) -> Self {
    match id {
        ActionId::MyNewChip => my_new_chip(),  // <- Add match arm
        // ...
    }
}
```

### Available Targeting Types
| Target | Description |
|--------|-------------|
| `OnSelf` | Affects the user (heals, buffs) |
| `SingleTile { range }` | Single tile in front |
| `Column { x_offset }` | Entire column (WideSword) |
| `Row { x_offset, traveling }` | Entire row (shockwave) |
| `Pattern { tiles }` | Specific tile pattern (LongSword) |
| `Projectile { x_offset, piercing }` | Traveling projectile |
| `ProjectileSpread { ..., spread_rows }` | Multiple row projectile |
| `AreaAroundSelf { radius }` | Area around user |
| `AreaAtPosition { x_offset, y_offset, pattern }` | Area at target |
| `EnemyArea` | All enemy tiles |
| `RandomEnemy { count }` | Random enemy tiles |

### Available Effect Types
| Effect | Description |
|--------|-------------|
| `Damage { amount, element, can_crit, guard_break }` | Deal damage |
| `Heal { amount }` | Restore HP |
| `Shield { duration, threshold }` | Block damage |
| `Invisibility { duration }` | Complete invincibility |
| `StealPanel { columns }` | Steal enemy panels |
| `CrackPanel { crack_only }` | Crack or destroy panels |
| `RepairPanel` | Fix broken panels |
| `Knockback { distance }` | Push targets back |
| `Stun { duration }` | Freeze targets |
| `Drain { amount }` | Steal HP from target |
| `MultiHit { damage_per_hit, hit_count, element }` | Multiple hits |
| `Delayed { delay, effect }` | Bomb-style delayed effect |
| `Combo { effects }` | Multiple effects combined |

### Current Default Actions
| Key | Action | Description |
|-----|--------|-------------|
| `1` | Recov50 | Instant heal 50 HP (5s cooldown) |
| `2` | Shield | Block all damage 2s (6s cooldown) |
| `3` | WideSwrd | Column attack 80 dmg (4s cooldown, 0.3s charge) |

### Key bindings
| Slot | Keyboard | Gamepad |
|------|----------|---------|
| 1    | `1`      | West (X) |
| 2    | `2`      | North (Y) |
| 3    | `3`      | East (B) |
| 4    | `4`      | South (A) |

### Action bar UI
- Located at bottom center of screen
- Each slot shows:
  - Icon (colored based on blueprint)
  - Key binding label below
  - Green dot indicator when ready
  - Dark overlay showing cooldown progress
  - Yellow charge bar during charging

### Available MMBN-Style Actions (70+ defined)
See `src/actions/blueprints.rs` for the full list including:
- **Recovery**: Recov10-300
- **Defense**: Barrier, Shield, MetGuard, Invis1-3, LifeAura
- **Swords**: Sword, WideSwrd, LongSwrd, FireSwrd, AquaSwrd, ElecSwrd, FtrSwrd, KngtSwrd, HeroSwrd
- **Cannons**: Cannon, HiCannon, M-Cannon
- **Bombs**: MiniBomb, LilBomb, CrosBomb, BigBomb
- **Waves**: ShokWave, SoniWave, DynaWave
- **Towers**: FireTowr, AquaTowr, WoodTowr
- And many more...

## Rendering rules
- Use `tile_floor_world(x,y)` for positioning sprites that stand on panels (feet snapping).
- Tile meshes are placed using `tile_center_world(x,y)`.
- Sprites use `Anchor` to correct for padding in the source frames.
- Keep visual offsets in `src/constants.rs` so tuning is centralized.

## Assets
- Fighter sprites live in `assets/characters/fighter/`.
- Enemy sprites live in `assets/enemies/`.
- Background music: `assets/audio/bgm/`.

## Audio
- MP3 decoding requires Bevy feature `mp3` (already enabled in `Cargo.toml`).

## Style guidelines
- Prefer small, focused systems.
- Keep gameplay logic (grid/tile rules) separate from rendering offsets.
- Avoid hardcoding asset paths in multiple places; put them in `setup.rs`/`assets.rs`.
