use super::constants::{NUM_ROWS, NUM_COLS};

#[derive(Clone, Copy)]
pub enum Shot {
    Hit(usize),
    Miss
}

pub struct Game {
    player1_boats: [[usize; NUM_ROWS]; NUM_COLS],
    player2_boats: [[usize; NUM_ROWS]; NUM_COLS],

    player1_shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS],
    player2_shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS],
}

impl Game {
    pub fn new(player1_boats: [[usize; NUM_ROWS]; NUM_COLS], player2_boats: [[usize; NUM_ROWS]; NUM_COLS]) -> Self {
        Self {
            player1_boats,
            player2_boats,

            player1_shots: [[None; NUM_ROWS]; NUM_COLS],
            player2_shots: [[None; NUM_ROWS]; NUM_COLS],
        }
    }
}