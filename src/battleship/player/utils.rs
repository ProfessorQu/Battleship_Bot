use crate::battleship::{constants::ShotMap, boat::Boat, game::Shot};

pub fn get_hits(shots: ShotMap) -> Vec<(Boat, usize, usize)> {
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