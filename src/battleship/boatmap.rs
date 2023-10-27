use crate::battleship::constants::*;

#[derive(Clone, Copy)]
pub enum BoatCell {
    Carrier,    // 5
    Battleship, // 4
    Cruiser,    // 3
    Submarine,  // 3
    Destroyer   // 2
}

#[derive(Clone)]
pub struct BoatMap {
    boats: [[Option<BoatCell>; ROWS]; COLUMNS]
}

impl BoatMap {
    pub fn empty() -> Self {
        Self {
            boats: [[None; ROWS]; COLUMNS]
        }
    }

    pub fn is_hit(&self, x: usize, y: usize) -> bool {
        self.boats[x][y].is_some()
    }

    pub fn set(&mut self, x: usize, y: usize, boat: BoatCell) {
        self.boats[x][y] = Some(boat);
    }
}