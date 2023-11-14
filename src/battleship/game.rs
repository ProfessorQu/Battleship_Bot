use super::constants::{ROWS, COLUMNS};

pub struct Game {
    player1_boats: [[usize; COLUMNS]; ROWS],
    player2_boats: [[usize; COLUMNS]; ROWS]
}

impl Game {
    pub fn new(player1_boats: [[usize; COLUMNS]; ROWS], player2_boats: [[usize; COLUMNS]; ROWS]) -> Self {
        Self {
            player1_boats,
            player2_boats
        }
    }
}