# Space Invaders — On-Chain Game Example

A Space Invaders game as a Soroban smart contract using **cougr-core** ECS.

## Purpose and pattern

This example demonstrates entity-centric gameplay on Soroban:

- **All gameplay objects** (ship, invaders, bullets) are entities in a persisted `SimpleWorld`
- **Components** use cougr-core `Position`, `Velocity`, and `Health`, plus local marker and type components
- **Systems** in `systems.rs` use `SimpleQueryBuilder` to scan entities by marker and update components each tick
- **Meta state** (`score`, `tick`, `cooldown`, `game_over`) lives in a small `GameState` struct for cheap reads

For the recommended `GameApp` + staged schedule pattern, see [`snake`](../snake).

## Public contract API

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `init_game` | — | — | Spawn ship + invader grid in `SimpleWorld` |
| `move_ship` | `direction: i32` | `i32` | Move ship (-1=left, 1=right) |
| `shoot` | — | `bool` | Spawn player bullet entity |
| `update_tick` | — | `bool` | Run movement, collision, and wave systems |
| `get_score` | — | `u32` | Current score |
| `get_lives` | — | `u32` | Ship `Health` component |
| `get_ship_position` | — | `i32` | Ship `Position.x` |
| `check_game_over` | — | `bool` | Game over flag |
| `get_active_invaders` | — | `u32` | Invaders with `Health > 0` |
| `get_entity_count` | — | `u32` | Total entities in world |

## Architecture overview

```
init_game
  └─ SimpleWorld: spawn ship (Position, Health, ShipMarker)
                 spawn 32 invaders (Position, Health, InvaderType, InvaderMarker)

update_tick
  ├─ Movement: query bullets by marker → apply Velocity to Position
  ├─ Collision: query player bullets × invaders → decrement Health, despawn bullets
  ├─ Collision: query enemy bullets × ship → decrement ship Health
  ├─ Invader wave: query invaders → shift Position, reverse on bounds
  └─ Enemy fire: spawn EnemyBulletMarker entities with Velocity

Storage: DataKey::World (SimpleWorld) + DataKey::State (meta)
```

## Storage model

| Key | Type | Contents |
|-----|------|----------|
| `State` (instance) | `GameState` | Score, tick, direction, cooldown, game over, ship entity id |
| `World` (instance) | `SimpleWorld` | Ship, invaders, bullets and all components |
| `Initialized` (instance) | `bool` | Init flag |

Gameplay positions and health are **only** in the world — not duplicated in parallel vectors.

## Main gameplay flow

1. `init_game` — 33 entities (1 ship + 32 invaders), no bullets
2. Player `move_ship` / `shoot` — update ship `Position` or spawn bullet entity
3. Each `update_tick`:
   - Move bullet entities via `Velocity`
   - Resolve bullet–invader and bullet–ship hits via position overlap
   - Every 5 ticks: move invader formation; reverse at edges
   - Every 7 ticks: one active invader fires
4. Win when all invaders destroyed; lose when ship health reaches 0 or invaders reach the player row

## Cougr APIs used

| API | Why |
|-----|-----|
| `SimpleWorld` | Central store for ship, invaders, and bullets |
| `Position`, `Velocity`, `Health` | Standard cougr-core gameplay components |
| `impl_component!` / `impl_marker_component!` | Invader type + entity role markers |
| `SimpleQueryBuilder` | Scan bullets and invaders by sparse marker each tick |
| `set_typed` / `get_typed` | Component read/write on entities |
| `RuntimeWorld::entity_count` | Exposed via `get_entity_count` |

Not used here: `GameApp` (systems are plain functions called from `update_tick`). The query + component pattern matches Cougr’s recommended data model.

## Build and test

```bash
cd examples/space_invaders
cargo test
stellar contract build
```

**Tests**: 14 passing, including `test_world_entity_count` for Cougr integration.

## Known limitations

- Systems run sequentially in `update_tick` rather than through `GameApp` stages
- Collision uses grid tolerance, not pixel physics
- Enemy shooting selects invaders by spawn-order index, not spatial AI

## Project structure

```
examples/space_invaders/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs          # Contract entrypoints
    ├── components.rs   # Marker and type components
    ├── game_state.rs   # Meta state and constants
    ├── systems.rs      # Query-driven movement and collision
    └── test.rs         # Unit tests
```

## License

MIT OR Apache-2.0
