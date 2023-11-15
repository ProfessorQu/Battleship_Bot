use std::fmt::Debug;

use super::constants::{NUM_ROWS, NUM_COLS, MIN_SHOTS};

#[derive(Clone, Copy)]
pub enum Shot {
    Hit(usize),
    Miss
}

impl Debug for Shot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hit(boat) => write!(f, "{}", boat),
            Self::Miss => write!(f, "M"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerType {
    P1,
    P2
}

impl PlayerType {
    pub fn opponent(&self) -> Self {
        match self {
            PlayerType::P1 => Self::P2,
            PlayerType::P2 => Self::P1
        }
    }
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

    fn get_boats(&self, player: PlayerType) -> [[usize; NUM_ROWS]; NUM_COLS] {
        match player {
            PlayerType::P1 => self.player1_boats,
            PlayerType::P2 => self.player2_boats,
        }
    }

    pub fn get_shots(&self, player: PlayerType) -> [[Option<Shot>; NUM_ROWS]; NUM_COLS] {
        match player {
            PlayerType::P1 => self.player1_shots,
            PlayerType::P2 => self.player2_shots,
        }
    }

    pub fn get_shots_ref(&mut self, player: PlayerType) -> &mut [[Option<Shot>; NUM_ROWS]; NUM_COLS] {
        match player {
            PlayerType::P1 => &mut self.player1_shots,
            PlayerType::P2 => &mut self.player2_shots,
        }
    }

    pub fn shoot(&mut self, player: PlayerType, pos: (usize, usize)) {
        let (x, y) = pos;
        let boat = self.get_boats(player.opponent())[x][y];

        self.get_shots_ref(player)[x][y] = if boat > 0 {
            Some(Shot::Hit(boat))
        } else {
            Some(Shot::Miss)
        };
    }

    pub fn show_boats(&self, player: PlayerType) {
        let boats = self.get_boats(player);

        println!("{}", "-".repeat(boats.len() * 3 + 2));
        for y in 0..NUM_ROWS {
            print!("|");
            for column in boats {
                let element = column[y];
                let value = if element == 0 {
                    "-".to_string()
                } else {
                    element.to_string()
                };

                print!(" {} ", value);
            }
            println!("|");
        }
        println!("{}", "-".repeat(boats.len() * 3 + 2));
    }

    pub fn show_shots(&self, player: PlayerType) {
        let shots = self.get_shots(player);

        println!("{}", "-".repeat(shots.len() * 3 + 2));
        for y in 0..NUM_ROWS {
            print!("|");
            for column in shots {
                let element = column[y];
                if let Some(value) = element {
                    print!(" {:?} ", value);
                } else {
                    print!(" - ");
                }

            }
            println!("|");
        }
        println!("{}", "-".repeat(shots.len() * 3 + 2));
    }

    pub fn won(&self) -> Option<PlayerType> {
        let player1_total_shots: Vec<Option<Shot>> = self.player1_shots
            .iter()
            .flat_map(|array| array.iter())
            .cloned().collect();
        let player1_hits: Vec<Shot> = player1_total_shots
            .iter()
            .flatten()
            .filter(|item| matches!(item, Shot::Hit(_)))
            .cloned().collect();
        let player2_total_shots: Vec<Option<Shot>> = self.player2_shots.iter().flat_map(|array| array.iter()).cloned().collect();
        let player2_hits: Vec<Shot> = player2_total_shots
            .iter()
            .flatten()
            .filter(|item| matches!(item, Shot::Hit(_)))
            .cloned().collect();

        if player1_total_shots.len() < MIN_SHOTS {
            None
        }
        else if player1_hits.len() == MIN_SHOTS {
            Some(PlayerType::P1)
        } else if player2_hits.len() == MIN_SHOTS {
            Some(PlayerType::P2)
        } else {
            None
        }
    }
}