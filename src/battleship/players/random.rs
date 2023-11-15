use rand::{random, Rng};

use crate::battleship::{Player, constants::{BOATS, NUM_COLS, NUM_ROWS, LENGTHS}, game::Shot};

use super::utils::{valid_pos, valid_shot};

pub struct Random {

}

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

        let mut random = rand::thread_rng();
        
        (
            horizontal,
            random.gen_range(x_range),
            random.gen_range(y_range)
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
}

impl Player for Random {
    fn place_boats() -> [[usize; NUM_ROWS]; NUM_COLS] {
        let mut boats = [[0; NUM_ROWS]; NUM_COLS];

        for boat in BOATS {
            Random::place_boat(&mut boats, boat);
        }

        boats
    }

    fn shoot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize) {
        let mut random = rand::thread_rng();

        let (mut x, mut y) = (
            random.gen_range(0..NUM_COLS),
            random.gen_range(0..NUM_ROWS)
        );

        while !valid_shot(shots, x, y) {
            (x, y) = (
                random.gen_range(0..NUM_COLS),
                random.gen_range(0..NUM_ROWS)
            );
        }

        (x, y)
    }
} 