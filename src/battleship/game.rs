use super::constants::{NUM_ROWS, NUM_COLS};

#[derive(Clone, Copy)]
pub enum Shot {
    Hit(usize),
    Miss
}

pub enum PlayerType {
    P1,
    P2
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

    pub fn show_boats(&self, player: PlayerType) {
        let boats = match player {
            PlayerType::P1 => self.player1_boats,
            PlayerType::P2 => self.player2_boats,
        };

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
}