//! Murdoku ECS components using cougr-core's ComponentTrait.
//!
//! Components that contain only fixed-size primitives use `cougr_core::impl_component!`.
//! Components with complex fields (Vec, nested #[contracttype]) implement
//! `ComponentTrait` manually via Soroban XDR serialization, following the same
//! pattern used in the canonical `snake` example.

use crate::types::{Clue, PuzzleMetadata, Suspect};
use cougr_core::component::{ComponentStorage, ComponentTrait};
use soroban_sdk::{contracttype, symbol_short, Bytes, Env, Symbol, Vec};

// ─── GridComponent ─────────────────────────────────────────────────────────

/// Flat NxN grid (row-major). Value = suspect index (0 = empty).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridComponent {
    pub grid_size: u32,
    pub cells: Vec<u32>,
}

impl ComponentTrait for GridComponent {
    fn component_type() -> Symbol {
        symbol_short!("grid")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── SuspectListComponent ──────────────────────────────────────────────────

/// Ordered list of suspects for a puzzle. Index position = suspect ID.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspectListComponent {
    pub suspects: Vec<Suspect>,
}

impl ComponentTrait for SuspectListComponent {
    fn component_type() -> Symbol {
        symbol_short!("suspects")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── ClueListComponent ─────────────────────────────────────────────────────

/// All clues associated with a puzzle.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClueListComponent {
    pub clues: Vec<Clue>,
}

impl ComponentTrait for ClueListComponent {
    fn component_type() -> Symbol {
        symbol_short!("clues")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── SolutionComponent ─────────────────────────────────────────────────────

/// Correct suspect arrangement (row-major, same indexing as GridComponent).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolutionComponent {
    pub cells: Vec<u32>,
}

impl ComponentTrait for SolutionComponent {
    fn component_type() -> Symbol {
        symbol_short!("solution")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── PuzzleMetaComponent ───────────────────────────────────────────────────

/// Authorship, difficulty, name, grid size.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PuzzleMetaComponent {
    pub metadata: PuzzleMetadata,
}

impl ComponentTrait for PuzzleMetaComponent {
    fn component_type() -> Symbol {
        symbol_short!("meta")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── PlayerProgressComponent ───────────────────────────────────────────────

/// Per-player current state. Mirrors GridComponent layout.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerProgressComponent {
    pub cells: Vec<u32>,
    pub move_count: u32,
    pub solved: bool,
}

impl ComponentTrait for PlayerProgressComponent {
    fn component_type() -> Symbol {
        symbol_short!("progress")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        use soroban_sdk::xdr::ToXdr;
        self.to_xdr(env)
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        use soroban_sdk::xdr::FromXdr;
        Self::from_xdr(env, data).ok()
    }

    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

// ─── PuzzleStatusComponent ─────────────────────────────────────────────────

/// Registry-level tracking.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PuzzleStatusComponent {
    pub active: bool,
    pub total_solvers: u32,
}

// PuzzleStatusComponent uses `impl_component!` because it contains only
// fixed-size primitives supported by the macro (bool + u32 = 5 bytes).
cougr_core::impl_component!(PuzzleStatusComponent, "status", Table, { active: bool, total_solvers: u32 });
