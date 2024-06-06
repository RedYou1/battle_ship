use std::collections::HashMap;

use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ship {
    PatrolBoat,
    Submarine,
    Destroyer,
    Battleship,
    Carrier,
}

impl Ship {
    pub const fn len(self) -> usize {
        match self {
            Self::PatrolBoat => 2,
            Self::Submarine | Self::Destroyer => 3,
            Self::Battleship => 4,
            Self::Carrier => 5,
        }
    }
    pub const fn char(self) -> char {
        match self {
            Self::PatrolBoat => 'P',
            Self::Submarine => 'S',
            Self::Destroyer => 'D',
            Self::Battleship => 'B',
            Self::Carrier => 'C',
        }
    }
}

impl TryFrom<&str> for Ship {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "P" => Ok(Self::PatrolBoat),
            "S" => Ok(Self::Submarine),
            "D" => Ok(Self::Destroyer),
            "B" => Ok(Self::Battleship),
            "C" => Ok(Self::Carrier),
            _ => Err(()),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Slot {
    #[default]
    None,
    NoneShot,
    Ship(Ship),
    Hit(Ship),
}

impl Slot {
    pub const fn char(self, same_player: bool) -> char {
        match (same_player, self) {
            (_, Self::None) | (false, Self::Ship(_)) => '_',
            (_, Self::NoneShot) => '#',
            (true, Self::Ship(ship)) => ship.char(),
            (_, Self::Hit(_)) => 'X',
        }
    }
}

pub enum State {
    Placing,
    Playing,
}

pub const WIDTH: usize = 21;
pub const HEIGHT: usize = 6;

pub struct Game {
    player: [[Slot; WIDTH]; HEIGHT],
    bot: [[Slot; WIDTH]; HEIGHT],
    state: State,
}

pub enum End {
    Error,
    Continue,
    Win,
    Lose,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Default::default(),
            bot: Default::default(),
            state: State::Placing,
        }
    }

    pub fn place(&mut self, ship: Ship, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<()> {
        Game::place_in(&mut self.player, ship, x1, y1, x2, y2)
    }
    #[allow(clippy::needless_range_loop)]
    pub fn place_in(
        board: &mut [[Slot; WIDTH]; HEIGHT],
        ship: Ship,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> Option<()> {
        if x1 >= WIDTH || x2 >= WIDTH || y1 >= HEIGHT || y2 >= HEIGHT {
            return None; //OutOfBound
        }
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        let diff1 = x2 - x1;
        let diff2 = y2 - y1;
        if (diff1 > 0) == (diff2 > 0) {
            return None; //Diagonal
        }
        let dir = diff1 > 0; // true => horizontal, false => vertical
        let len = if dir { diff1 } else { diff2 };
        if len != ship.len() - 1 {
            return None; //pas la bonne longueur
        }
        if dir {
            for x in x1..=x2 {
                if board[y1][x] != Slot::None && board[y1][x] != Slot::Ship(ship) {
                    return None;
                }
            }
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if board[y][x] == Slot::Ship(ship) {
                        board[y][x] = Slot::None;
                    }
                }
            }
            for x in x1..=x2 {
                board[y1][x] = Slot::Ship(ship);
            }
        } else {
            for y in y1..=y2 {
                if board[y][x1] != Slot::None && board[y][x1] != Slot::Ship(ship) {
                    return None;
                }
            }
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if board[y][x] == Slot::Ship(ship) {
                        board[y][x] = Slot::None;
                    }
                }
            }
            for y in y1..=y2 {
                board[y][x1] = Slot::Ship(ship);
            }
        }
        Some(())
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn random(&mut self, ships: &mut HashMap<Ship, bool>) {
        let mut rng = rand::thread_rng();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.player[y][x] = Slot::None;
            }
        }
        for ship in [
            Ship::PatrolBoat,
            Ship::Submarine,
            Ship::Destroyer,
            Ship::Battleship,
            Ship::Carrier,
        ] {
            let len = ship.len();
            loop {
                let dir = rng.gen_bool(0.5); // true => horizontal, false => vertical
                let x1 = rng.gen_range(0..(WIDTH - if dir { len - 1 } else { 0 }));
                let y1 = rng.gen_range(0..(HEIGHT - if dir { 0 } else { len - 1 }));
                let len = len as i8;
                let (x2, y2) = if dir {
                    ((x1 as i8 + rng.gen_range((-len + 1)..len)) as usize, y1)
                } else {
                    (x1, (y1 as i8 + rng.gen_range((-len + 1)..len)) as usize)
                };
                if self.place(ship, x1, y1, x2, y2).is_some() {
                    break;
                }
            }
        }
        *ships = HashMap::from([
            (Ship::PatrolBoat, true),
            (Ship::Submarine, true),
            (Ship::Destroyer, true),
            (Ship::Battleship, true),
            (Ship::Carrier, true),
        ]);
    }

    pub fn hit(&mut self, x: usize, y: usize) -> End {
        if x >= WIDTH || y >= HEIGHT {
            return End::Error; //OutOfBound
        }
        let slot = &mut self.bot[y][x];
        if match slot {
            Slot::NoneShot | Slot::Hit(_) => false,
            Slot::None => {
                *slot = Slot::NoneShot;
                true
            }
            Slot::Ship(ship) => {
                *slot = Slot::Hit(*ship);
                true
            }
        } {
            if all_hit(&self.bot) {
                return End::Win;
            }
            let mut rng = rand::thread_rng();
            loop {
                let x = rng.gen_range(0..WIDTH);
                let y = rng.gen_range(0..HEIGHT);
                let slot = &mut self.player[y][x];
                if match slot {
                    Slot::NoneShot | Slot::Hit(_) => false,
                    Slot::None => {
                        *slot = Slot::NoneShot;
                        true
                    }
                    Slot::Ship(ship) => {
                        *slot = Slot::Hit(*ship);
                        true
                    }
                } {
                    return if all_hit(&self.player) {
                        End::Lose
                    } else {
                        End::Continue
                    };
                }
            }
        } else {
            End::Error
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn play(&mut self) {
        let mut rng = rand::thread_rng();
        for ship in [
            Ship::PatrolBoat,
            Ship::Submarine,
            Ship::Destroyer,
            Ship::Battleship,
            Ship::Carrier,
        ] {
            let len = ship.len();
            loop {
                let dir = rng.gen_bool(0.5); // true => horizontal, false => vertical
                let x1 = rng.gen_range(0..(WIDTH - if dir { len - 1 } else { 0 }));
                let y1 = rng.gen_range(0..(HEIGHT - if dir { 0 } else { len - 1 }));
                let len = len as i8;
                let (x2, y2) = if dir {
                    ((x1 as i8 + rng.gen_range((-len + 1)..len)) as usize, y1)
                } else {
                    (x1, (y1 as i8 + rng.gen_range((-len + 1)..len)) as usize)
                };
                if Game::place_in(&mut self.bot, ship, x1, y1, x2, y2).is_some() {
                    break;
                }
            }
        }
        self.state = State::Playing;
    }

    pub const fn is_placing(&self) -> bool {
        matches!(self.state, State::Placing)
    }

    pub const fn is_playing(&self) -> bool {
        matches!(self.state, State::Playing)
    }

    pub fn print(&self) {
        println!("Bot");
        for row in &self.bot {
            println!(
                "{}",
                row.map(|slot| slot.char(false)).map(String::from).join("")
            );
        }
        println!();
        println!("Player");
        for row in &self.player {
            println!(
                "{}",
                row.map(|slot| slot.char(true)).map(String::from).join("")
            );
        }
    }
}

fn all_hit(board: &[[Slot; WIDTH]; HEIGHT]) -> bool {
    for row in board {
        for slot in row {
            if let Slot::Ship(_) = slot {
                return false;
            }
        }
    }
    true
}
