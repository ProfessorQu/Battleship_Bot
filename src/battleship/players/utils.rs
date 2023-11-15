use crate::battleship::constants::{NUM_ROWS, NUM_COLS, LENGTHS};

pub fn valid_pos(
    boats: &[[usize; NUM_ROWS]; NUM_COLS], boat: usize,
    horizontal: bool, x: usize, y: usize
) -> bool {

    let mut valid_position = true;

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

    valid_position
}