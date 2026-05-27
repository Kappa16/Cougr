#![no_std]

pub mod components;
pub mod types;

/*
STORAGE MODEL PARTITIONING:

| Data | Storage Type | Key | Reason |
|---|---|---|---|
| Puzzle definitions (grid, clues, solution, meta) | Persistent | (Symbol("PUZZLE"), puzzle_id: u32) | Puzzles must survive ledger expiration, and remain permanently in the registry catalog. |
| Puzzle registry counter | Persistent | Symbol("PUZZLE_COUNT") | The running count of submitted puzzles used for monotonic ID generation. Must survive expiration. |
| Player progress per puzzle | Instance | (Symbol("PROGRESS"), puzzle_id: u32, player: Address) | Active sessions. Instance storage is cheaper and its TTL is acceptable since inactive/abandoned games can expire or be cleaned up. |
| Puzzle status (active, solver count) | Persistent | (Symbol("STATUS"), puzzle_id: u32) | Long-lived operational registry-level data that tracks the puzzle's status and solver statistics. |
*/
