# Tetris Smart Contract

An on-chain Tetris game using **cougr-core** on Stellar Soroban.

> **Hybrid ECS example**: The locked board stays in a compact `GameState` struct; the active falling piece lives in a persisted `SimpleWorld`. For a full `GameApp` tick model, see [`snake`](../snake).

## Purpose and pattern

This example demonstrates a practical split when part of the game state is a dense grid and part is a small set of moving entities:

- **Board, score, level** — instance storage as `GameState` (bit-packed rows)
- **Active piece** — one entity in `SimpleWorld` with `Position`, `TetrominoComponent`, and `ActivePieceMarker`
- **Queries** — `SimpleQueryBuilder` locates the active piece before moves and locks

The board is not modeled as 200 cell entities because that would be expensive on-chain. Cougr owns the piece lifecycle (spawn, move, rotate, despawn, respawn).

## Public contract API

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `init_game` | — | `GameState` | Create board, spawn active piece in `SimpleWorld` |
| `move_left` | — | `bool` | Move active piece left |
| `move_right` | — | `bool` | Move active piece right |
| `move_down` | — | `bool` | Soft drop; locks piece if blocked |
| `rotate` | — | `bool` | Rotate active piece clockwise |
| `drop` | — | `u32` | Hard drop; returns cells dropped |
| `update_tick` | — | `GameState` | Gravity tick |
| `get_state` | — | `GameState` | Current board + piece (piece read from world) |
| `get_entity_count` | — | `u32` | Entities in `SimpleWorld` (0 or 1 while playing) |

## Architecture overview

```
Contract entrypoint
       │
       ├─ load GameState (board, score, next piece, meta)
       ├─ load SimpleWorld (active piece entity)
       │
       ├─ SimpleQuery → find entity with ActivePieceMarker
       ├─ read/update Position + TetrominoComponent
       ├─ collision against board rows
       │
       └─ save GameState + SimpleWorld
```

On lock: piece coordinates are written into the board bitfield, the entity is despawned, and a new piece entity is spawned from `next_piece`.

## Storage model

| Key | Type | Contents |
|-----|------|----------|
| `game` (instance) | `GameState` | Board rows, next piece preview, score, level, lines, game over |
| `world` (instance) | `SimpleWorld` | Active piece entity and components |

Both are updated together after every move or tick.

## Main gameplay flow

1. `init_game` — empty board, spawn first piece entity, generate `next_piece`
2. Player calls `move_*`, `rotate`, or `drop` — world components update after collision check
3. When the piece cannot move down, it locks: board updated, entity despawned, new entity spawned
4. Line clears update score/level in `GameState`
5. If the new piece collides immediately, set `game_over`

## Cougr APIs used

| API | Why |
|-----|-----|
| `SimpleWorld` | Persisted runtime for the active piece |
| `Position` | Piece x/y on the grid |
| `impl_component!` / `impl_marker_component!` | `TetrominoComponent`, `ActivePieceMarker` |
| `SimpleQueryBuilder` | Find the single active piece entity (sparse marker) |
| `set_typed` / `get_typed` | Read and write components on the piece entity |

Not used here (by design): `GameApp`, multi-entity simulation of the full board. See `snake` or `space_invaders` for those patterns.

## Build and test

```bash
cd examples/tetris
cargo test
stellar contract build
```

## Known limitations

- Only the falling piece is an ECS entity; locked cells live in the board vector
- No `GameApp` scheduler — move/lock logic is inline because each contract call is one player action
- Piece randomness uses Soroban PRNG; not suitable for competitive play without commit-reveal

## Project structure

```
examples/tetris/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs          # Contract, components, and game logic
```

## License

MIT OR Apache-2.0
