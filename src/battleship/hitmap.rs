use crate::battleship::constants::*;

#[derive(Clone, Copy)]
pub enum HitCell {
    Hit, Miss
}

pub struct HitMap {
    hits: [[Option<HitCell>; ROWS]; COLUMNS]
}

impl HitMap {
    pub fn empty() -> Self {
        Self {
            hits: [[None; ROWS]; COLUMNS]
        }
    }

    pub fn hit(&mut self, x: usize, y: usize) {
        self.hits[x][y] = Some(HitCell::Hit);
    }

    pub fn miss(&mut self, x: usize, y: usize) {
        self.hits[x][y] = Some(HitCell::Miss);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<HitCell> {
        self.hits[x][y]
    }
}