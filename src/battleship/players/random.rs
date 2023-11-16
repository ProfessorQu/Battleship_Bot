use rand::{random, Rng, seq::SliceRandom};

use crate::battleship::{constants::{BOATS, NUM_COLS, NUM_ROWS, LENGTHS}, game::Shot};

use super::utils::{valid_pos, valid_shot};

pub struct Random { }

impl Random {
    fn random_boat_pos(boat: usize) -> (bool, usize, usize) {
        let horizontal: bool = random();

        let (x_range, y_range) = if horizontal {
            (
                0..(NUM_COLS - LENGTHS[boat]),
                0..NUM_ROWS
            )
        } else {
            (
                0..NUM_COLS,
                0..(NUM_ROWS - LENGTHS[boat])
            )
        };

        let mut rng = rand::thread_rng();
        
        (
            horizontal,
            rng.gen_range(x_range),
            rng.gen_range(y_range)
        )
    }

    fn random_valid_boat_pos(boats: &[[usize; NUM_ROWS]; NUM_COLS], boat: usize) -> (bool, usize, usize) {
        let (mut horizontal, mut x, mut y) = Random::random_boat_pos(boat);

        while !valid_pos(boats, boat, horizontal, x, y) {
            (horizontal, x, y) = Random::random_boat_pos(boat);
        }

        (horizontal, x, y)
    }

    fn place_boat(boats: &mut [[usize; NUM_ROWS]; NUM_COLS], boat: usize) {
        let (horizontal, x, y) = Random::random_valid_boat_pos(boats, boat);

        if horizontal {
            for x_off in 0..LENGTHS[boat] {
                boats[x + x_off][y] = boat;
            }
        }
        else {
            for y_off in 0..LENGTHS[boat] {
                boats[x][y + y_off] = boat;
            }
        }
    }

    pub fn place_boats() -> [[usize; NUM_ROWS]; NUM_COLS] {
        let mut boats = [[0; NUM_ROWS]; NUM_COLS];

        for boat in BOATS {
            Random::place_boat(&mut boats, boat);
        }

        boats
    }

    pub fn shoot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize) {
        let mut rng = rand::thread_rng();

        let (mut x, mut y) = (
            rng.gen_range(0..NUM_COLS),
            rng.gen_range(0..NUM_ROWS)
        );

        while !valid_shot(shots, x, y) {
            (x, y) = (
                rng.gen_range(0..NUM_COLS),
                rng.gen_range(0..NUM_ROWS)
            );
        }

        (x, y)
    }

    fn offset_pos(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS], x: usize, y: usize) -> (usize, usize) {
        let mut positions = vec![];

        if x as i32 - 1 > 0 && valid_shot(shots, x - 1, y) {
            positions.push((x - 1, y));
        }
        if x + 1 < NUM_COLS && valid_shot(shots, x + 1, y) {
            positions.push((x + 1, y));
        }
        if y as i32 - 1 > 0 && valid_shot(shots, x, y - 1) {
            positions.push((x, y - 1));
        }
        if y + 1 < NUM_ROWS && valid_shot(shots, x, y + 1) {
            positions.push((x, y + 1));
        }

        let rand_pos = positions.choose(&mut rand::thread_rng());

        if let Some(pos) = rand_pos {
            *pos
        } else {
            Random::shoot(shots)
        }
    }

    pub fn shoot_and_focus(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize) {
        let mut hits = vec![];

        for (x, column) in shots.iter().enumerate() {
            for (y, shot) in column.iter().enumerate() {
                if let Some(Shot::Hit(boat)) = shot {
                    hits.push((*boat, x, y));
                }
            }
        }
    
        for boat in BOATS {
            let boat_hits = hits.iter().filter(|hit| hit.0 == boat);
            let hits_len = boat_hits.clone().count();

            if hits_len < LENGTHS[boat] && hits_len > 0 {
                let (_, x, y) = boat_hits
                .collect::<Vec<&(usize, usize, usize)>>()
                .choose(&mut rand::thread_rng()).expect("No boat hits");

                return Random::offset_pos(shots, *x, *y)
            }
        }

        Random::shoot(shots)
    }
} 