use std::fmt::Display;
use std::{fmt::Debug, fs::File};
use std::io::Write;

use crate::battleship::player::destroy::valid_shot;

use super::Pos;
use super::boat::{Boat, BOATS};
use super::constants::{NUM_ROWS, NUM_COLS, ShotMap, BoatMap, ShootFn, PlaceFn, SHOOT_FNS, PLACE_FNS, FNS};

#[derive(Clone, Copy)]
pub enum Shot {
    Hit(Boat),
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

impl Display for Shot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hit(boat) => write!(f, "{}", boat),
            Self::Miss => write!(f, "M")
        }
    }
}

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

pub struct Recording {
    pub player1_boats: BoatMap,
    pub player2_boats: BoatMap,

    pub player1_shots: Vec<ShotMap>,
    pub player2_shots: Vec<ShotMap>,

    pub all_shots: Vec<ShotMap>,
}

impl Recording {
    fn new(
        player1_boats: BoatMap,
        player2_boats: BoatMap,

        player1_shots: Vec<ShotMap>,
        player2_shots: Vec<ShotMap>,
    ) -> Self {
        let all_shots = Recording::get_all_shots(
            &player1_shots,
            &player2_shots
        );

        Self {
            player1_boats,
            player2_boats,

            player1_shots,
            player2_shots,

            all_shots
        }
    }

    fn get_all_shots(player1_shots: &Vec<ShotMap>, player2_shots: &Vec<ShotMap>) -> Vec<ShotMap> {
        let mut all_shots = vec![];

        for i in 0..player1_shots.len() {
            all_shots.push(player1_shots[i]);
            if i < player2_shots.len() {
                all_shots.push(player2_shots[i]);
            }
        }

        all_shots
    }
}

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
}

impl Battleship {
    pub fn new(
        player1_place_fn: PlaceFn, player2_place_fn: PlaceFn,
        player1_shoot_fn: ShootFn, player2_shoot_fn: ShootFn,
    ) -> Self {
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
            player2_place_fn
        }
    }

    fn get_boats(&self, player: Player) -> [[Boat; NUM_ROWS]; NUM_COLS] {
        match player {
            Player::P1 => self.player1_boats,
            Player::P2 => self.player2_boats,
        }
    }

    fn get_shoot_fn(&self, player: Player) -> ShootFn {
        match player {
            Player::P1 => self.player1_shoot_fn,
            Player::P2 => self.player2_shoot_fn,
        }
    }

    fn get_shots(&self, player: Player) -> ShotMap {
        match player {
            Player::P1 => self.player1_shots,
            Player::P2 => self.player2_shots,
        }
    }

    fn get_shots_ref(&mut self, player: Player) -> &mut ShotMap {
        match player {
            Player::P1 => &mut self.player1_shots,
            Player::P2 => &mut self.player2_shots,
        }
    }

    fn step(&mut self) {
        let pos = (self.get_shoot_fn(self.current_player))
            (self.current_player, self.get_shots(self.current_player));
        
        debug_assert!(valid_shot(self.get_shots(self.current_player), pos));

        self.shoot(self.current_player, pos);

        self.current_player = self.current_player.opponent();
    }

    pub fn reset(&mut self) {
        self.player1_boats = (self.player1_place_fn)();
        self.player2_boats = (self.player2_place_fn)();

        self.player1_shots = [[None; NUM_ROWS]; NUM_COLS];
        self.player2_shots = [[None; NUM_ROWS]; NUM_COLS];

        self.current_player = Player::P1;
    }

    fn play_game(&mut self) -> Player {
        self.reset();

        let mut winner = None;

        while winner.is_none() {
            self.step();
            winner = self.winner();
        }

        winner.expect("It's a draw?")
    }

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

    pub fn save_games(games_per_fn: usize, filename: &str) {
        let mut file = File::create(filename).expect("Failed to create file");

        write!(file, ",").expect("FIALED");
        for (place_name, _) in PLACE_FNS.iter() {
            for _ in 0..SHOOT_FNS.len() {
                write!(file, ",{}", place_name).expect("Failed to write");
            }
        }
        writeln!(file).expect("Failed to write");

        write!(file, ",").expect("FIALED");
        for _ in 0..PLACE_FNS.len() {
            for (shoot_name, _) in SHOOT_FNS.iter() {
                write!(file, ",{}", shoot_name).expect("Failed to write");
            }
        }

        writeln!(file).expect("Failed to write to file");

        for (
            (p1_place_fn_name, p1_place_fn),
            (p1_shoot_fn_name, p1_shoot_fn)
        ) in FNS.iter() {
            let mut winrates = vec![];

            for (
                (_, p2_place_fn),
                (_, p2_shoot_fn)
                ) in FNS.iter() {
                let mut battleship = Battleship::new(
                    *p1_place_fn,
                    *p2_place_fn,

                    *p1_shoot_fn,
                    *p2_shoot_fn,
                );

                let (p1_wins, _) = battleship.play_games(games_per_fn);
                let p1_winrate = p1_wins as f32 / games_per_fn as f32 * 100.0;

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

    fn shoot(&mut self, player: Player, pos: Pos) {
        let (x, y) = (pos.x, pos.y);
        let boat = self.get_boats(player.opponent())[x][y];

        self.get_shots_ref(player)[x][y] = if boat.has_some() {
            Some(Shot::Hit(boat))
        } else {
            Some(Shot::Miss)
        };
    }

    fn show_boats(&self, player: Player) {
        let boats = self.get_boats(player);

        println!("{}", "-".repeat(boats.len() * 3 + 2));
        for y in 0..NUM_ROWS {
            print!("|");
            for column in boats {
                let boat = column[y];
                let value = if boat.is_empty() {
                    "-".to_string()
                } else {
                    boat.to_string()
                };

                print!(" {} ", value);
            }
            println!("|");
        }
        println!("{}", "-".repeat(boats.len() * 3 + 2));
    }

    fn show_shots(&self, player: Player) {
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

    pub fn winner(&self) -> Option<Player> {
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

    pub fn play_and_show_game(&mut self) {
        let mut winner = None;
        self.reset();

        while winner.is_none() {
            self.step();

            winner = self.winner();
        }

        println!("P1 BOATS ===============");
        self.show_boats(Player::P1);
        self.show_shots(Player::P2);

        println!("P2 BOATS ===============");
        self.show_boats(Player::P2);
        self.show_shots(Player::P1);

        println!("{:?} won", winner.expect("Noone won"))
    }

    pub fn play_and_record_game(&mut self) -> Recording {
        let mut player1_shots = vec![];
        let mut player2_shots = vec![];

        let mut winner = None;
        self.reset();

        while winner.is_none() {
            let player = self.current_player;
            self.step();

            match player {
                Player::P1 => player1_shots.push(self.get_shots(Player::P1)),
                Player::P2 => player2_shots.push(self.get_shots(Player::P2)),
            }

            winner = self.winner();
        }

        Recording::new(
            self.get_boats(Player::P1),
            self.get_boats(Player::P2),
            
            player1_shots,
            player2_shots
        )
    }
}