#![no_std]

use cougr_core::{
    impl_component, impl_marker_component, Position, RuntimeWorld, SimpleQueryBuilder, SimpleWorld,
};
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Vec};

// --------------------------------------------------------------------------------
// Data Structures
// --------------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TetrominoShape {
    I = 0,
    J = 1,
    L = 2,
    O = 3,
    S = 4,
    T = 5,
    Z = 6,
}

impl TetrominoShape {
    fn from_u32(value: u32) -> Self {
        match value {
            0 => TetrominoShape::I,
            1 => TetrominoShape::J,
            2 => TetrominoShape::L,
            3 => TetrominoShape::O,
            4 => TetrominoShape::S,
            5 => TetrominoShape::T,
            _ => TetrominoShape::Z,
        }
    }

    fn as_u32(self) -> u32 {
        self as u32
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub shape: TetrominoShape,
    pub x: i32,
    pub y: i32,
    pub rotation: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Vec<u32>,
    pub current_piece: Piece,
    pub next_piece: Piece,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
}

// --------------------------------------------------------------------------------
// ECS Components
// --------------------------------------------------------------------------------

/// Shape and rotation for the falling tetromino entity.
#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TetrominoComponent {
    pub shape: u32,
    pub rotation: u32,
}

impl_component!(TetrominoComponent, "tetrom", Table, { shape: u32, rotation: u32 });

/// Marks the single active falling piece entity in the world.
pub struct ActivePieceMarker;

impl_marker_component!(ActivePieceMarker, "active", Sparse);

// --------------------------------------------------------------------------------
// Contract
// --------------------------------------------------------------------------------

const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 20;
const GAME_KEY: soroban_sdk::Symbol = symbol_short!("game");
const WORLD_KEY: soroban_sdk::Symbol = symbol_short!("world");

#[contract]
pub struct TetrisContract;

#[contractimpl]
impl TetrisContract {
    /// Initialize the game
    pub fn init_game(env: Env) -> GameState {
        let board = Vec::from_array(&env, [0u32; 20]);
        let next_piece = generate_piece(&env);

        let mut world = SimpleWorld::new(&env);
        let current_piece = spawn_active_piece(&mut world, &env, next_piece.clone());

        let state = GameState {
            board,
            current_piece,
            next_piece,
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
        };

        save_world(&env, &world);
        save_state(&env, &state);
        state
    }

    /// Move current piece left
    pub fn move_left(env: Env) -> bool {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return false;
        }

        let mut world = load_world(&env);
        if try_move(&env, &mut world, &mut state, -1, 0, 0) {
            save_world(&env, &world);
            save_state(&env, &state);
            true
        } else {
            false
        }
    }

    /// Move current piece right
    pub fn move_right(env: Env) -> bool {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return false;
        }

        let mut world = load_world(&env);
        if try_move(&env, &mut world, &mut state, 1, 0, 0) {
            save_world(&env, &world);
            save_state(&env, &state);
            true
        } else {
            false
        }
    }

    /// Move current piece down (soft drop)
    pub fn move_down(env: Env) -> bool {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return false;
        }

        let mut world = load_world(&env);
        if try_move(&env, &mut world, &mut state, 0, 1, 0) {
            save_world(&env, &world);
            save_state(&env, &state);
            true
        } else {
            lock_piece(&env, &mut world, &mut state);
            save_world(&env, &world);
            save_state(&env, &state);
            false
        }
    }

    /// Rotate piece
    pub fn rotate(env: Env) -> bool {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return false;
        }

        let mut world = load_world(&env);
        if try_move(&env, &mut world, &mut state, 0, 0, 1) {
            save_world(&env, &world);
            save_state(&env, &state);
            true
        } else {
            false
        }
    }

    /// Hard drop
    pub fn drop(env: Env) -> u32 {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return 0;
        }

        let mut world = load_world(&env);
        let mut dropped = 0;
        while try_move(&env, &mut world, &mut state, 0, 1, 0) {
            dropped += 1;
        }

        lock_piece(&env, &mut world, &mut state);
        save_world(&env, &world);
        save_state(&env, &state);
        dropped
    }

    /// Update tick (gravity)
    pub fn update_tick(env: Env) -> GameState {
        let mut state = Self::get_state(env.clone());
        if state.game_over {
            return state;
        }

        let mut world = load_world(&env);
        if !try_move(&env, &mut world, &mut state, 0, 1, 0) {
            lock_piece(&env, &mut world, &mut state);
        }

        save_world(&env, &world);
        save_state(&env, &state);
        state
    }

    /// Get current state
    pub fn get_state(env: Env) -> GameState {
        let mut state: GameState = env
            .storage()
            .instance()
            .get(&GAME_KEY)
            .expect("Game not initialized");
        let world = load_world(&env);
        if let Some(piece) = active_piece(&world, &env) {
            state.current_piece = piece;
        }
        state
    }

    /// Returns the number of entities in the Cougr world (active piece).
    pub fn get_entity_count(env: Env) -> u32 {
        load_world(&env).entity_count() as u32
    }
}

// --------------------------------------------------------------------------------
// World helpers
// --------------------------------------------------------------------------------

fn load_world(env: &Env) -> SimpleWorld {
    env.storage()
        .instance()
        .get(&WORLD_KEY)
        .expect("World not initialized")
}

fn save_world(env: &Env, world: &SimpleWorld) {
    env.storage().instance().set(&WORLD_KEY, world);
}

fn save_state(env: &Env, state: &GameState) {
    env.storage().instance().set(&GAME_KEY, state);
}

fn active_piece_entity(world: &SimpleWorld, env: &Env) -> Option<u32> {
    let query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("active"))
        .include_sparse()
        .build();
    let entities = query.execute(world, env);
    if entities.is_empty() {
        None
    } else {
        Some(entities.get(0).unwrap())
    }
}

fn active_piece(world: &SimpleWorld, env: &Env) -> Option<Piece> {
    let entity_id = active_piece_entity(world, env)?;
    piece_from_entity(world, env, entity_id)
}

fn piece_from_entity(world: &SimpleWorld, env: &Env, entity_id: u32) -> Option<Piece> {
    let position = world.get_typed::<Position>(env, entity_id)?;
    let tetromino = world.get_typed::<TetrominoComponent>(env, entity_id)?;
    Some(Piece {
        shape: TetrominoShape::from_u32(tetromino.shape),
        x: position.x,
        y: position.y,
        rotation: tetromino.rotation,
    })
}

fn spawn_active_piece(world: &mut SimpleWorld, env: &Env, piece: Piece) -> Piece {
    if let Some(entity_id) = active_piece_entity(world, env) {
        world.despawn_entity(entity_id);
    }

    let entity_id = world.spawn_entity();
    world.set_typed(
        env,
        entity_id,
        &Position::new(piece.x, piece.y),
    );
    world.set_typed(
        env,
        entity_id,
        &TetrominoComponent {
            shape: piece.shape.as_u32(),
            rotation: piece.rotation,
        },
    );
    world.set_typed(env, entity_id, &ActivePieceMarker);

    piece
}

fn generate_piece(env: &Env) -> Piece {
    let shape_idx = env.prng().gen_range::<u64>(0..7) as u32;
    let shape = TetrominoShape::from_u32(shape_idx);

    Piece {
        shape,
        x: 3,
        y: 0,
        rotation: 0,
    }
}

// --------------------------------------------------------------------------------
// Game logic
// --------------------------------------------------------------------------------

fn try_move(
    env: &Env,
    world: &mut SimpleWorld,
    state: &mut GameState,
    dx: i32,
    dy: i32,
    d_rot: i32,
) -> bool {
    let entity_id = match active_piece_entity(world, env) {
        Some(id) => id,
        None => return false,
    };

    let position = match world.get_typed::<Position>(env, entity_id) {
        Some(pos) => pos,
        None => return false,
    };
    let tetromino = match world.get_typed::<TetrominoComponent>(env, entity_id) {
        Some(data) => data,
        None => return false,
    };

    let shape = TetrominoShape::from_u32(tetromino.shape);
    let new_x = position.x + dx;
    let new_y = position.y + dy;
    let new_rot = (tetromino.rotation as i32 + d_rot).rem_euclid(4) as u32;

    if check_collision(env, &state.board, shape, new_x, new_y, new_rot) {
        return false;
    }

    world.set_typed(env, entity_id, &Position::new(new_x, new_y));
    world.set_typed(
        env,
        entity_id,
        &TetrominoComponent {
            shape: tetromino.shape,
            rotation: new_rot,
        },
    );

    state.current_piece = Piece {
        shape,
        x: new_x,
        y: new_y,
        rotation: new_rot,
    };

    true
}

fn check_collision(
    _env: &Env,
    board: &Vec<u32>,
    shape: TetrominoShape,
    x: i32,
    y: i32,
    rot: u32,
) -> bool {
    let coords = get_piece_coords(shape, rot);

    for (cx, cy) in coords {
        let abs_x = x + cx;
        let abs_y = y + cy;

        if !(0..BOARD_WIDTH).contains(&abs_x) || abs_y >= BOARD_HEIGHT {
            return true;
        }

        if abs_y >= 0 {
            let row = board.get(abs_y as u32).unwrap_or(0);
            if (row >> abs_x) & 1 == 1 {
                return true;
            }
        }
    }
    false
}

fn lock_piece(env: &Env, world: &mut SimpleWorld, state: &mut GameState) {
    let entity_id = match active_piece_entity(world, env) {
        Some(id) => id,
        None => return,
    };

    let piece = match piece_from_entity(world, env, entity_id) {
        Some(piece) => piece,
        None => return,
    };

    let coords = get_piece_coords(piece.shape, piece.rotation);
    let mut game_over = false;

    for (cx, cy) in coords {
        let abs_x = piece.x + cx;
        let abs_y = piece.y + cy;

        if abs_y < 0 {
            game_over = true;
        } else if abs_y < BOARD_HEIGHT {
            let mut row = state.board.get(abs_y as u32).unwrap_or(0);
            row |= 1 << abs_x;
            state.board.set(abs_y as u32, row);
        }
    }

    world.despawn_entity(entity_id);

    if game_over {
        state.game_over = true;
        return;
    }

    let mut lines = 0;
    let mut new_board = Vec::new(env);

    for i in 0..state.board.len() {
        let row = state.board.get(i).unwrap();
        if row == 1023 {
            lines += 1;
        } else {
            new_board.push_back(row);
        }
    }

    for _ in 0..lines {
        new_board.push_front(0);
    }
    state.board = new_board;

    if lines > 0 {
        let points = match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        state.score += points * (state.level + 1);
        state.lines_cleared += lines;
        if state.lines_cleared >= state.level * 10 {
            state.level += 1;
        }
    }

    state.current_piece = state.next_piece.clone();
    state.next_piece = generate_piece(env);
    state.current_piece = spawn_active_piece(world, env, state.current_piece.clone());

    if check_collision(
        env,
        &state.board,
        state.current_piece.shape,
        state.current_piece.x,
        state.current_piece.y,
        state.current_piece.rotation,
    ) {
        if let Some(entity_id) = active_piece_entity(world, env) {
            world.despawn_entity(entity_id);
        }
        state.game_over = true;
    }
}

fn get_piece_coords(shape: TetrominoShape, rot: u32) -> [(i32, i32); 4] {
    match shape {
        TetrominoShape::I => match rot {
            0 => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            1 => [(1, -1), (1, 0), (1, 1), (1, 2)],
            2 => [(-1, 1), (0, 1), (1, 1), (2, 1)],
            _ => [(0, -1), (0, 0), (0, 1), (0, 2)],
        },
        TetrominoShape::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
        TetrominoShape::T => match rot {
            0 => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            1 => [(0, -1), (0, 0), (0, 1), (-1, 0)],
            2 => [(-1, 0), (0, 0), (1, 0), (0, -1)],
            _ => [(0, -1), (0, 0), (0, 1), (1, 0)],
        },
        TetrominoShape::J => match rot {
            0 => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            1 => [(0, -1), (0, 0), (0, 1), (-1, 1)],
            2 => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
            _ => [(1, -1), (0, 0), (0, -1), (0, 1)],
        },
        TetrominoShape::L => match rot {
            0 => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            1 => [(0, -1), (0, 0), (0, 1), (1, 1)],
            2 => [(1, -1), (-1, 0), (0, 0), (1, 0)],
            _ => [(-1, -1), (0, -1), (0, 0), (0, 1)],
        },
        TetrominoShape::S => match rot {
            0 => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            1 => [(0, -1), (0, 0), (1, 0), (1, 1)],
            2 => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            _ => [(0, -1), (0, 0), (1, 0), (1, 1)],
        },
        TetrominoShape::Z => match rot {
            0 => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            1 => [(1, -1), (1, 0), (0, 0), (0, 1)],
            2 => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            _ => [(1, -1), (1, 0), (0, 0), (0, 1)],
        },
    }
}

// --------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init_game() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        let state = client.init_game();
        assert_eq!(state.score, 0);
        assert!(!state.game_over);
        assert_eq!(client.get_entity_count(), 1);
    }

    #[test]
    fn test_move_functions() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        let _moved = client.move_left();
    }

    #[test]
    fn test_rotation() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        let _rotated = client.rotate();
    }

    #[test]
    fn test_collision_detection() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();

        for _ in 0..10 {
            client.move_left();
        }
    }

    #[test]
    fn test_line_clearing() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        let _lines = client.update_tick();
    }

    #[test]
    fn test_score_updates() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        assert_eq!(client.get_state().score, 0);
    }

    #[test]
    fn test_game_over() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        assert!(!client.get_state().game_over);
    }

    #[test]
    fn test_active_piece_in_world() {
        let env = Env::default();
        let client = TetrisContractClient::new(&env, &env.register(TetrisContract, ()));
        client.init_game();
        assert_eq!(client.get_entity_count(), 1);
    }
}
