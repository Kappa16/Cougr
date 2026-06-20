# Cougr Game Circuits — Trusted Setup Policy

## Scope

This folder contains **development/test** Circom circuits for `cougr_core::circuits`.
Circuits use Poseidon commitments and game-specific constraints; production deploys
still require an independent phase-2 ceremony per circuit variant.

## Key tiers

| Tier | Source | Use |
|------|--------|-----|
| **Test** | `scripts/run-pipeline.sh` (CI + local) | Examples, integration wiring, CI |
| **Staging** | Team-run ceremony with recorded entropy | Testnet contracts |
| **Production** | Independent ceremony, keys outside repo | Mainnet |

## Test pipeline (CI-safe)

1. `compile.sh` — Circom → R1CS + WASM
2. `download-ptau.sh` — Hermez pot14 (`PTAU_POWER=14`, required for `hidden_cards`)
3. `setup.sh` — Groth16 phase-2 per circuit + `*_vk.json`
4. `prove.sh` — witness → proof → snarkjs verify
5. `export-vk.sh` — copy VK JSON to `exported/` for contract embedding

**Never** ship test `*_final.zkey` or CI-generated VKs to production.

## Production requirements

- Run a dedicated phase-2 ceremony per circuit variant (deck size, map bounds, etc.).
- Store proving keys off-repo; embed only verification keys in contracts.
- Record ceremony participants, entropy sources, and artifact hashes in your release notes.
- Re-run Cougr integration tests after VK swap (`GameCircuitSpec::with_verification_key`).

## Public-input freeze

Layouts are versioned by `CircuitId` in `src/circuits/spec.rs`. Changing Circom
public signals requires a new `CircuitId` or ADR amendment.