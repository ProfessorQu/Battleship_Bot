use rand::{random, Rng};

use crate::battleship::{Player, constants::{BOATS, COLUMNS, ROWS, LENGTHS}};

pub struct Random {

}

impl Random {
    fn random_pos(boat: usize) -> (bool, usize, usize) {
        let horizontal: bool = random();

        let (x_range, y_range) = if horizontal {
            (0..(COLUMNS - LENGTHS[boat]), 0..ROWS)
        } else {
            (0..COLUMNS, 0..(ROWS - LENGTHS[boat]))
        };

        let mut random = rand::thread_rng();
        
        (
            horizontal,
            random.gen_range(x_range),
            random.gen_range(y_range)
        )
    }

    fn random_valid_pos(boats: &[[usize; COLUMNS]; ROWS], boat: usize) -> (bool, usize, usize) {
        let mut valid_position = false;

        let (mut horizontal, mut x, mut y) = Random::random_pos(boat);

        while !valid_position {
            (horizontal, x, y) = Random::random_pos(boat);

            valid_position = true;

            if horizontal {
                for x_off in 0..LENGTHS[boat] {
                    if boats[x + x_off][y] != 0 {
                        valid_position = false;
                        break;
                    }
                }
            }
            else {
                for y_off in 0..LENGTHS[boat] {
                    if boats[x][y + y_off] != 0 {
                        valid_position = false;
                        break;
                    }
                }
            }
        }

        (horizontal, x, y)
    }

    fn place_boat(boats: &mut [[usize; COLUMNS]; ROWS], boat: usize) {
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
    fn place_boats() -> [[usize; COLUMNS]; ROWS] {
        let mut boats = [[0;COLUMNS]; ROWS];

        for boat in BOATS {
            Random::place_boat(&mut boats, boat);
        }

        boats
    }

    fn shoot() -> (usize, usize) {
        (0, 0)
    }
} 