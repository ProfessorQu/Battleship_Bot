use std::sync::RwLock;
use rand::Rng;

use lazy_static::lazy_static;

use crate::pos;
use crate::battleship::boat::BOATS;
use crate::battleship::{Pos, Player};
use crate::battleship::player::utils::get_hits;
use crate::battleship::constants::{NUM_COLS, NUM_ROWS, ShotMap};
use crate::battleship::player::destroy::{valid_shot, destroy, random_destroy};

pub fn random_find(shots: ShotMap) -> Pos {
    let mut rng = rand::thread_rng();

    let (mut x, mut y) = (
        rng.gen_range(0..NUM_COLS),
        rng.gen_range(0..NUM_ROWS),
    );

    while !valid_shot(shots, pos!(x, y)) {
        (x, y) = (
            rng.gen_range(0..NUM_COLS),
            rng.gen_range(0..NUM_ROWS),
        );
    }

    pos!(x, y)
}

pub fn random_shoot(_player: Player, shots: ShotMap) -> Pos {
    random_find(shots)
}

pub fn random_find_and_random_destroy(_player: Player, shots: ShotMap) -> Pos {
    if let Some(pos) = random_destroy(shots) {
        pos
    } else {
        random_find(shots)
    }
}

pub fn random_find_and_destroy(_player: Player, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        random_find(shots)
    }
}

lazy_static!(
    static ref LAST_POS_P1: RwLock<Pos> = RwLock::new(Pos::new(0, 0));
    static ref LAST_POS_P2: RwLock<Pos> = RwLock::new(Pos::new(0, 0));
);

fn grid_find(shots: ShotMap, last_pos: &mut Pos) -> Pos {
    if shots[0][0].is_none() {
        return pos!(0, 0)
    }

    let mut min_len = 6;

    for boat in BOATS {
        let hits = get_hits(shots);
        let hits = hits
            .iter()
            .filter(|shot| shot.0 == boat);
        let hits_len = hits.clone().count();

        let boat_len = boat.length();

        if hits_len < boat_len && boat_len < min_len {
            min_len = boat_len;
        }
    }

    let mut position = last_pos.to_owned();

    let mut iterations = 0;
    while !valid_shot(shots, position) {
        position.x += min_len;
        
        if position.x >= NUM_COLS {
            position.y += 1;

            position.x %= NUM_COLS;
            if position.y % 2 == 1 && position.x == 0 {
                position.x += 1;
            } else if position.y % 2 == 0 && position.x == 1 {
                position.x -= 1;
            }
        }

        if position.y >= NUM_ROWS {
            position = pos!(0, 0);
        }

        if iterations > 100 {
            return random_find(shots)
        }

        iterations += 1;
    }

    *last_pos = position;

    position
}

pub fn grid_find_and_destroy(player: Player, shots: ShotMap) -> Pos {
    if let Some(pos) = destroy(shots) {
        pos
    } else {
        let mut pos_lock = match player {
            Player::P1 => LAST_POS_P1.write().expect("Failed to write Grid state"),
            Player::P2 => LAST_POS_P2.write().expect("Failed to write Grid state")
        };

        let mut last_pos = pos_lock.to_owned();

        let result = grid_find(shots, &mut last_pos);

        *pos_lock = last_pos;

        result
    }
}
