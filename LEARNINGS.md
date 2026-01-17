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

## References

- [Bevy ECS Book](https://bevy.org/learn/book/ecs/) - Core ECS concepts
- [Bevy Error B0001](https://bevy.org/learn/errors/b0001) - Query conflict docs
- [Mega Man Battle Network](https://megaman.fandom.com/wiki/Mega_Man_Battle_Network)
  - Gameplay inspiration for tile-based arena combat
