use crate::battleship::boatmap::{ BoatCell, BoatMap };
use crate::battleship::hitmap::HitMap;
use crate::battleship::constants::PlayerType;

pub struct Game {
    hitmap_player: HitMap,
    hitmap_bot: HitMap,
    boatmap_player: BoatMap,
    boatmap_bot: BoatMap,
}

impl Game {
    pub fn new() -> Self {
        Self {
            hitmap_player: HitMap::empty(),
            hitmap_bot: HitMap::empty(),
            boatmap_player: BoatMap::empty(),
            boatmap_bot: BoatMap::empty(),
        }
    }

    pub fn get_hitmap(&self, player_type: PlayerType) -> &HitMap {
        match player_type {
            PlayerType::Player => &self.hitmap_player,
            PlayerType::Bot => &self.hitmap_bot
        }
    }

    pub fn get_boatmap(&self, player_type: PlayerType) -> &BoatMap {
        match player_type {
            PlayerType::Player => &self.boatmap_player,
            PlayerType::Bot => &self.boatmap_bot,
        }
    }

    pub fn place_boat(&mut self, player_type: PlayerType, x: usize, y: usize, boat: BoatCell) {
        let length = match boat {
            BoatCell::Carrier => 5,
            BoatCell::Battleship => 4,
            BoatCell::Cruiser => 3,
            BoatCell::Submarine => 3,
            BoatCell::Destroyer => 2,
        };

        match player_type {
            PlayerType::Player => for offset in 0..length {
                self.boatmap_player.set(x + offset, y, boat);
            }
            PlayerType::Bot => for offset in 0..length {
                self.boatmap_bot.set(x + offset, y, boat);
            }
        }
    }

    pub fn player_make_move(&mut self, x: usize, y: usize) {
        if self.hitmap_player.get(x, y).is_some() {
            return
        }

        if self.boatmap_bot.is_hit(x, y) {
            self.hitmap_player.hit(x, y);
        }
        else {
            self.hitmap_player.miss(x, y);
        }
    }

    pub fn bot_make_move(&mut self) {
        let (x, y) = self.make_decision();

        if self.hitmap_bot.get(x, y).is_some() {
            return
        }

        if self.boatmap_player.is_hit(x, y) {
            self.hitmap_bot.hit(x, y);
        }
        else {
            self.hitmap_bot.miss(x, y);   
        }
    }

    pub fn make_decision(&self) -> (usize, usize) {
        (0, 0)
    }
}