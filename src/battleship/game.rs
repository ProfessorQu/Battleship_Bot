use std::fmt::Debug;

use super::{constants::{NUM_ROWS, NUM_COLS, MIN_SHOTS, Boat, EMPTY}, players::Pos};

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

#[derive(Debug, Clone, Copy)]
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

type ShootFn = fn([[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> Pos;
type PlaceFn = fn() -> [[Boat; NUM_ROWS]; NUM_COLS];

pub struct Battleship {
    current_player: Player,

    player1_boats: [[Boat; NUM_ROWS]; NUM_COLS],
    player2_boats: [[Boat; NUM_ROWS]; NUM_COLS],

    player1_shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS],
    player2_shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS],

    player1_shoot_fn: ShootFn,
    player2_shoot_fn: ShootFn,

    player1_place_fn: PlaceFn,
    player2_place_fn: PlaceFn,
}

impl Battleship {
    pub fn new(
        player1_place_fn: PlaceFn, player2_place_fn: PlaceFn,
        player1_shoot_fn: ShootFn, player2_shoot_fn: ShootFn
    ) -> Self {
        Self {
            current_player: Player::P1,

            player1_boats: [[EMPTY; NUM_ROWS]; NUM_COLS],
            player2_boats: [[EMPTY; NUM_ROWS]; NUM_COLS],

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

    fn get_shots(&self, player: Player) -> [[Option<Shot>; NUM_ROWS]; NUM_COLS] {
        match player {
            Player::P1 => self.player1_shots,
            Player::P2 => self.player2_shots,
        }
    }

    fn get_shots_ref(&mut self, player: Player) -> &mut [[Option<Shot>; NUM_ROWS]; NUM_COLS] {
        match player {
            Player::P1 => &mut self.player1_shots,
            Player::P2 => &mut self.player2_shots,
        }
    }

    fn step(&mut self) {
        let pos = (self.get_shoot_fn(self.current_player))(self.get_shots(self.current_player));
        self.shoot(self.current_player, pos);
        self.current_player = self.current_player.opponent();
    }

    fn reset(&mut self) {
        self.player1_boats = (self.player1_place_fn)();
        self.player2_boats = (self.player2_place_fn)();

        self.player1_shots = [[None; NUM_ROWS]; NUM_COLS];
        self.player2_shots = [[None; NUM_ROWS]; NUM_COLS];

        self.current_player = Player::P1;
    }

    fn play(&mut self) -> Player {
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
            let winner = self.play();

            if matches!(winner, Player::P1) {
                p1_won += 1;
            } else if matches!(winner, Player::P2) {
                p2_won += 1;
            }
        }

        (p1_won, p2_won)
    }

    fn shoot(&mut self, player: Player, pos: Pos) {
        let (x, y) = (pos.x, pos.y);
        let boat = self.get_boats(player.opponent())[x][y];

        self.get_shots_ref(player)[x][y] = if boat != EMPTY {
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

        if player1_hits == MIN_SHOTS {
            Some(Player::P1)
        } else if player2_hits == MIN_SHOTS {
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

            if matches!(self.current_player, Player::P2) {
            //     println!("TURN =========================================================================");
            //     self.show_boats(Player::P1);
                self.show_shots(Player::P2);
            }

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
}