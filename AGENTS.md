# AGENTS.md

This repo is a small Bevy (Rust) prototype for a Mega Man Battle Network–style arena.

## Quick commands
- Run: `cargo run`
- Compile check: `cargo check`

## Project structure
- `src/main.rs`: App wiring (plugins + system scheduling)
- `src/constants.rs`: Gameplay + rendering constants
- `src/components.rs`: ECS components/resources
- `src/assets.rs`: Asset handles/resources (sprite sheets)
- `src/systems/`
  - `setup.rs`: Spawns arena, entities, and BGM
  - `common.rs`: Grid → world transform updates (tile-floor based)
  - `grid_utils.rs`: Tile coordinate mapping helpers
  - `player.rs`: Movement + shooting input
  - `combat.rs`: Bullet movement + tile-based hits
  - `animation.rs`: Player sprite-sheet animation
  - `actions.rs`: Action system (special abilities with cooldowns)
  - `action_ui.rs`: Action bar UI at bottom of screen
  - `enemy_ai.rs`: Enemy movement and shooting AI

## Core gameplay rules (keep consistent)
- Arena is `3x6` tiles (`GRID_HEIGHT=3`, `GRID_WIDTH=6`).
- Left `3x3` is player territory; movement is restricted to that area.
- Combat is tile/lane based:
  - Bullets spawn from the player's current `GridPosition`.
  - Bullets travel along the same row.
  - Hits are calculated only when `bullet.GridPosition == enemy.GridPosition`.
  - Sprite sizes must not affect hit detection.

## Action System
The fighter has up to 4 action slots (currently 2 implemented). Actions are special abilities with cooldowns.

### Key bindings
| Slot | Keyboard | Gamepad (planned) | Action |
|------|----------|-------------------|--------|
| 1    | `1`      | A                 | Charged Shot |
| 2    | `2`      | B                 | Heal |
| 3    | `3`      | X                 | (empty) |
| 4    | `4`      | Y                 | (empty) |

### Current actions
1. **Charged Shot** (Slot 1, Key `1`)
   - Requires charge-up time before firing (0.8s)
   - Deals high damage (25 HP)
   - 3 second cooldown after use
   - Visual: Orange projectile, larger than normal bullet

2. **Heal** (Slot 2, Key `2`)
   - Instant cast (no charge time)
   - Restores 20 HP (capped at max HP)
   - 8 second cooldown (longer to prevent spam)
   - Visual: Green flash on player

### Action states
- `Ready`: Action can be triggered
- `Charging`: Action is charging up (for charged abilities)
- `OnCooldown`: Action was used, waiting for cooldown

### Action bar UI
- Located at bottom center of screen (`ACTION_BAR_Y = -250`)
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
