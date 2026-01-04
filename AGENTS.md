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

## Core gameplay rules (keep consistent)
- Arena is `3x6` tiles (`GRID_HEIGHT=3`, `GRID_WIDTH=6`).
- Left `3x3` is player territory; movement is restricted to that area.
- Combat is tile/lane based:
  - Bullets spawn from the player’s current `GridPosition`.
  - Bullets travel along the same row.
  - Hits are calculated only when `bullet.GridPosition == enemy.GridPosition`.
  - Sprite sizes must not affect hit detection.

## Rendering rules
- Use `tile_floor_world(x,y)` for positioning sprites that stand on panels (feet snapping).
- Tile meshes are placed using `tile_center_world(x,y)`.
- Sprites use `Anchor` to correct for padding in the source frames.
- Keep visual offsets in `src/constants.rs` so tuning is centralized.

## Assets
- Fighter sprites live in `assets/characters/fighter/`.
- Background music: `assets/characters/fighter/audio/bgm/`.

## Audio
- MP3 decoding requires Bevy feature `mp3` (already enabled in `Cargo.toml`).

## Style guidelines
- Prefer small, focused systems.
- Keep gameplay logic (grid/tile rules) separate from rendering offsets.
- Avoid hardcoding asset paths in multiple places; put them in `setup.rs`/`assets.rs`.
