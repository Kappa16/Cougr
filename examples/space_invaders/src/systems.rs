//! Game systems operating on the Cougr `SimpleWorld`.

use crate::components::{
    EnemyBulletMarker, InvaderMarker, InvaderTypeComponent, PlayerBulletMarker, ShipMarker,
};
use crate::game_state::{
    InvaderType, BULLET_SPEED, GAME_HEIGHT, GAME_WIDTH, INVADER_COLS, INVADER_ROWS,
    INVADER_WIN_Y, SHIP_Y,
};
use cougr_core::component::{Health, Velocity};
use cougr_core::{Position, SimpleQueryBuilder, SimpleWorld};
use soroban_sdk::{symbol_short, Env, Vec};

pub fn init_world(env: &Env) -> (SimpleWorld, u32) {
    let mut world = SimpleWorld::new(env);

    let ship_id = world.spawn_entity();
    world.set_typed(env, ship_id, &Position::new(GAME_WIDTH / 2, SHIP_Y));
    world.set_typed(
        env,
        ship_id,
        &Health {
            current: 3,
            max: 3,
        },
    );
    world.set_typed(env, ship_id, &ShipMarker);

    for row in 0..INVADER_ROWS {
        let invader_type = match row {
            0 => InvaderType::Squid,
            1 | 2 => InvaderType::Crab,
            _ => InvaderType::Octopus,
        };

        for col in 0..INVADER_COLS {
            let entity_id = world.spawn_entity();
            let x = (col as i32 * 4) + 4;
            let y = (row as i32 * 3) + 2;

            world.set_typed(env, entity_id, &Position::new(x, y));
            world.set_typed(
                env,
                entity_id,
                &Health {
                    current: 1,
                    max: 1,
                },
            );
            world.set_typed(
                env,
                entity_id,
                &InvaderTypeComponent {
                    invader_type: invader_type.as_u32(),
                },
            );
            world.set_typed(env, entity_id, &InvaderMarker);
        }
    }

    (world, ship_id)
}

pub fn ship_x(world: &SimpleWorld, env: &Env, ship_entity_id: u32) -> i32 {
    world
        .get_typed::<Position>(env, ship_entity_id)
        .map(|pos| pos.x)
        .unwrap_or(GAME_WIDTH / 2)
}

pub fn set_ship_x(world: &mut SimpleWorld, env: &Env, ship_entity_id: u32, x: i32) {
    if let Some(pos) = world.get_typed::<Position>(env, ship_entity_id) {
        world.set_typed(env, ship_entity_id, &Position::new(x, pos.y));
    }
}

pub fn lives(world: &SimpleWorld, env: &Env, ship_entity_id: u32) -> u32 {
    world
        .get_typed::<Health>(env, ship_entity_id)
        .map(|health| health.current as u32)
        .unwrap_or(0)
}

pub fn spawn_player_bullet(world: &mut SimpleWorld, env: &Env, ship_entity_id: u32) {
    let ship_x = ship_x(world, env, ship_entity_id);
    let entity_id = world.spawn_entity();
    world.set_typed(env, entity_id, &Position::new(ship_x, SHIP_Y - 1));
    world.set_typed(
        env,
        entity_id,
        &Velocity::new(0, -BULLET_SPEED),
    );
    world.set_typed(env, entity_id, &PlayerBulletMarker);
}

pub fn spawn_enemy_bullet(world: &mut SimpleWorld, env: &Env, x: i32, y: i32) {
    let entity_id = world.spawn_entity();
    world.set_typed(env, entity_id, &Position::new(x, y + 1));
    world.set_typed(
        env,
        entity_id,
        &Velocity::new(0, BULLET_SPEED),
    );
    world.set_typed(env, entity_id, &EnemyBulletMarker);
}

pub fn move_player_bullets(world: &mut SimpleWorld, env: &Env) {
    move_bullets(world, env, symbol_short!("p_bull"), true);
}

pub fn move_enemy_bullets(world: &mut SimpleWorld, env: &Env) {
    move_bullets(world, env, symbol_short!("e_bull"), false);
}

fn move_bullets(
    world: &mut SimpleWorld,
    env: &Env,
    marker: soroban_sdk::Symbol,
    player_bullet: bool,
) {
    let query = SimpleQueryBuilder::new(env)
        .with_component(marker)
        .include_sparse()
        .build();
    let entities = query.execute(world, env);

    let mut to_despawn = Vec::new(env);
    for i in 0..entities.len() {
        let entity_id = entities.get(i).unwrap();
        if let (Some(mut pos), Some(vel)) = (
            world.get_typed::<Position>(env, entity_id),
            world.get_typed::<Velocity>(env, entity_id),
        ) {
            pos.x += vel.x;
            pos.y += vel.y;
            let off_screen = if player_bullet {
                pos.y <= 0
            } else {
                pos.y >= GAME_HEIGHT
            };
            if off_screen {
                to_despawn.push_back(entity_id);
            } else {
                world.set_typed(env, entity_id, &pos);
            }
        }
    }

    for i in 0..to_despawn.len() {
        world.despawn_entity(to_despawn.get(i).unwrap());
    }
}

pub fn move_invaders(
    world: &mut SimpleWorld,
    env: &Env,
    direction: i32,
    descend: bool,
) -> bool {
    let query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("invader"))
        .include_sparse()
        .build();
    let entities = query.execute(world, env);

    let mut reached_win_line = false;
    for i in 0..entities.len() {
        let entity_id = entities.get(i).unwrap();
        if let Some(health) = world.get_typed::<Health>(env, entity_id) {
            if health.current == 0 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(mut pos) = world.get_typed::<Position>(env, entity_id) {
            if descend {
                pos.y += 1;
            } else {
                pos.x += direction;
            }
            if pos.y >= INVADER_WIN_Y {
                reached_win_line = true;
            }
            world.set_typed(env, entity_id, &pos);
        }
    }

    reached_win_line
}

pub fn invader_bounds_reached(world: &SimpleWorld, env: &Env, direction: i32) -> (bool, bool) {
    let query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("invader"))
        .include_sparse()
        .build();
    let entities = query.execute(world, env);

    let mut should_reverse = false;
    let mut should_descend = false;
    for i in 0..entities.len() {
        let entity_id = entities.get(i).unwrap();
        if let Some(health) = world.get_typed::<Health>(env, entity_id) {
            if health.current == 0 {
                continue;
            }
        } else {
            continue;
        }

        if let Some(pos) = world.get_typed::<Position>(env, entity_id) {
            let new_x = pos.x + direction;
            if new_x <= 0 || new_x >= GAME_WIDTH - 1 {
                should_reverse = true;
                should_descend = true;
                break;
            }
        }
    }

    (should_reverse, should_descend)
}

pub fn resolve_player_bullet_hits(world: &mut SimpleWorld, env: &Env) -> u32 {
    let bullet_query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("p_bull"))
        .include_sparse()
        .build();
    let bullets = bullet_query.execute(world, env);

    let invader_query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("invader"))
        .include_sparse()
        .build();
    let invaders = invader_query.execute(world, env);

    let mut score = 0u32;
    let mut bullets_to_remove = Vec::new(env);

    for i in 0..bullets.len() {
        let bullet_id = bullets.get(i).unwrap();
        let bullet_pos = match world.get_typed::<Position>(env, bullet_id) {
            Some(pos) => pos,
            None => continue,
        };

        let mut hit = false;
        for j in 0..invaders.len() {
            let invader_id = invaders.get(j).unwrap();
            let health = match world.get_typed::<Health>(env, invader_id) {
                Some(health) if health.current > 0 => health,
                _ => continue,
            };

            let invader_pos = match world.get_typed::<Position>(env, invader_id) {
                Some(pos) => pos,
                None => continue,
            };

            if check_collision(
                bullet_pos.x,
                bullet_pos.y,
                invader_pos.x,
                invader_pos.y,
                2,
            ) {
                let mut updated_health = health;
                if updated_health.current > 0 {
                    updated_health.current -= 1;
                }
                world.set_typed(env, invader_id, &updated_health);

                if let Some(invader_type) =
                    world.get_typed::<InvaderTypeComponent>(env, invader_id)
                {
                    score += InvaderType::from_u32(invader_type.invader_type).points();
                }

                hit = true;
                break;
            }
        }

        if hit {
            bullets_to_remove.push_back(bullet_id);
        }
    }

    for i in 0..bullets_to_remove.len() {
        world.despawn_entity(bullets_to_remove.get(i).unwrap());
    }

    score
}

pub fn resolve_enemy_bullet_hits(
    world: &mut SimpleWorld,
    env: &Env,
    ship_entity_id: u32,
) -> bool {
    let bullet_query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("e_bull"))
        .include_sparse()
        .build();
    let bullets = bullet_query.execute(world, env);

    let ship_pos = match world.get_typed::<Position>(env, ship_entity_id) {
        Some(pos) => pos,
        None => return false,
    };

    let mut hit_ship = false;
    let mut bullets_to_remove = Vec::new(env);

    for i in 0..bullets.len() {
        let bullet_id = bullets.get(i).unwrap();
        let bullet_pos = match world.get_typed::<Position>(env, bullet_id) {
            Some(pos) => pos,
            None => continue,
        };

        if check_collision(
            bullet_pos.x,
            bullet_pos.y,
            ship_pos.x,
            ship_pos.y,
            2,
        ) {
            bullets_to_remove.push_back(bullet_id);
            hit_ship = true;
        }
    }

    for i in 0..bullets_to_remove.len() {
        world.despawn_entity(bullets_to_remove.get(i).unwrap());
    }

    if hit_ship {
        if let Some(mut health) = world.get_typed::<Health>(env, ship_entity_id) {
            if health.current > 0 {
                health.current -= 1;
            }
            world.set_typed(env, ship_entity_id, &health);
            return health.current == 0;
        }
    }

    false
}

pub fn active_invader_count(world: &SimpleWorld, env: &Env) -> u32 {
    let query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("invader"))
        .include_sparse()
        .build();
    let entities = query.execute(world, env);

    let mut count = 0u32;
    for i in 0..entities.len() {
        let entity_id = entities.get(i).unwrap();
        if let Some(health) = world.get_typed::<Health>(env, entity_id) {
            if health.current > 0 {
                count += 1;
            }
        }
    }
    count
}

pub fn first_active_invader_for_shot(world: &SimpleWorld, env: &Env, tick: u32) -> Option<(i32, i32)> {
    let query = SimpleQueryBuilder::new(env)
        .with_component(symbol_short!("invader"))
        .include_sparse()
        .build();
    let entities = query.execute(world, env);

    let target_col = (tick / 7) % INVADER_COLS;
    for i in 0..entities.len() {
        if i % INVADER_COLS != target_col {
            continue;
        }
        let entity_id = entities.get(i).unwrap();
        if let Some(health) = world.get_typed::<Health>(env, entity_id) {
            if health.current == 0 {
                continue;
            }
        } else {
            continue;
        }
        if let Some(pos) = world.get_typed::<Position>(env, entity_id) {
            return Some((pos.x, pos.y));
        }
    }
    None
}

fn check_collision(x1: i32, y1: i32, x2: i32, y2: i32, tolerance: i32) -> bool {
    (x1 - x2).abs() < tolerance && (y1 - y2).abs() < tolerance
}
