# Treasure Hunt (Merkle Map Pattern)

A single-player exploration game demonstrating **hidden information** using Merkle map commitments and sparse fog-of-war on Stellar Soroban. The full map is committed off-chain, and players prove cell contents via Merkle inclusion proofs as they explore.

## Status

**Transitional** — uses `cougr-core = "1.1.0"` and `privacy::stable` Merkle primitives (`MerkleTree`, `SparseMerkleTree`, `verify_inclusion`, `OnChainMerkleProof`). See [battleship](../battleship/) for the canonical hidden-information reference.

## The Hidden Information Pattern

Large maps are too expensive to store fully on-chain. This example keeps on-chain storage bounded:

- Full map remains off-chain, committed as a Merkle root
- Contract stores only: committed root, player state, and explored-cell sparse state
- Players reveal cells one at a time using Merkle inclusion proofs

### Commit Phase (Off-Chain)

```
Map Generator:
├─ Encode each cell as 32-byte leaf: [x(4) | y(4) | value(1) | padding(23)]
├─ Build SHA256 Merkle tree from all leaves
├─ Store root hash on-chain via init_game()
└─ Keep full tree off-chain for proof generation
```

### Reveal Phase (On-Chain)

```
Player:
├─ Moves to adjacent cell (x, y)
├─ Submits: cell_value + MerkleProof(siblings, leaf_hash, leaf_index)
└─ Contract:
    ├─ verify_inclusion(proof, committed_root) → true/false
    ├─ Applies discovery (treasure → +score, trap → -health)
    └─ Updates sparse fog-of-war via SparseMerkleTree
```

## When to Use Commit-Reveal vs ZK Circuits

Use **commit-reveal with Merkle proofs** (this pattern) when you need to hide large state spaces (maps, card decks, board layouts) and reveal them incrementally. Merkle proofs provide O(log n) verification without exposing the full state.

Use **ZK circuits** when the game logic itself must remain private — proving validity of moves without revealing any state. See [hidden_hand](../hidden_hand/) for a canonical ZK circuit example, and [proof_of_hunt](../proof_of_hunt/) for Groth16-based hidden state.

## Contract API

### Public Functions

| Function | Parameters | Description |
|----------|-----------|-------------|
| `init_game` | `player: Address`, `map_root: BytesN<32>`, `width: u32`, `height: u32`, `total_treasures: u32` | Initialize game with committed map root |
| `explore` | `player: Address`, `x: u32`, `y: u32`, `cell_value: u32`, `proof: Vec<BytesN<32>>` | Explore a cell with Merkle proof |
| `get_state` | — | Return complete `GameState` |
| `is_explored` | `x: u32`, `y: u32` | Check if a cell has been revealed |

### Data Types

```rust
enum GameStatus { Active, Won, Lost }

struct MapRoot {
    root: BytesN<32>,   // Merkle root of full map
    width: u32,
    height: u32,
    total_treasures: u32,
}

struct PlayerState {
    x: u32, y: u32, health: u32, score: u32, treasures_found: u32
}

struct ExploredMap {
    explored: Map<u32, bool>  // Sparse fog-of-war tracking
}
```

## Storage Model

### Committed State

Stored on-chain during initialization:
- `map_root` — SHA256 Merkle root of the full encoded map
- Map dimensions (`width`, `height`) and `total_treasures`
- `fog_root` — initial sparse Merkle tree root (all cells unexplored)

### Revealed State

Updated during exploration:
- `explored_map.explored` — sparse map of visited cell indices
- `player_state` — position, health, score, treasures found
- `fog_root` — recomputed `SparseMerkleTree` root after each exploration

### Proven State

The `explore` function verifies cell contents via Merkle inclusion:
1. Reconstructs the leaf hash from `(x, y, cell_value)` using the same encoding as the commit phase
2. Builds `OnChainMerkleProof` from sibling hashes and path bits
3. Calls `verify_inclusion(&env, &proof, &map_root.root)` — rejects if proof is invalid
4. Only if verification passes is the discovery applied and the cell marked explored

## Cell Encoding

Each map cell is encoded into a 32-byte leaf:

```
bytes [0..4]:   x coordinate (big-endian u32)
bytes [4..8]:   y coordinate (big-endian u32)
byte [8]:       cell_value (0=empty, 1=treasure, 2=trap)
bytes [9..31]:  zero padding
```

The off-chain generator hashes each cell and builds a SHA256 Merkle tree using `MerkleTree::from_leaves()` from `privacy::stable`.

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
- ✅ Game initialization with committed map root
- ✅ Valid exploration with correct Merkle proof
- ✅ Invalid proof rejection
- ✅ Treasure scoring and trap damage
- ✅ Re-exploration rejection
- ✅ Win condition (all treasures found)
- ✅ Loss condition (health reaches zero)
- ✅ Full playable sequence from init to terminal state
- ✅ Sparse fog root updates after exploration
- ✅ Non-adjacent move rejection
- ✅ Commit phase validation
- ✅ Reveal phase with valid proof
- ✅ Invalid reveal value rejection

## Security Considerations

### ✅ Secure
- **Map commitment binding**: Merkle root prevents changing cell contents after init
- **Selective reveal**: Only explored cells are revealed; unexplored cells stay hidden
- **Proof verification**: Invalid proofs are rejected by `verify_inclusion()`
- **Adjacency enforcement**: Players can only move to adjacent cells

### ⚠️ Important
- **Off-chain map trust**: The map must be generated honestly; the contract trusts the committed root
- **Sparse fog-of-war**: Recomputed on every move for full explored-cell tracking

## Deployment

```bash
stellar keys generate treasure-hunt-deployer --network testnet --fund
stellar contract deploy \
  --wasm target/wasm32v1-none/release/treasure_hunt.wasm \
  --source treasure-hunt-deployer \
  --network testnet
```

## Resources

- [Cougr Repository](https://github.com/salazarsebas/Cougr)
- [Merkle Trees](https://en.wikipedia.org/wiki/Merkle_tree)
- [battleship — canonical commit-reveal example](../battleship/)
- [hidden_hand — ZK circuit example](../hidden_hand/)
- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts)

## License

MIT OR Apache-2.0
