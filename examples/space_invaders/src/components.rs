//! Cougr components for Space Invaders entities.

use cougr_core::{impl_component, impl_marker_component};
use soroban_sdk::contracttype;

/// Marks the player ship entity.
pub struct ShipMarker;

impl_marker_component!(ShipMarker, "ship", Sparse);

/// Marks an invader entity.
pub struct InvaderMarker;

impl_marker_component!(InvaderMarker, "invader", Sparse);

/// Marks a player-fired bullet entity.
pub struct PlayerBulletMarker;

impl_marker_component!(PlayerBulletMarker, "p_bull", Sparse);

/// Marks an enemy-fired bullet entity.
pub struct EnemyBulletMarker;

impl_marker_component!(EnemyBulletMarker, "e_bull", Sparse);

/// Invader type encoded as u32 (matches `InvaderType` enum).
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvaderTypeComponent {
    pub invader_type: u32,
}

impl_component!(InvaderTypeComponent, "inv_type", Table, { invader_type: u32 });
