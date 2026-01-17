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
  - `actions.rs`: Action system (special abilities with cooldowns)
  - `action_ui.rs`: Action bar UI at bottom of screen
  - `enemy_ai.rs`: Legacy enemy AI (deprecated, use enemies/ instead)
- `src/enemies/` **NEW - Composable Enemy System**
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

## Action System
The fighter has 3 action slots. Actions are special abilities separate from the equipped weapon.

### Key bindings
| Slot | Keyboard | Gamepad (planned) | Action |
|------|----------|-------------------|--------|
| 1    | `1`      | A                 | Heal |
| 2    | `2`      | B                 | Shield |
| 3    | `3`      | X                 | WideSword |

### Current actions
1. **Heal** (Slot 1, Key `1`)
   - Instant cast (no charge time)
   - Restores 20 HP (capped at max HP)
   - 8 second cooldown (longer to prevent spam)
   - Visual: Green flash on player

2. **Shield** (Slot 2, Key `2`)
   - Instant activation (no charge time)
   - Blocks all incoming damage for 2 seconds
   - 6 second cooldown after shield expires
   - Visual: Blue semi-transparent shield around player

3. **WideSword** (Slot 3, Key `3`)
   - Quick charge time (0.3s)
   - Melee attack hitting the entire column in front of player (all 3 rows)
   - Deals high damage (40 HP)
   - 4 second cooldown after use
   - Visual: Pink vertical slash effect

### Action states
- `Ready`: Action can be triggered
- `Charging`: Action is charging up (for charged abilities)
- `OnCooldown`: Action was used, waiting for cooldown

### Action bar UI
- Located at bottom center of screen (`ACTION_BAR_Y = -340`)
- Each slot shows:
  - Icon (colored square representing the action)
  - Key binding label below
  - Green dot indicator when ready
  - Dark overlay showing cooldown progress (sweeps down as cooldown finishes)
  - Yellow charge bar during charging

### Adding new actions
1. Add new variant to `ActionType` enum in `components.rs`
2. Add constants in `constants.rs` (cooldown, charge time, damage/effect, colors)
3. Handle the action in `execute_action()` in `actions.rs`
4. Add UI slot in `setup_action_bar()` in `action_ui.rs`
5. Spawn the `ActionSlot` in `spawn_player_actions()`

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
