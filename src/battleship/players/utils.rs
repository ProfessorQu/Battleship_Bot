use std::vec;

use rand::seq::SliceRandom;

use crate::battleship::{constants::{NUM_ROWS, NUM_COLS, BOATS, Boat}, game::Shot};

const LENGTHS: [usize; 5] = [2, 3, 3, 4, 5];

pub fn length(boat: Boat) -> usize {
    LENGTHS[boat as usize - 1]
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

#[macro_export]
macro_rules! pos {
    ($t:expr) => {
        Pos::new($t.0, $t.1)
    };
    ($x:expr, $y:expr) => {
        Pos::new($x, $y)
    };
}

pub fn valid_boat_pos(
    boats: &[[Boat; NUM_ROWS]; NUM_COLS],
    boat: Boat,
    horizontal: bool, pos: Pos
) -> bool {
    let mut valid_position = true;

    if horizontal {
        for x_off in 0..length(boat) {
            if boats[pos.x + x_off][pos.y].has_some() {
                valid_position = false;
                break;
            }
        }
    }
    else {
        for y_off in 0..length(boat) {
            if boats[pos.x][pos.y + y_off].has_some() {
                valid_position = false;
                break;
            }
        }
    }

    valid_position
}

pub fn valid_shot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], pos: Pos) -> bool  {
    shots[pos.x][pos.y].is_none()
}

pub fn valid_shot_any(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: i32, y: i32) -> bool {
    x >= 0 && x < NUM_COLS as i32 &&
    y >= 0 && y < NUM_ROWS as i32 &&
    shots[x as usize][y as usize].is_none()
}

fn random_offset_shoot_pos(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], pos: Pos) -> Option<Pos> {
    let mut positions = vec![];

    let (x, y) = (pos.x, pos.y);
    let (x_i, y_i) = (x as i32, y as i32);

    if valid_shot_any(shots, x_i - 1, y_i) { positions.push(pos!(x - 1, y)); }
    if valid_shot_any(shots, x_i + 1, y_i) { positions.push(pos!(x + 1, y)); }
    if valid_shot_any(shots, x_i, y_i - 1) { positions.push(pos!(x, y - 1)); }
    if valid_shot_any(shots, x_i, y_i + 1) { positions.push(pos!(x, y + 1)); }

    let rand_pos = positions.choose(&mut rand::thread_rng());

    rand_pos.copied()
}

fn get_hits(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Vec<(Boat, usize, usize)> {
    let mut hits = vec![];

    for (x, column) in shots.iter().enumerate() {
        for (y, shot) in column.iter().enumerate() {
            if let Some(Shot::Hit(boat)) = shot {
                hits.push((*boat, x, y));
            }
        }
    }

    hits
}

pub fn random_focus(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Option<Pos> {
    let hits = get_hits(shots);

    for boat in BOATS {
        let boat_hits = hits
            .iter()
            .filter_map(|hit| if hit.0 == boat {
                Some((hit.1, hit.2))
            } else {
                None
            });

        let hits_len = boat_hits.clone().count();

        if hits_len > 0 && hits_len < length(boat) {
            let boat_hits_vec: Vec<(usize, usize)> = boat_hits.collect();

            let pos = if hits_len == 1 {
                boat_hits_vec.first().copied().expect("No hits")
            }
            else {
                let min_pos = boat_hits_vec
                    .first().copied().expect("No hits");
                let max_pos = boat_hits_vec
                    .last().copied().expect("No hits");

                [min_pos, max_pos]
                    .choose(&mut rand::thread_rng())
                    .copied().expect("No hits")
            };

            return random_offset_shoot_pos(shots, pos!(pos))
        }
    }

    None
}

pub fn focus(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Option<Pos> {
    let hits = get_hits(shots);

    for boat in BOATS {
        let boat_hits = hits
            .iter()
            .filter(|hit| hit.0 == boat)
            .copied();

        let hits_len = boat_hits.clone().count();

        if hits_len > 1 && hits_len < length(boat) {
            let boat_hits_vec: Vec<(Boat, usize, usize)> = boat_hits.collect();

            let min_pos = boat_hits_vec
                .first().copied().expect("No boats");
            let max_pos = boat_hits_vec
                .last().copied().expect("No boats");

            let horizontal = max_pos.1 - min_pos.1 != 0;

            let mut possible_positions = vec![];

            if horizontal {
                if valid_shot_any(shots, min_pos.1 as i32 - 1, min_pos.2 as i32) {
                    possible_positions.push(pos!(min_pos.1 - 1, min_pos.2));
                }
                if valid_shot_any(shots, max_pos.1 as i32 + 1, max_pos.2 as i32) {
                    possible_positions.push(pos!(max_pos.1 + 1, max_pos.2));
                }
            } else {
                if valid_shot_any(shots, min_pos.1 as i32, min_pos.2 as i32 - 1) {
                    possible_positions.push(pos!(min_pos.1, min_pos.2 - 1));
                }
                if valid_shot_any(shots, max_pos.1 as i32, max_pos.2 as i32 + 1) {
                    possible_positions.push(pos!(max_pos.1, max_pos.2 + 1));
                }
            }

            return possible_positions.choose(&mut rand::thread_rng()).copied();
        } else if hits_len > 0 && hits_len < length(boat) {
            return random_focus(shots)
        }
    }

    None
}
