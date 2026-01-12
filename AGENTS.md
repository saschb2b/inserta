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
  - `enemy_ai.rs`: Enemy movement and shooting AI
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
