# Rock Paper Scissors with Commit-Reveal

A two-player Rock Paper Scissors game demonstrating the **commit-reveal pattern** using cryptographic hashing on Stellar Soroban. This is the simplest example of hidden information in the Cougr framework.

## Status

**Transitional** — uses `cougr-core = "1.1.0"` with partial migration to `impl_component!` macros. See [battleship](../battleship/) for the canonical hidden-information reference.

## What is Commit-Reveal?

The commit-reveal pattern solves a fundamental problem in blockchain games: **how can players make simultaneous secret choices when all transactions are public?**

### The Problem

In a naive implementation:
1. Player A submits "Rock" → visible on-chain
2. Player B sees A's choice, submits "Paper" → B always wins!

### The Solution: Commit-Reveal

```
COMMIT PHASE (hide choices)
├─ Player A: hash(Rock + secret_salt) → 0x7a3f...
├─ Player B: hash(Paper + secret_salt) → 0x9b2e...
└─ Both hashes stored on-chain (choices hidden)

REVEAL PHASE (prove choices)
├─ Player A: reveals (Rock, secret_salt)
├─ Contract: verify hash(Rock + secret_salt) == 0x7a3f... ✓
├─ Player B: reveals (Paper, secret_salt)
├─ Contract: verify hash(Paper + secret_salt) == 0x9b2e... ✓
└─ Contract: compare choices → B wins!
```

**Key Properties:**
- ✅ **Binding**: Can't change choice after committing (hash locks it in)
- ✅ **Hiding**: Opponent can't see choice until reveal
- ✅ **Order-independent**: Neither player gains advantage by going first/second

## When to Use Commit-Reveal vs ZK Circuits

Use **commit-reveal** (this pattern) when you need to hide simple choices that are later revealed and verified by recomputing a hash. It's simple, efficient, and doesn't require external proving systems.

Use **ZK circuits** when the game logic itself must remain private — for example, proving that a move is valid without revealing what the move is. See [hidden_hand](../hidden_hand/) for a canonical ZK circuit-based example using Groth16 proofs.

## Game Flow

### 1. Initialize Match
```rust
new_match(player_a, player_b, best_of: 3)
```
Creates a best-of-N match (1, 3, 5, etc.)

### 2. Commit Phase
Both players compute and submit hashes:

```rust
// Off-chain: Player A
let salt = random_32_bytes();
let hash = sha256(choice || salt);

// On-chain
commit(player_a, hash)
```

When both players commit → automatically transitions to Reveal phase

### 3. Reveal Phase
Players reveal their choices with salts:

```rust
reveal(player_a, choice, salt)
```

Contract verifies: `sha256(choice || salt) == stored_hash`

When both players reveal → automatically resolves round

### 4. Resolution
```
Rock > Scissors
Scissors > Paper
Paper > Rock
Same choice = Draw
```

Updates scoreboard, checks if match winner determined (best-of-N), or starts next round.

### 5. Timeout Protection
If a player refuses to reveal after committing:

```rust
claim_timeout(honest_player)
```

After 100 ledgers, the honest player who revealed wins by forfeit.

## Contract API

### Public Functions

| Function | Parameters | Description |
|----------|-----------|-------------|
| `new_match` | `player_a: Address`, `player_b: Address`, `best_of: u32` | Initialize a new match |
| `commit` | `player: Address`, `hash: BytesN<32>` | Submit commitment hash |
| `reveal` | `player: Address`, `choice: u32`, `salt: BytesN<32>` | Reveal choice (0=Rock, 1=Paper, 2=Scissors) |
| `claim_timeout` | `player: Address` | Claim win if opponent doesn't reveal after 100 ledgers |
| `get_state` | — | Get current `MatchState` |
| `get_score` | — | Get `ScoreBoard` |

### Data Types

```rust
enum Choice { Rock = 0, Paper = 1, Scissors = 2 }
enum Phase { Committing, Revealing, Resolved }

struct MatchState {
    phase: Phase,
    winner: Option<Address>,
    round: u32,
}

struct ScoreBoard {
    wins_a: u32,
    wins_b: u32,
    draws: u32,
    best_of: u32,
}
```

## Storage Model

### Committed State

Stored on-chain during the commit phase:
- `hash_a/b` — SHA256 hash of each player's choice + random salt
- `has_commit_a/b` — flags tracking which players have committed
- `commit_ledger` — ledger sequence when both players committed (for timeout calculation)

Both commitments must be submitted before the game transitions to `Phase::Revealing`.

### Revealed State

Updated during the reveal phase:
- `revealed_a/b` — flags tracking which players have revealed
- `choice_a/b` — actual numeric choices (0, 1, or 2)
- `scoreboard` — accumulated wins/draws across rounds

### Proven State

The `reveal` function verifies that disclosed choices match the original commitment by recomputing `SHA256(choice || salt)` and comparing it to the stored hash.

## Component Serialization

`PlayerCommitment` uses the `impl_component!` macro for standardized serialization:

```rust
impl_component!(PlayerCommitment, "commit", Table, {
    hash: bytes32,
    revealed: bool
});
```

`MatchState` implements `ComponentTrait` manually since its `Phase` enum and `Option<Address>` fields don't map cleanly to the macro's fixed-size type system.

## Building & Testing

### Prerequisites
- Rust 1.70.0+
- Stellar CLI 25.0.0+ (optional)

### Build
```bash
cargo build
cargo build --release --target wasm32v1-none
```

### Test
```bash
cargo test
```

**Test Coverage:**
- ✅ Match initialization
- ✅ Commit phase transitions
- ✅ All 9 choice combinations (RR, RP, RS, PR, PP, PS, SR, SP, SS)
- ✅ Hash mismatch rejection
- ✅ Best-of-3 match flow
- ✅ Double commit prevention
- ✅ Premature reveal prevention
- ✅ Component trait serialization (via `impl_component!`)

## Security Considerations

### ✅ Secure
- **Commitment binding**: Hash function is collision-resistant
- **Choice hiding**: Preimage resistance prevents guessing
- **Replay protection**: Each round requires new commitments
- **Timeout protection**: Prevents griefing by non-revealing players

### ⚠️ Important
- **Salt randomness**: Use cryptographically secure random salts (32 bytes)
- **Salt uniqueness**: Never reuse salts across rounds

## Deployment

```bash
stellar keys generate rps-deployer --network testnet --fund
stellar contract deploy \
  --wasm target/wasm32v1-none/release/rock_paper_scissors.wasm \
  --source rps-deployer \
  --network testnet
```

## Resources

- [Cougr Repository](https://github.com/salazarsebas/Cougr)
- [Commit-Reveal Schemes](https://en.wikipedia.org/wiki/Commitment_scheme)
- [battleship — canonical commit-reveal example](../battleship/)
- [hidden_hand — ZK circuit example](../hidden_hand/)
- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts)

## License

MIT OR Apache-2.0
