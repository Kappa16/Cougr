//! Meta game state and domain types for Space Invaders.

use soroban_sdk::contracttype;

/// Direction for ship movement
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

/// Type of invader (affects points and behavior)
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum InvaderType {
    Squid = 0,
    Crab = 1,
    Octopus = 2,
}

impl InvaderType {
    pub fn points(&self) -> u32 {
        match self {
            InvaderType::Squid => 30,
            InvaderType::Crab => 20,
            InvaderType::Octopus => 10,
        }
    }

    pub fn as_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => InvaderType::Squid,
            1 => InvaderType::Crab,
            _ => InvaderType::Octopus,
        }
    }
}

/// Non-ECS meta state persisted alongside the Cougr world.
#[contracttype]
#[derive(Clone, Debug)]
pub struct GameState {
    pub score: u32,
    pub game_over: bool,
    pub invader_direction: i32,
    pub tick: u32,
    pub shoot_cooldown: u32,
    pub ship_entity_id: u32,
}

impl GameState {
    pub fn new(ship_entity_id: u32) -> Self {
        Self {
            score: 0,
            game_over: false,
            invader_direction: 1,
            tick: 0,
            shoot_cooldown: 0,
            ship_entity_id,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}

pub const GAME_WIDTH: i32 = 40;
pub const GAME_HEIGHT: i32 = 30;
pub const INVADER_COLS: u32 = 8;
pub const INVADER_ROWS: u32 = 4;
pub const SHIP_Y: i32 = GAME_HEIGHT - 2;
pub const INVADER_WIN_Y: i32 = SHIP_Y - 2;
pub const SHOOT_COOLDOWN: u32 = 3;
pub const BULLET_SPEED: i32 = 2;
pub const INVADER_MOVE_INTERVAL: u32 = 5;

/// Storage keys for Soroban instance storage.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    State,
    World,
    Initialized,
}
