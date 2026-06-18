use crate::simple_world::SimpleWorld;
use soroban_sdk::{Env, Symbol};

/// Standard trait for Soroban contracts that use Cougr's ECS.
///
/// Provides a consistent pattern for loading and persisting the game world
/// to Soroban instance storage — removing the need to repeat the storage key
/// symbol in every contract entrypoint. Wire up once with
/// [`impl_soroban_game!`] and then call `load_world` / `save_world` throughout
/// the contract implementation.
///
/// # Example
/// ```no_run
/// use cougr_core::game::SorobanGame;
/// use cougr_core::impl_soroban_game;
/// use soroban_sdk::{contract, contractimpl, Env};
///
/// #[contract]
/// pub struct MyGame;
///
/// impl_soroban_game!(MyGame, "world");
///
/// #[contractimpl]
/// impl MyGame {
///     pub fn entity_count(env: Env) -> u32 {
///         let world = MyGame::load_world(&env);
///         world.next_entity_id().saturating_sub(1)
///     }
///
///     pub fn spawn(env: Env) -> u32 {
///         let mut world = MyGame::load_world(&env);
///         let id = world.spawn_entity();
///         MyGame::save_world(&env, &world);
///         id
///     }
/// }
/// ```
pub trait SorobanGame {
    /// The Soroban `Symbol` key used to store the world in instance storage.
    fn world_key(env: &Env) -> Symbol;

    /// Load the game world from Soroban instance storage.
    ///
    /// Returns a fresh empty world if the contract has not been initialised yet
    /// (i.e. the key does not exist in instance storage).
    fn load_world(env: &Env) -> SimpleWorld {
        SimpleWorld::load_from_instance(env, &Self::world_key(env))
    }

    /// Persist the game world to Soroban instance storage.
    ///
    /// Call at the end of every mutating entrypoint after all ECS operations
    /// have been applied.
    fn save_world(env: &Env, world: &SimpleWorld) {
        world.save_to_instance(env, &Self::world_key(env));
    }
}
