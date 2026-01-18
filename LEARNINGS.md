# LEARNINGS

This document records reusable knowledge discovered in this codebase.
Only "reusable knowledge" belongs here - no transient TODOs or changelog noise.

## Context

- Problem: Building a Mega Man Battle Network-style arena game in Bevy/Rust
  with composable enemy behaviors, weapon systems, and action abilities.
- Constraints: Bevy ECS patterns, tile-based combat, parallel system
  scheduling conflicts, sprite-based animation.
- Non-goals: 3D rendering, networking, complex AI pathfinding.

## Decisions

### DEC-001: Composable enemy behaviors via enums over trait objects
Status: accepted

Summary: Use `MovementBehavior` and `AttackBehavior` enums with pattern
matching instead of trait objects for enemy AI.

Context:
- Needed a "LEGO-like" system for defining enemies by combining behaviors.
- Considered trait-based polymorphism (`Box<dyn MovementBehavior>`).
- Bevy ECS prefers data-driven components over OOP inheritance.

Decision:
- Define `MovementBehavior` and `AttackBehavior` as enums with variants for
  each behavior type (Random, ChasePlayer, Projectile, Melee, etc.).
- Store these in `EnemyMovement` and `EnemyAttack` components.
- Use pattern matching in systems to execute behavior-specific logic.

Alternatives:
- Trait objects: More flexible but harder to serialize, clone, and debug.
- Separate components per behavior: Explosion of component types.
- State machines: More complex, overkill for current needs.

Consequences:
- All behaviors defined in one place, easy to see all options.
- Adding new behavior = add enum variant + match arm.
- No dynamic dispatch overhead.
- Cannot add behaviors at runtime without recompiling.

Refs:
- src/enemies/behaviors.rs
- src/enemies/systems.rs

---

### DEC-002: Blueprint pattern for enemy definitions
Status: accepted

Summary: Use `EnemyBlueprint` structs to define complete enemy templates that
combine stats, behaviors, traits, and visuals.

Context:
- Spawning enemies required setting 10+ components manually.
- Easy to forget components or misconfigure them.
- Wanted a single source of truth per enemy type.

Decision:
- Create `EnemyBlueprint` struct containing all enemy configuration.
- Define blueprint functions per enemy type (e.g., `slime_blueprint()`).
- `EnemyBlueprint::get(id)` returns the complete definition.
- Spawn function reads blueprint and creates all components.

Alternatives:
- Asset files (RON/JSON): More flexible but requires asset loading pipeline.
- Builder pattern: More verbose, still code-based.
- Prefabs: Bevy doesn't have native prefab support yet.

Consequences:
- Adding enemy = 1 function + 1 match arm.
- All enemy data visible in one file.
- Type-safe, compile-time checked.
- Not hot-reloadable (requires recompile).

Refs:
- src/enemies/blueprints.rs
- src/enemies/components.rs:EnemyId

---

### DEC-003: Chain systems to avoid Bevy query conflicts
Status: accepted

Summary: Use `.chain()` to run systems sequentially when they access the same
components mutably, rather than adding complex filter combinations.

Context:
- Bevy's parallel scheduler detects potential conflicts at initialization.
- Multiple systems needed `&mut Sprite` on enemy entities.
- `Without<T>` filters help but don't always prevent static conflict detection.

Decision:
- Group systems that access the same mutable components.
- Use `.chain()` to force sequential execution.
- Accept minor performance cost for simpler conflict resolution.

Alternatives:
- Complex `Without<T>` filter chains: Hard to maintain, error-prone.
- `ParamSet`: More complex API, harder to read.
- Single monolithic system: Violates single-responsibility.

Consequences:
- Eliminated all query conflict panics.
- Slightly reduced parallelism (acceptable for game scale).
- Clear ownership: each system knows what it can touch.

Refs:
- src/main.rs:119-134 (chained animation systems)
- src/enemies/mod.rs:45-55 (chained enemy systems)

---

### DEC-004: Marker components for system filtering
Status: accepted

Summary: Use marker components (`BehaviorEnemy`, `ChargingTelegraph`) to
distinguish entity subsets for system queries.

Context:
- Old enemy AI and new behavior system needed to coexist during migration.
- Some effects (charging flash) should only apply during specific states.
- Need to exclude entities from certain systems without complex logic.

Decision:
- Add `BehaviorEnemy` marker to enemies using new system.
- Add `ChargingTelegraph` component during charge phase.
- Use `With<T>` and `Without<T>` filters to route entities to correct systems.

Alternatives:
- Boolean flags in components: Requires checking in every system.
- State enum: Single component but complex matching.
- Separate entity types: Breaks shared behavior.

Consequences:
- Clean system separation via query filters.
- Zero-cost at runtime (just archetype filtering).
- Easy to add new markers for new states.

Refs:
- src/enemies/components.rs:BehaviorEnemy
- src/enemies/components.rs:ChargingTelegraph
- src/systems/animation.rs:130 (Without filters)

### DEC-005: Sprite-based tiles with lip overlap
Status: accepted

Summary: Render arena tiles using PNG sprites with a "lip" that overlaps
the row below, rather than procedurally generated meshes.

Context:
- Original tiles used complex mesh generation (rounded rects, frames, highlights).
- Wanted artist-friendly tile customization via PNG assets.
- MMBN tiles have a 3D lip effect where front rows partially obscure back rows.

Decision:
- Use PNG tile sprites (`tile_red.png`, `tile_blue.png`) at 240x190 pixels.
- Bottom 48px is the "lip" that overlaps the tile below.
- `TILE_VISIBLE_HEIGHT = TILE_ASSET_HEIGHT - TILE_LIP_HEIGHT` for row spacing.
- Render back rows first (higher y) so front rows overlap correctly via z-ordering.
- Character floor point is at the center of the visible area (above lip).

Alternatives:
- Keep procedural meshes: Harder to customize visually.
- 3D rendering: Overkill for 2D game.
- No overlap: Loses the 3D depth effect.

Consequences:
- Artists can edit tiles in any image editor.
- Tile dimensions are configurable via constants.
- Z-ordering critical: back rows must render first.
- Character positioning requires accounting for lip offset.

Refs:
- src/constants.rs:23-49 (tile asset constants)
- src/systems/grid_utils.rs (tile positioning functions)
- src/systems/arena.rs:spawn_tile_panels()
- assets/battle/arena/tile_red.png, tile_blue.png

---

### DEC-006: Responsive arena scaling via ArenaLayout resource
Status: accepted

Summary: Use an `ArenaLayout` resource computed from window dimensions to make
the arena fill the screen width while maintaining tile aspect ratio.

Context:
- Arena was hardcoded to 1280x800 resolution.
- Needed tiles to fill full screen width regardless of window size.
- Character and effect sizes should scale proportionally.

Decision:
- Create `ArenaLayout` resource with computed tile dimensions.
- Tile width = screen_width / GRID_WIDTH (fills screen).
- Scale factor = tile_width / TILE_ASSET_WIDTH.
- All other dimensions scaled by this factor.
- Layout computed at arena setup from window dimensions.
- Systems use `ArenaLayout` methods for world positions.

Alternatives:
- Camera zoom: Doesn't truly fill width, may cut off edges.
- Multiple resolution presets: Tedious, not truly responsive.
- Letterboxing: Wastes screen space.

Consequences:
- Arena always fills screen width.
- Tile aspect ratio preserved (no stretching).
- Characters and effects scale proportionally.
- Layout must be passed to spawn functions.
- Need to handle window resize for runtime changes (TODO).

Refs:
- src/resources.rs:ArenaLayout
- src/systems/setup.rs:52-58 (layout initialization)
- src/systems/common.rs:update_transforms()
- src/systems/arena.rs:spawn_tile_panels()

---

## Interfaces

### INT-001: EnemyBlueprint
Summary: Complete template for spawning an enemy type.

```rust
pub struct EnemyBlueprint {
    pub id: EnemyId,
    pub name: &'static str,
    pub stats: EnemyStats,
    pub movement: MovementBehavior,
    pub attack: AttackBehavior,
    pub traits: EnemyTraits,
    pub visuals: EnemyVisuals,
}

impl EnemyBlueprint {
    pub fn get(id: EnemyId) -> Self;
    pub fn scaled_hp(&self, level: i32) -> i32;
}
```

Invariants:
- Every `EnemyId` variant must have a corresponding blueprint function.
- `get()` must be exhaustive over all `EnemyId` variants.
- `scaled_hp()` returns `base_hp + level * 50` by default.

Refs:
- src/enemies/blueprints.rs

---

### INT-002: EnemyConfig
Summary: Configuration for spawning a specific enemy instance in battle.

```rust
pub struct EnemyConfig {
    pub enemy_id: EnemyId,
    pub start_x: i32,
    pub start_y: i32,
    pub hp_override: Option<i32>,
}

impl EnemyConfig {
    pub fn new(enemy_id: EnemyId, x: i32, y: i32) -> Self;
    pub fn with_hp(self, hp: i32) -> Self;
}
```

Invariants:
- `start_x` must be in enemy territory: `PLAYER_AREA_WIDTH..GRID_WIDTH`.
- `start_y` must be in grid: `0..GRID_HEIGHT`.
- If `hp_override` is None, HP is calculated from blueprint.

Refs:
- src/components.rs:46-82

---

### INT-003: MovementBehavior
Summary: Enum defining how an enemy moves in the arena.

```rust
pub enum MovementBehavior {
    Stationary,
    Random { idle_chance: f32 },
    ChaseRow,
    ChasePlayer,
    PatrolHorizontal,
    PatrolVertical,
    HideAndPeek { hide_duration: f32, peek_duration: f32 },
    Teleport { min_interval: f32, max_interval: f32 },
    BackRowOnly,
    MirrorPlayer,
    Advance { max_advance: i32 },
}
```

Invariants:
- `idle_chance` should be 0.0-1.0.
- Movement is constrained to enemy territory (columns 3-5).
- `base_cooldown()` returns movement interval in seconds.

Refs:
- src/enemies/behaviors.rs:12-60

---

### INT-004: AttackBehavior
Summary: Enum defining how an enemy attacks.

```rust
pub enum AttackBehavior {
    None,
    Projectile { damage: i32, speed: f32, charge_time: f32 },
    ProjectileSpread { damage: i32, speed: f32, charge_time: f32,
                       count: i32, row_offsets: Vec<i32> },
    ShockWave { damage: i32, speed: f32, charge_time: f32 },
    Melee { damage: i32, range: i32, charge_time: f32 },
    AreaAttack { damage: i32, charge_time: f32, pattern: Vec<(i32, i32)> },
    Bomb { damage: i32, fuse_time: f32, radius: i32 },
    LaserBeam { damage: i32, charge_time: f32, duration: f32 },
    Summon { summon_id: String, max_summons: i32, charge_time: f32 },
}
```

Invariants:
- `charge_time` is telegraph duration before attack executes.
- `speed` is tiles per second for projectiles.
- `base_cooldown()` returns time between attacks.

Refs:
- src/enemies/behaviors.rs:88-165

---

### INT-005: TargetsTiles
Summary: Component for entities that highlight tiles they target.

```rust
pub struct TargetsTiles {
    pub tiles: Vec<(i32, i32)>,      // Explicit tiles (for multi-tile attacks)
    pub use_grid_position: bool,     // If true, use entity's GridPosition
}

impl TargetsTiles {
    pub fn single() -> Self;                      // Use GridPosition
    pub fn multiple(tiles: Vec<(i32, i32)>) -> Self;  // Use explicit list
}
```

Invariants:
- If `use_grid_position` is true, entity must have `GridPosition` component.
- If `use_grid_position` is false, `tiles` should be non-empty.
- Tiles outside the grid are safely ignored by the highlight system.

Refs:
- src/components.rs:TargetsTiles
- src/systems/combat.rs:tile_attack_highlight()

## Patterns

### PAT-001: Component insertion in batches to avoid tuple limits
Summary: Bevy has a tuple size limit (~15) for component bundles. Split large
spawns into initial spawn + `.insert()` calls.

Details:
- Bevy's `spawn()` accepts a tuple of components as a bundle.
- Tuples larger than ~15 elements fail to compile with cryptic errors.
- Solution: spawn with essential components, then `.insert()` the rest.

```rust
let entity = commands.spawn((
    Sprite { .. },
    Transform::default(),
    GridPosition { .. },
    // ... up to ~12 components
)).id();

commands.entity(entity).insert((
    EnemyStats { .. },
    EnemyMovement { .. },
    EnemyAttack { .. },
    // ... remaining components
));
```

Refs:
- src/systems/setup.rs:219-270

---

### PAT-002: State-based component insertion for visual effects
Summary: Add temporary components to trigger time-limited visual effects,
remove them when the effect ends.

Details:
- Instead of boolean flags or timers in existing components, spawn a new
  component like `ChargingTelegraph` or `FlashTimer`.
- A dedicated system queries for this component and applies the effect.
- System removes the component when done.

```rust
// Start effect
commands.entity(entity).insert(ChargingTelegraph {
    timer: Timer::from_seconds(0.5, TimerMode::Once),
});

// In system
for (entity, mut sprite, mut telegraph) in &mut query {
    telegraph.timer.tick(time.delta());
    if telegraph.timer.just_finished() {
        commands.entity(entity).remove::<ChargingTelegraph>();
    }
}
```

Refs:
- src/enemies/components.rs:ChargingTelegraph
- src/enemies/systems.rs:363-385

---

### PAT-003: Default trait implementations for behavior enums
Summary: Implement `Default` manually for enums with variants containing data.

Details:
- Rust's `#[derive(Default)]` only works on unit variants.
- For enums where the default variant has fields, implement manually.

```rust
// Won't compile:
#[derive(Default)]
enum Foo {
    #[default]
    Bar { x: i32 },  // Error: not a unit variant
}

// Solution:
impl Default for Foo {
    fn default() -> Self {
        Foo::Bar { x: 42 }
    }
}
```

Refs:
- src/enemies/behaviors.rs:62-66
- src/enemies/behaviors.rs:167-174

---

### PAT-004: Smooth texture transitions via intensity tracking
Summary: Use an intensity component to smoothly transition between texture
variants (e.g., normal/highlighted) with fade effects.

Details:
- Store `intensity` (current) and `target` (desired) in a component.
- Each frame, move intensity toward target at a constant speed.
- Swap textures at a threshold (e.g., 50% intensity).
- Apply alpha fade during transition for visual smoothness.

```rust
#[derive(Component)]
pub struct TileHighlightState {
    pub intensity: f32,  // 0.0 = normal, 1.0 = highlighted
    pub target: f32,
    pub is_player_side: bool,
}

// In system:
highlight.target = if has_bullet { 1.0 } else { 0.0 };
let direction = (highlight.target - highlight.intensity).signum();
highlight.intensity += direction * FADE_SPEED * dt;
highlight.intensity = highlight.intensity.clamp(0.0, 1.0);

let use_highlighted = highlight.intensity > 0.5;
sprite.image = if use_highlighted { highlighted_tex } else { normal_tex };
```

Refs:
- src/components.rs:TileHighlightState
- src/systems/combat.rs:bullet_tile_highlight()

## Gotchas

### GCH-001: Bevy query conflicts are detected at system initialization
Summary: Query conflicts panic at startup, not when entities actually overlap.

Details:
- Bevy analyzes query access patterns when building the schedule.
- If two systems COULD access the same component mutably on the same entity
  (based on their filters), Bevy panics immediately.
- Even if the filters are logically disjoint (`With<A>` vs `Without<A>`),
  Bevy may not prove this statically.
- The error message says "Enable the debug feature to see the name" which
  requires compiling with `bevy/trace` feature.

Debug steps:
1. Run with `cargo run --features bevy/trace` to see system names.
2. Search for all systems querying the conflicting component with `&mut`.
3. Check if their filters are truly disjoint.
4. Either add explicit `Without<T>` filters or chain the systems.

Refs:
- src/main.rs:119-134 (fix example)
- src/systems/animation.rs:130 (filter example)

---

### GCH-002: Reading another entity's component in a query causes conflicts
Summary: A system with `Query<&T, With<A>>` and `Query<&mut T, With<B>>`
conflicts even if A and B are different markers.

Details:
- `execute_movement_behavior` had `player_query: Query<&GridPosition, With<Player>>`
  and `enemy_query: Query<&mut GridPosition, With<BehaviorEnemy>>`.
- `move_player` had `Query<&mut GridPosition, With<Player>>`.
- Bevy saw both systems accessing `GridPosition` (one read, one write on
  Player entities) and flagged a conflict.
- Solution: Remove the read query and use a different mechanism (e.g.,
  shared resource) to communicate player position.

Refs:
- src/enemies/systems.rs:23-31

---

### GCH-003: Timer methods changed between Bevy versions
Summary: `Timer::finished()` is a field, not a method. Use `just_finished()`.

Details:
- Common mistake: `timer.finished()` - this accesses a private field.
- Correct: `timer.just_finished()` - returns true on the tick it completes.
- Also: `timer.elapsed_secs()` for how long it's been running.

Refs:
- src/enemies/systems.rs:380

## Tooling

### TLG-001: Development commands
Summary: Key commands for development workflow.

Details:
- `cargo check` - Fast type checking, run after every change.
- `cargo run` - Build and run the game.
- `cargo run --features bevy/trace` - Run with detailed system names in errors.
- `RUST_BACKTRACE=1 cargo run` - Run with backtraces for panics.

Refs:
- AGENTS.md:6-7

---

### TLG-002: Asset paths convention
Summary: Asset paths are relative to `assets/` directory.

Details:
- Fighter sprites: `assets/characters/fighter/`
- Enemy sprites: `assets/enemies/{enemy_name}/`
- Expected files per enemy: `IDLE.png`, `ATTACK.png` (optional), `DEAD.png`
  (optional).
- Audio: `assets/audio/bgm/`
- Arena tiles: `assets/battle/arena/tile_red.png`, `tile_blue.png`
  (240x190 pixels, 48px lip at bottom)

Refs:
- AGENTS.md:139-142

## Metrics and tests

### TST-001: Manual testing workflow
Summary: No automated tests yet. Manual testing via gameplay.

Details:
- Start game, enter battle from menu.
- Verify enemies spawn and move.
- Verify enemies shoot projectiles with red flash telegraph.
- Verify player can damage and kill enemies.
- Verify victory condition triggers when all enemies dead.

Refs:
- (No automated test files yet)

---

### DEC-007: Sprite-based projectile animations
Status: accepted

Summary: Replace mesh projectiles with 4-frame sprite animations using texture atlases.

Context:
- Original projectiles were simple colored squares (BULLET_DRAW_SIZE meshes).
- Needed artistic control for attack visuals with proper launch/travel/impact/finish sequence.
- Player and enemy projectiles both use sprite system for consistency.

Decision:
- Add ProjectileAnimation component tracking animation state and timers.
- Create ProjectileSprites resource with blaster and blaster_charged variants.
- Use 4-frame texture atlases (64x16px spritesheets).
- Implement state transitions: Launch→Travel→Impact→Finish with timing.

Alternatives:
- Keep colored meshes: Simpler but no artistic control.
- Single sprite with UV animation: More complex manual timing.
- Custom animation curves: Overkill for current needs.

Consequences:
- Full artistic control over projectile visuals.
- Charged and normal projectiles can have different sprites.
- Consistent animation system for all projectile types.
- Projectiles stop moving on hit (ProjectileImmobile component).

Refs:
- src/assets.rs:ProjectileSprites, ProjectileAnimation
- src/weapons/mod.rs:spawn_projectile() changes
- src/systems/combat.rs:projectile_animation_system()
- assets/battle/attacks/projectile/blaster_charged.png

---

### DEC-008: Projectile collision hit prevention
Status: accepted

Summary: Prevent projectiles from repeatedly damaging enemies by using Without<ProjectileHit> filter.

Context:
- After implementing sprite animations, projectiles stayed at same GridPosition as enemy.
- projectile_hit_system ran every frame causing continuous damage.
- Need to exclude projectiles that have already hit to prevent re-hits.

Decision:
- Add ProjectileHit marker component when projectile first hits enemy.
- Add ProjectileImmobile component to stop movement during animation.
- Update projectile_hit_system query: Without<ProjectileHit> filter.
- Maintain movement systems that skip immobile projectiles.

Alternatives:
- State flags in Projectile component: Requires checking all collision systems.
- Cooldown on entity: Could prevent re-firing but more complex.
- Collision layers: Overkill for simple grid-based combat.

Consequences:
- Single projectiles deal damage exactly once.
- Clean animation cycle (impact → finish → despawn).
- No performance impact (simple component filter).
- Easier to debug - hit state is explicit marker.

Refs:
- src/components.rs:ProjectileHit, ProjectileImmobile
- src/weapons/mod.rs:projectile_hit_system() query filters
- src/systems/combat.rs:bullet/enemy movement updates

---

### DEC-009: Charged projectile sprite support
Status: accepted

Summary: Differentiate normal and charged blaster projectiles using separate sprites.

Context:
- Added blaster_charged.png asset (64x16, 4 frames for charged shots).
- Need to preserve is_charged flag through projectile lifecycle.
- Animation system must select correct sprite based on projectile type.

Decision:
- Extend ProjectileSprites with blaster_charged_image and blaster_charged_layout.
- Add is_charged field to ProjectileAnimation component.
- Update ProjectileAnimation::blaster() to accept is_charged parameter.
- spawn_projectile() chooses sprite/layout based on is_charged flag.
- Animation system uses is_charged to select correct sprite resource.

Alternatives:
- UV offset manipulation: Complex to manage, hard to debug.
- Color tinting: Simpler but loses visual uniqueness.
- State machines: Overkill for simple frame sequence.

Consequences:
- Normal and charged shots visually distinct.
- Consistent 4-frame animation for both types.
- Easy to extend to other weapon types (add new sprite assets).
- Maintains existing damage calculation (is_charged flag independent).

Refs:
- src/assets.rs:ProjectileSprites, ProjectileAnimation extensions
- src/systems/setup.rs:charged sprite loading
- src/weapons/mod.rs:is_charged parameter passing
- assets/battle/attacks/projectile/blaster_charged.png

---

### DEC-010: Campaign system with arc-based battle progression
Status: accepted

Summary: Implement campaign mode with 10-battle arcs, player-controlled battle selection,
and persistent progress tracking.

Context:
- Original game loop went directly from menu to battle to shop in a forced cycle.
- Wanted player agency: choose which battle to play, grind completed battles.
- MMBN games have arc-based progression with unlockable content.

Decision:
- Add `GameState::Campaign` between MainMenu and Playing.
- Create `CampaignProgress` resource tracking unlocked arcs and completed battles.
- Create `SelectedBattle` resource to remember which battle player chose.
- Define arcs via `ArcDef` and battles via `BattleDef` in resources.rs.
- Arc 1 "Slime Invasion" has 10 battles with escalating difficulty.
- Battle 10 is always a boss battle; completing it unlocks next arc.
- Victory returns to Campaign screen (not Shop) and marks battle complete.

Alternatives:
- Linear progression: Less player agency, can't grind.
- Random battles: No sense of progression.
- World map: More complex UI, overkill for current scope.

Consequences:
- Player controls pace of progression.
- Can replay any completed battle for currency grinding.
- Boss battles gate arc progression.
- Easy to add new arcs (just add to get_all_arcs()).
- Menu now has Campaign and Shop buttons.

Refs:
- src/resources.rs:CampaignProgress, SelectedBattle, BattleDef, ArcDef
- src/systems/campaign.rs
- src/systems/menu.rs:MenuAction::Campaign, MenuAction::Shop
- src/systems/combat.rs:check_victory_condition() updates

---

### INT-006: Campaign Battle Selection
Summary: Resources and structs for campaign progression.

```rust
// Tracks what player has unlocked/completed
pub struct CampaignProgress {
    pub unlocked_arc: usize,           // Highest unlocked arc (0-based)
    pub completed_battles: Vec<Vec<bool>>,  // [arc][battle] = won?
}

// Currently selected battle to play
pub struct SelectedBattle {
    pub arc: usize,
    pub battle: usize,
}

// Definition of a single battle
pub struct BattleDef {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: Vec<EnemyConfig>,
    pub is_boss: bool,
}

// Definition of an arc (10 battles)
pub struct ArcDef {
    pub name: &'static str,
    pub description: &'static str,
    pub battles: Vec<BattleDef>,
}
```

Invariants:
- Each arc should have exactly 10 battles.
- Battle index 9 (10th battle) should have is_boss: true.
- Completing boss unlocks next arc automatically.
- Battle N is available if N==0 or battle N-1 is completed.

Refs:
- src/resources.rs:157-359

---

### PAT-008: Victory Outro Sequence
Summary: Post-battle victory screen with stats display and confirm-to-continue.

Pattern:
1. Victory condition inserts `VictoryOutro` resource (triggers outro mode).
2. `outro_active` run condition gates outro systems; `outro_not_active` gates gameplay.
3. Setup system spawns UI only once (checks if UI already exists).
4. Update system animates phases: HitStop -> Clear -> Stats -> WaitConfirm.
5. Confirm input sets `outro.confirmed = true`.
6. Transition system checks `is_done()` and changes state.

Timing:
- 0.0-0.1s: HitStop (brief pause)
- 0.1-0.5s: "CLEAR!" banner fades in with scale bounce
- 0.5-1.5s: Stats panel slides in, numbers count up
- 1.5s+: Wait for SPACE/Enter/Gamepad-South to continue

Key Components:
- `VictoryOutro` resource with elapsed time, phase, battle_time, reward
- `OutroPhase` enum for sequencing
- `BattleTimer` resource tracks elapsed battle time
- Victory sound plays at outro start

Query Conflicts:
- All text queries need `Without<OtherTextMarkers>` to avoid conflicts
- Stats panel children are separate entities with own markers

Refs:
- src/components.rs:VictoryOutro, OutroPhase
- src/systems/outro.rs
- src/resources.rs:BattleTimer

---

### PAT-009: Defeat Outro Sequence (parallel to Victory)
Summary: Defeat outro mirrors victory structure but with different visuals, no reward, and no progress.

Pattern:
1. `check_defeat_condition` checks if player HP <= 0 or player entity missing.
2. Inserts `DefeatOutro` resource (triggers outro mode).
3. Both `outro_active` and `outro_not_active` check for EITHER victory OR defeat resources.
4. Defeat systems run in parallel with victory systems (only one resource exists at a time).
5. Defeat does NOT call `campaign_progress.complete_battle()` - no progression.
6. Returns to Campaign screen without marking battle complete.

Timing (slightly longer hitstop for dramatic effect):
- 0.0-0.3s: HitStop (longer freeze on death)
- 0.3-0.8s: "GAME OVER" text with shake effect
- 0.8-1.5s: Stats panel slides in (time shown, "NO REWARD" message)
- 1.5s+: Wait for confirm input

Key Differences from Victory:
- Red "GAME OVER" text instead of gold "CLEAR!"
- Shake effect instead of bounce
- Shows "NO REWARD" instead of counting up reward
- `game-over.mp3` sound instead of `victory.mp3`
- No battle progress saved

Refs:
- src/components.rs:DefeatOutro, DefeatPhase
- src/systems/outro.rs:setup_defeat_outro, update_defeat_outro, check_defeat_outro_complete
- src/systems/combat.rs:check_defeat_condition

---

### GCH-004: Dead code from deprecated systems lingers in main.rs
Status: resolved

Summary: When removing deprecated systems (e.g., enemy_ai.rs), references in main.rs
system scheduling may remain, causing compilation errors.

Details:
- Deleted `src/systems/enemy_ai.rs` (replaced by `enemies::EnemyPlugin`).
- Removed `pub mod enemy_ai` and `use enemy_ai::*` from `systems/mod.rs`.
- But `bullet_hit_enemy` reference in main.rs line 190 remained.
- Compilation failed with "cannot find value `bullet_hit_enemy` in this scope".

Resolution:
- Always search main.rs for function names after deleting system files.
- Run `cargo check` immediately after deletions to catch dangling references.
- Also check for unused imports in files that called the deleted code.

Refs:
- src/main.rs:190 (removed reference)
- src/systems/combat.rs:2-12 (cleaned unused imports)

---

### GCH-005: Enemy movement must check for tile collisions
Status: resolved

Summary: Enemies moving independently can overlap on the same tile if collision
checking isn't implemented in the movement system.

Details:
- Multiple enemies using `MovementBehavior::Random` could all pick the same
  destination tile in the same frame.
- Must collect occupied positions BEFORE processing movement.
- Must update occupied set dynamically as each enemy moves to prevent two
  enemies from claiming the same empty tile.

Solution:
- Use `HashSet<(i32, i32)>` for O(1) collision lookups.
- Before moving, check `!occupied_positions.contains(&(new_x, new_y))`.
- After moving, remove old position and insert new position in the set.

```rust
let mut occupied: HashSet<(i32, i32)> = query.iter().map(|(_, pos, ..)| (pos.x, pos.y)).collect();

// In movement loop:
if is_valid_enemy_position(new_x, new_y) && !occupied.contains(&(new_x, new_y)) {
    occupied.remove(&(pos.x, pos.y));
    occupied.insert((new_x, new_y));
    pos.x = new_x;
    pos.y = new_y;
}
```

Refs:
- src/enemies/systems.rs:execute_movement_behavior()

---

### DEC-011: Composable action system via blueprints (MMBN Battle Chips)
Status: accepted

Summary: Create a blueprint-based action/chip system mirroring the enemy system, where
actions are defined by combining targeting, effects, and visuals.

Context:
- Original action system had hardcoded ActionType enum with 3 variants.
- Adding new actions required touching 5+ files (components, constants, actions, setup, UI).
- MMBN has 175+ Battle Chips with varied effects - needed scalable architecture.
- Wanted "LEGO-like" action creation matching the enemy blueprint pattern.

Decision:
- Create `src/actions/` module with same structure as `src/enemies/`.
- Define `ActionId` enum for all action types (expandable like EnemyId).
- Create `ActionBlueprint` combining:
  - Stats: cooldown, charge_time, element, rarity
  - Targeting: `ActionTarget` enum (OnSelf, Column, Row, Pattern, Projectile, etc.)
  - Effects: `ActionEffect` enum (Damage, Heal, Shield, Invisibility, etc.)
  - Modifiers: `ActionModifiers` struct (guard_break, element bonuses, etc.)
  - Visuals: `ActionVisuals` for icon and effect colors/sprites
- `ActionBlueprint::get(id)` returns complete definition.
- `ActionsPlugin` handles input, execution, and effect processing.

Alternatives:
- Keep hardcoded ActionType: Not scalable for 100+ actions.
- External data files (JSON/RON): More flexible but requires asset pipeline.
- Trait-based polymorphism: Harder to serialize and debug.

Consequences:
- Adding action = add enum variant + blueprint function + match arm.
- All action data visible in blueprints.rs.
- Type-safe, compile-time checked.
- Element system enables weakness bonuses.
- FighterConfig now uses Vec<ActionId> instead of Vec<ActionType>.
- Old ActionType deprecated but kept for backwards compatibility.

Refs:
- src/actions/mod.rs
- src/actions/blueprints.rs
- src/actions/behaviors.rs (ActionTarget, ActionEffect, ActionModifiers)
- src/actions/components.rs (ActionId, ActionSlot, Element, Rarity)
- src/actions/systems.rs (execution logic)

---

### INT-007: ActionBlueprint
Summary: Complete template for defining an action/chip.

```rust
pub struct ActionBlueprint {
    pub id: ActionId,
    pub name: &'static str,
    pub description: &'static str,
    pub element: Element,
    pub rarity: Rarity,
    pub cooldown: f32,
    pub charge_time: f32,
    pub target: ActionTarget,
    pub effect: ActionEffect,
    pub modifiers: ActionModifiers,
    pub visuals: ActionVisuals,
}

impl ActionBlueprint {
    pub fn get(id: ActionId) -> Self;
    pub fn display_name(&self) -> String;  // "Recov50 *"
}
```

Invariants:
- Every `ActionId` variant must have a corresponding blueprint function.
- `get()` must be exhaustive over all `ActionId` variants.
- `cooldown` is in seconds, must be > 0.
- `charge_time` can be 0 for instant actions.

Refs:
- src/actions/blueprints.rs

---

### INT-008: ActionTarget
Summary: Enum defining how an action selects its targets.

```rust
pub enum ActionTarget {
    OnSelf,
    SingleTile { range: i32 },
    Column { x_offset: i32 },
    Row { x_offset: i32, traveling: bool },
    Pattern { tiles: Vec<(i32, i32)> },
    Projectile { x_offset: i32, piercing: bool },
    ProjectileSpread { x_offset: i32, spread_rows: Vec<i32> },
    AreaAroundSelf { radius: i32 },
    AreaAtPosition { x_offset: i32, y_offset: i32, pattern: Vec<(i32, i32)> },
    EnemyArea,
    RandomEnemy { count: i32 },
}
```

Invariants:
- Offsets are relative to player position (positive = toward enemy).
- Pattern tiles are relative to action center.
- Tiles outside grid boundaries are filtered out.

Refs:
- src/actions/behaviors.rs:ActionTarget

---

### GCH-006: Legacy action components replaced by new system
Status: resolved

Summary: Old action components (Shield, WideSwordSlash, ChargedShot, HealFlashTimer)
were superseded by new actions module components but systems remained in main.rs.

Details:
- New actions module introduced: ActiveShield, DamageZone, HealFlash
- WeaponPlugin uses Projectile { is_charged: true } instead of ChargedShot
- Legacy systems in src/systems/actions.rs were never called but still imported
- Animation.rs referenced HealFlashTimer for query filters

Resolution:
- Removed legacy system imports from main.rs (heal_flash_effect, update_shield, etc.)
- Removed legacy system registrations from main.rs scheduling
- Updated animation.rs to use HealFlash from actions module
- Gutted src/systems/actions.rs to stub file with migration notes
- File kept for reference; can be deleted once stable

Refs:
- src/main.rs:26-30 (removed imports)
- src/main.rs:195-207, 216 (removed system registrations)
- src/systems/animation.rs:9 (updated import)
- src/systems/actions.rs (now stub file)

---

### INT-009: ActionEffect
Summary: Enum defining what an action does to targets.

```rust
pub enum ActionEffect {
    Damage { amount: i32, element: Element, can_crit: bool, guard_break: bool },
    Heal { amount: i32 },
    Shield { duration: f32, threshold: Option<i32> },
    Invisibility { duration: f32 },
    StealPanel { columns: i32 },
    CrackPanel { crack_only: bool },
    RepairPanel,
    Knockback { distance: i32 },
    Stun { duration: f32 },
    Drain { amount: i32 },
    MultiHit { damage_per_hit: i32, hit_count: i32, element: Element },
    Delayed { delay: f32, effect: Box<ActionEffect> },
    Combo { effects: Vec<ActionEffect> },
}
```

Invariants:
- Shield threshold = None means blocks all damage.
- Shield threshold = Some(0) means barrier (breaks after 1 hit).
- Shield threshold = Some(n) means aura (blocks damage < n).
- Combo effects are executed in order.
- Delayed effects spawn a pending entity.

Refs:
- src/actions/behaviors.rs:ActionEffect

---

## References

- [Bevy ECS Book](https://bevy.org/learn/book/ecs/) - Core ECS concepts
- [Bevy Error B0001](https://bevy.org/learn/errors/b0001) - Query conflict docs
- [Mega Man Battle Network](https://megaman.fandom.com/wiki/Mega_Man_Battle_Network)
  - Gameplay inspiration for tile-based arena combat
- [MMBN Battle Chip List](https://megaman.fandom.com/wiki/Battle_Network_1_chip_list)
  - Reference for chip variety and effects
