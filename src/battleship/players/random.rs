use rand::{random, Rng};

use crate::battleship::{Player, constants::{BOATS, NUM_COLS, NUM_ROWS, LENGTHS}};

use super::utils::valid_pos;

pub struct Random {

}

impl Random {
    fn random_pos(boat: usize) -> (bool, usize, usize) {
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

    fn random_valid_pos(boats: &[[usize; NUM_ROWS]; NUM_COLS], boat: usize) -> (bool, usize, usize) {
        let (mut horizontal, mut x, mut y) = Random::random_pos(boat);

        while !valid_pos(boats, boat, horizontal, x, y) {
            (horizontal, x, y) = Random::random_pos(boat);
        }

        (horizontal, x, y)
    }

    fn place_boat(boats: &mut [[usize; NUM_ROWS]; NUM_COLS], boat: usize) {
        let (horizontal, x, y) = Random::random_valid_pos(boats, boat);

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

    fn shoot() -> (usize, usize) {
        (0, 0)
    }
} 