use std::vec;

use rand::seq::SliceRandom;

use crate::battleship::{constants::{NUM_ROWS, NUM_COLS, BOATS, Boat, EMPTY}, game::Shot};

const LENGTHS: [usize; 5] = [2, 3, 3, 4, 5];

pub fn length(boat: Boat) -> usize {
    LENGTHS[boat - 1]
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
}

pub fn valid_pos(
    boats: &[[Boat; NUM_ROWS]; NUM_COLS], boat: Boat,
    horizontal: bool, pos: Pos
) -> bool {
    let mut valid_position = true;

    if horizontal {
        for x_off in 0..length(boat) {
            if boats[pos.x + x_off][pos.y] != EMPTY {
                valid_position = false;
                break;
            }
        }
    }
    else {
        for y_off in 0..length(boat) {
            if boats[pos.x][pos.y + y_off] != EMPTY {
                valid_position = false;
                break;
            }
        }
    }

    valid_position
}

pub fn valid_shot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: usize, y: usize) -> bool  {
    shots[x][y].is_none()
}

fn offset_shoot_pos(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: usize, y: usize) -> Option<Pos> {
    let mut positions = vec![];

    if x as i32 - 1 > 0 && valid_shot(shots, x - 1, y) { positions.push(Pos::new(x - 1, y)); }
    if x + 1 < NUM_COLS && valid_shot(shots, x + 1, y) { positions.push(Pos::new(x + 1, y)); }
    if y as i32 - 1 > 0 && valid_shot(shots, x, y - 1) { positions.push(Pos::new(x, y - 1)); }
    if y + 1 < NUM_ROWS && valid_shot(shots, x, y + 1) { positions.push(Pos::new(x, y + 1)); }

    let rand_pos = positions.choose(&mut rand::thread_rng());

    rand_pos.copied()
}

pub fn random_focus(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Option<Pos> {
    let mut hits = vec![];

    for (x, column) in shots.iter().enumerate() {
        for (y, shot) in column.iter().enumerate() {
            if let Some(Shot::Hit(boat)) = shot {
                hits.push((*boat, x, y));
            }
        }
    }

    for boat in BOATS {
        let boat_hits = hits
            .iter()
            .filter(|hit| hit.0 == boat)
            .copied();

        let hits_len = boat_hits.clone().count();

        if hits_len > 0 && hits_len < length(boat) {
            let boat_hits_vec: Vec<(Boat, usize, usize)> = boat_hits.collect();

            let (_, x, y) = boat_hits_vec
                .choose(&mut rand::thread_rng())
                .copied()
                .expect("No boat hits");

            return offset_shoot_pos(shots, x, y)
        }
    }

    None
}
