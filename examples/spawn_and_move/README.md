# spawn_and_move

The canonical Cougr starter game demonstrating the full ECS lifecycle on Soroban.

A player calls `spawn` to enter the world and receives an entity ID. They then call
`move_entity` with a direction to walk around a 2D grid. Every position change emits
an indexed Soroban event so off-chain clients can track movement in real time without
polling.

## What this example demonstrates

| Pattern | Where |
|---|---|
| `impl_component_observed!` — ECS component with indexer events | `Position` |
| `impl_component!` — private ECS component (no events needed) | `Moves` |
| `SorobanGame` + `impl_soroban_game!` — standard load/save | `SpawnAndMove` |
| Typed ECS access (`get_typed`, `set_typed_observed`) | `move_entity` |
| Multi-entity independence | test suite |

## Quick start

```toml
# Cargo.toml
[dependencies]
cougr-core = "1.1.0"
soroban-sdk = "25.1.0"
```

```rust
use cougr_core::game::SorobanGame;
use cougr_core::{impl_component_observed, impl_soroban_game};
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Position { pub x: i32, pub y: i32 }
impl_component_observed!(Position, "position", Table, { x: i32, y: i32 });

#[contract]
pub struct MyGame;
impl_soroban_game!(MyGame, "world");

#[contractimpl]
impl MyGame {
    pub fn spawn(env: Env) -> u32 {
        let mut world = MyGame::load_world(&env);
        let player = world.spawn_entity();
        world.set_typed_observed(&env, player, &Position { x: 0, y: 0 });
        MyGame::save_world(&env, &world);
        player
    }
}
```

## Build and test

```bash
cargo test
stellar contract build
```

## Directions

| Value | Direction | Effect |
|---|---|---|
| `0` | North | `y += 1` |
| `1` | East | `x += 1` |
| `2` | South | `y -= 1` |
| `3` | West | `x -= 1` |

## Events emitted

Every `set_typed_observed` call publishes a Soroban event with topics
`("COUGR", "set", "position")` and data `{ entity_id, data }`. Subscribe to
this topic from your frontend or indexer to track all position changes in
real time.

## Next steps

- Add an `Address` field to a `Players` component using `impl_rich_component!`
- Add win conditions and a game-over state
- Explore ZK hidden state with `impl_component_observed!` + `zk::stable` for
  fog-of-war mechanics
