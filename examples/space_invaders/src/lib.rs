//! Space Invaders - On-Chain Game Using Cougr
//!
//! Gameplay entities (ship, invaders, bullets) live in a persisted `SimpleWorld`.
//! Meta state (score, tick, cooldown) is stored separately for cheap reads.

#![no_std]

mod components;
mod game_state;
mod systems;

#[cfg(test)]
mod test;

use crate::game_state::*;
use cougr_core::{RuntimeWorld, SimpleWorld};
use soroban_sdk::{contract, contractimpl, Env};

pub use game_state::{Direction, InvaderType, GAME_HEIGHT, GAME_WIDTH, INVADER_COLS, INVADER_ROWS};

#[contract]
pub struct SpaceInvadersContract;

#[contractimpl]
impl SpaceInvadersContract {
    /// Initialize a new game with a Cougr `SimpleWorld` containing all entities.
    pub fn init_game(env: Env) {
        let (world, ship_entity_id) = systems::init_world(&env);
        let state = GameState::new(ship_entity_id);

        env.storage().instance().set(&DataKey::State, &state);
        env.storage().instance().set(&DataKey::World, &world);
        env.storage().instance().set(&DataKey::Initialized, &true);
    }

    /// Move the player's ship left or right.
    pub fn move_ship(env: Env, direction: i32) -> i32 {
        let state: GameState = env.storage().instance().get(&DataKey::State).unwrap();
        let mut world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();

        if state.game_over {
            return systems::ship_x(&world, &env, state.ship_entity_id);
        }

        let new_x = systems::ship_x(&world, &env, state.ship_entity_id) + direction;
        if (1..GAME_WIDTH - 1).contains(&new_x) {
            systems::set_ship_x(&mut world, &env, state.ship_entity_id, new_x);
            env.storage().instance().set(&DataKey::World, &world);
        }

        systems::ship_x(&world, &env, state.ship_entity_id)
    }

    /// Fire a bullet from the player's ship.
    pub fn shoot(env: Env) -> bool {
        let mut state: GameState = env.storage().instance().get(&DataKey::State).unwrap();
        let mut world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();

        if state.game_over || state.shoot_cooldown > 0 {
            return false;
        }

        systems::spawn_player_bullet(&mut world, &env, state.ship_entity_id);
        state.shoot_cooldown = SHOOT_COOLDOWN;

        env.storage().instance().set(&DataKey::World, &world);
        env.storage().instance().set(&DataKey::State, &state);
        true
    }

    /// Advance the game by one tick using Cougr queries and component updates.
    pub fn update_tick(env: Env) -> bool {
        let mut state: GameState = env.storage().instance().get(&DataKey::State).unwrap();
        let mut world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();

        if state.game_over {
            return false;
        }

        state.tick += 1;
        if state.shoot_cooldown > 0 {
            state.shoot_cooldown -= 1;
        }

        systems::move_player_bullets(&mut world, &env);
        systems::move_enemy_bullets(&mut world, &env);

        state.score += systems::resolve_player_bullet_hits(&mut world, &env);

        if systems::resolve_enemy_bullet_hits(&mut world, &env, state.ship_entity_id) {
            state.game_over = true;
        }

        if state.tick.is_multiple_of(INVADER_MOVE_INTERVAL) {
            let (should_reverse, should_descend) =
                systems::invader_bounds_reached(&world, &env, state.invader_direction);
            if systems::move_invaders(
                &mut world,
                &env,
                state.invader_direction,
                should_descend,
            ) {
                state.game_over = true;
            }
            if should_reverse {
                state.invader_direction *= -1;
            }
        }

        if state.tick.is_multiple_of(7) {
            if let Some((x, y)) = systems::first_active_invader_for_shot(&world, &env, state.tick) {
                systems::spawn_enemy_bullet(&mut world, &env, x, y);
            }
        }

        if systems::active_invader_count(&world, &env) == 0 {
            state.game_over = true;
        }

        env.storage().instance().set(&DataKey::State, &state);
        env.storage().instance().set(&DataKey::World, &world);
        !state.game_over
    }

    pub fn get_score(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .map(|state: GameState| state.score)
            .unwrap_or(0)
    }

    pub fn get_lives(env: Env) -> u32 {
        let state: GameState = env.storage().instance().get(&DataKey::State).unwrap();
        let world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();
        systems::lives(&world, &env, state.ship_entity_id)
    }

    pub fn get_ship_position(env: Env) -> i32 {
        let state: GameState = env.storage().instance().get(&DataKey::State).unwrap();
        let world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();
        systems::ship_x(&world, &env, state.ship_entity_id)
    }

    pub fn check_game_over(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .map(|state: GameState| state.game_over)
            .unwrap_or(true)
    }

    pub fn get_active_invaders(env: Env) -> u32 {
        let world: SimpleWorld = env.storage().instance().get(&DataKey::World).unwrap();
        systems::active_invader_count(&world, &env)
    }

    /// Returns the number of entities currently in the Cougr world.
    pub fn get_entity_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::World)
            .map(|world: SimpleWorld| world.entity_count() as u32)
            .unwrap_or(0)
    }
}
