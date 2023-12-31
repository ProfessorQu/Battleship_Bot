use std::fmt::Display;
use std::{fmt::Debug, fs::File};
use std::io::Write;

use crate::pos;
use crate::shoot::valid_shot;

use super::boat::{Boat, BOATS};
use super::constants::{NUM_ROWS, NUM_COLS, ShotMap, BoatMap, ShootFn, PlaceFn};
use super::position::Pos;
use super::shot::Shot;

type Fns = Vec<((&'static str, PlaceFn), (&'static str, ShootFn))>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    P1,
    P2
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::P1 => Self::P2,
            Player::P2 => Self::P1
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq)]
/// Stores data about a game
/// 
/// This struct stores data from a game, it is generated by [`play_and_record_game`](Battleship::play_and_record_game)
pub struct Recording {
    /// The boats for player 1
    pub player1_boats: BoatMap,
    /// The boats for player 2
    pub player2_boats: BoatMap,

    /// A list of all the shots player 1 took over the course of the game
    pub player1_shots: Vec<ShotMap>,
    /// A list of all the shots player 2 took over the course of the game
    pub player2_shots: Vec<ShotMap>,

    /// Get the winner of the recorded game
    pub winner: Player
}

impl Recording {
    fn new(
        player1_boats: BoatMap,
        player2_boats: BoatMap,

        player1_shots: Vec<ShotMap>,
        player2_shots: Vec<ShotMap>,

        winner: Player
    ) -> Self {
        Self {
            player1_boats,
            player2_boats,

            player1_shots,
            player2_shots,

            winner
        }
    }
}

#[derive(Clone)]
/// Handles the games
pub struct Battleship {
    current_player: Player,
    min_shots: usize,

    player1_boats: BoatMap,
    player2_boats: BoatMap,

    player1_shots: ShotMap,
    player2_shots: ShotMap,

    player1_shoot_fn: ShootFn,
    player2_shoot_fn: ShootFn,

    player1_place_fn: PlaceFn,
    player2_place_fn: PlaceFn,

    player1_last_shot: Pos,
    player2_last_shot: Pos,
}

impl Battleship {
    fn boats_valid(boats: BoatMap) -> bool {
        for boat in BOATS {
            let mut length = 0;
            for row in boats {
                for boat_item in row {
                    if boat_item == boat {
                        length += 1;
                    }
                }
            }

            if length < boat.length() {
                return false
            }
        }

        true
    }

    /// This function is used to create the game.
    /// The parameters are basically what they are named.
    /// 
    /// `player1_place_fn` and `player2_place_fn` can both be any function from [`place`](crate::place).
    /// 
    /// `player1_shoot_fn` and `player2_shoot_fn` can both be any function from [`shoot`](crate::shoot).
    pub fn new(
        player1_place_fn: PlaceFn, player2_place_fn: PlaceFn,
        player1_shoot_fn: ShootFn, player2_shoot_fn: ShootFn,
    ) -> Self {
        if !Battleship::boats_valid(player1_place_fn()) {
            panic!("Player 1 boat function isn't valid")
        }
        if !Battleship::boats_valid(player2_place_fn()) {
            panic!("Player 2 boat function isn't valid")
        }

        Self {
            current_player: Player::P1,
            min_shots: BOATS.iter().map(|boat| boat.length()).sum(),

            player1_boats: [[Boat::Empty; NUM_ROWS]; NUM_COLS],
            player2_boats: [[Boat::Empty; NUM_ROWS]; NUM_COLS],

            player1_shots: [[None; NUM_ROWS]; NUM_COLS],
            player2_shots: [[None; NUM_ROWS]; NUM_COLS],

            player1_shoot_fn,
            player2_shoot_fn,

            player1_place_fn,
            player2_place_fn,
            
            player1_last_shot: pos!(0, 0),
            player2_last_shot: pos!(0, 0)
        }
    }

    fn get_boats(&self, player: Player) -> [[Boat; NUM_ROWS]; NUM_COLS] {
        match player {
            Player::P1 => self.player1_boats,
            Player::P2 => self.player2_boats,
        }
    }

    fn get_shoot_fn(&self) -> ShootFn {
        match self.current_player {
            Player::P1 => self.player1_shoot_fn,
            Player::P2 => self.player2_shoot_fn,
        }
    }

    fn get_shots(&self, player: Option<Player>) -> ShotMap {
        match player {
            Some(Player::P1) => self.player1_shots,
            Some(Player::P2) => self.player2_shots,
            None => match self.current_player {
                Player::P1 => self.player1_shots,
                Player::P2 => self.player2_shots
            }
        }
    }

    fn set_shot(&mut self, pos: Pos, value: Shot) {
        match self.current_player {
            Player::P1 => self.player1_shots[pos.x][pos.y] = Some(value),
            Player::P2 => self.player2_shots[pos.x][pos.y] = Some(value),
        }
    }

    fn get_last_shot(&self) -> Pos {
        match self.current_player {
            Player::P1 => self.player1_last_shot,
            Player::P2 => self.player2_last_shot
        }
    }

    fn set_last_shot(&mut self, pos: Pos) {
        match self.current_player {
            Player::P1 => self.player1_last_shot = pos,
            Player::P2 => self.player2_last_shot = pos
        }
    }

    fn step(&mut self) {
        let (pos, new_last_pos) = (self.get_shoot_fn())
            (self.get_last_shot(), self.get_shots(None));


        debug_assert!(valid_shot(self.get_shots(None), pos));

        if new_last_pos {
            self.set_last_shot(pos);
        }
        self.shoot(pos);

        self.current_player = self.current_player.opponent();
    }

    fn reset(&mut self) {
        self.player1_boats = (self.player1_place_fn)();
        self.player2_boats = (self.player2_place_fn)();

        self.player1_shots = [[None; NUM_ROWS]; NUM_COLS];
        self.player2_shots = [[None; NUM_ROWS]; NUM_COLS];

        self.current_player = Player::P1;
    }

    fn shoot(&mut self, pos: Pos) {
        let (x, y) = (pos.x, pos.y);
        let boat = self.get_boats(self.current_player.opponent())[x][y];

        self.set_shot(pos!(x, y), if boat.has_some() {
            Shot::Hit(boat)
        } else {
            Shot::Miss
        });
    }

    fn winner(&self) -> Option<Player> {
        let mut player1_hits = 0;
        let mut player2_hits = 0;

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if let Some(shot) = self.player1_shots[x][y] {
                    if matches!(shot, Shot::Hit(_)) {
                        player1_hits += 1;
                    }
                }

                if let Some(shot) = self.player2_shots[x][y] {
                    if matches!(shot, Shot::Hit(_)) {
                        player2_hits += 1;
                    }
                }
            }
        }

        if player1_hits == self.min_shots {
            Some(Player::P1)
        } else if player2_hits == self.min_shots {
            Some(Player::P2)
        } else {
            None
        }
    }

    fn play_game(&mut self) -> Player {
        self.reset();

        let mut winner = None;

        while winner.is_none() {
            self.step();
            winner = self.winner();
        }

        winner.expect("Noone won")
    }

    /// This function allows you to record a game and get data from it.
    /// It will return a [`Recording`] struct, which has a lot of handy features.
    /// # Example
    /// ```rust
    /// use battleship_bot::Battleship;
    /// use battleship_bot::{place, shoot};
    /// 
    /// let mut battleship = Battleship::new(
    ///     place::random,
    ///     place::random,
    /// 
    ///     shoot::random,
    ///     shoot::grid_and_destroy,
    /// );
    /// 
    /// let recording = battleship.play_and_record_game();
    /// 
    /// // Small chance this assertion will fail, but it's really small
    /// assert_ne!(recording.player1_boats, recording.player2_boats);
    /// 
    /// println!("Player {} won!", recording.winner);
    /// ```
    pub fn play_and_record_game(&mut self) -> Recording {
        let mut player1_shots = vec![];
        let mut player2_shots = vec![];

        let mut winner = None;
        self.reset();

        while winner.is_none() {
            let player = self.current_player;
            self.step();

            match player {
                Player::P1 => player1_shots.push(self.get_shots(Some(Player::P1))),
                Player::P2 => player2_shots.push(self.get_shots(Some(Player::P2))),
            }

            winner = self.winner();
        }

        Recording::new(
            self.get_boats(Player::P1),
            self.get_boats(Player::P2),
            
            player1_shots,
            player2_shots,

            winner.expect("Noone won")
        )
    }

    /// This function allows you to have the bots play many games.
    /// The only parameter is a usize, and that is the number of games the bots will play agains each other.
    /// It will return a tuple of player 1 wins and player 2 wins, respectively.
    /// 
    /// # Example
    /// ```rust
    /// use battleship_bot::Battleship;
    /// use battleship_bot::{place, shoot};
    /// 
    /// let mut battleship = Battleship::new(
    ///     place::random,
    ///     place::random,
    /// 
    ///     shoot::random,
    ///     shoot::grid_and_destroy,
    /// );
    /// 
    /// // When playing multiple games, it will return a tuple of player 1 wins and player 2 wins
    /// let (p1_wins, p2_wins) = battleship.play_games(1000);
    /// 
    /// assert!(p2_wins > p1_wins);
    /// ```
    pub fn play_games(&mut self, num_games: usize) -> (usize, usize) {
        let mut p1_won = 0;
        let mut p2_won = 0;

        for _ in 0..num_games {
            let winner = self.play_game();

            if matches!(winner, Player::P1) {
                p1_won += 1;
            } else if matches!(winner, Player::P2) {
                p2_won += 1;
            }
        }

        (p1_won, p2_won)
    }

    /// Saves games from the inputs
    /// 
    /// Saves games against all possible combinations of place_fns and shoot_fns.
    /// It runs games_per_comb for each combination of place_fns and shoot_fns.
    /// After all the games it stores the data in filename, which is a csv file.
    pub fn save_games(
        place_fns: Vec<(&'static str, PlaceFn)>,
        shoot_fns: Vec<(&'static str, ShootFn)>,
        games_per_comb: usize,
        filename: &str
    ) {
        let csv_filename =
        if !filename.ends_with(".csv") {
            filename.to_owned() + ".csv"
        } else {
            filename.to_owned()
        };

        let mut file = File::create(csv_filename).expect("Failed to create file");

        write!(file, "Placing,").expect("Failed to write to file");
        for (place_name, _) in place_fns.iter() {
            for _ in 0..shoot_fns.len() {
                write!(file, ",{}", place_name).expect("Failed to write to file");
            }
        }
        writeln!(file).expect("Failed to write to file");

        write!(file, ",Shooting").expect("Failed to write to file");
        for _ in 0..place_fns.len() {
            for (shoot_name, _) in shoot_fns.iter() {
                write!(file, ",{}", shoot_name).expect("Failed to write to file");
            }
        }

        writeln!(file).expect("Failed to write to file");

        let fns: Fns = place_fns
            .iter().zip(shoot_fns.iter())
            .map(|item| (*item.0, *item.1))
            .collect();

        for (
            (p1_place_fn_name, p1_place_fn),
            (p1_shoot_fn_name, p1_shoot_fn)
        ) in fns.iter() {
            let mut winrates = vec![];

            for (
                (_, p2_place_fn),
                (_, p2_shoot_fn)
                ) in fns.iter() {
                let mut battleship = Battleship::new(
                    *p1_place_fn,
                    *p2_place_fn,

                    *p1_shoot_fn,
                    *p2_shoot_fn,
                );

                let (p1_wins, _) = battleship.play_games(games_per_comb);
                let p1_winrate = p1_wins as f32 / games_per_comb as f32 * 100.0;

                winrates.push(p1_winrate);
            }

            println!("Writing {}, {}...", p1_shoot_fn_name, p1_place_fn_name);

            write!(file, "{},{}", p1_shoot_fn_name, p1_place_fn_name).expect("Failed to write to file");
            for winrate in winrates {
                write!(file, ",{:.1}", winrate).expect("Failed to write to file");
            }
            writeln!(file).expect("Failed to write to file");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{place, shoot};

    use super::*;

    #[test]
    fn test_battleship() {
        const NUM_GAMES: usize = 1_000;

        let mut game = Battleship::new(
            place::random,
            place::random,

            shoot::random,
            shoot::random,
        );

        let (p1_wins, p2_wins) = game.play_games(NUM_GAMES);
        assert!(p1_wins > p2_wins);

        let mut game = Battleship::new(
            place::sides,
            place::sides,

            shoot::random,
            shoot::heatmap_and_destroy,
        );

        let (p1_wins, p2_wins) = game.play_games(NUM_GAMES);
        assert!(p2_wins > p1_wins);

        let mut game = Battleship::new(
            place::spread,
            place::spread,

            shoot::grid_and_destroy,
            shoot::heatmap_and_destroy,
        );

        let (p1_wins, p2_wins) = game.play_games(NUM_GAMES);
        assert!(p2_wins > p1_wins);
    }
}