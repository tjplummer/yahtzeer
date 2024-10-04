use rand::Rng;
use rand::rngs::ThreadRng;
use struct_iterable::Iterable;
use argh::FromArgs;
use rand::distr::{Distribution, Standard};

#[derive(PartialEq, Clone, Copy)]
enum Focus {
    Top,
    Bottom
}

impl Distribution<Focus> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Focus {
        match rng.gen_range(0..=1) {
            0 => Focus::Bottom,
            _ => Focus::Top
        }
    }
}

struct AI {
    focus: Focus,
}

impl AI {
}

struct Players {
    players: Vec<Player>
}

impl Players {
    fn new(players_playing: u8) -> Players {
        let mut players: Vec<Player> = Vec::new();

        for x in 0..players_playing {
            players.push(Player::new(x, Card::new())
            )
        }

        Players {
            players
        }
    }
}

struct Player {
    id: u8,
    card: Card,
    complete: bool
}

impl Player {
    fn new(id: u8, card: Card) -> Player {
        Player {
            id,
            card,
            complete: false
        }
    }
}

#[derive(Iterable)]
struct Top {
    one: i8,
    two: i8,
    three: i8,
    four: i8,
    five: i8,
    six: i8
}

impl Default for Top {
    fn default() -> Self {
        Top {
            one: -1,
            two: -1,
            three: -1,
            four: -1,
            five: -1,
            six: -1,
        }
    }
}

impl Top {
    fn one(&mut self, total: i8) {
        self.one = total
    }

    fn two(&mut self, total: i8) {
        self.two = total
    }

    fn three(&mut self, total: i8) {
        self.three = total
    }

    fn four(&mut self, total: i8) {
        self.four = total
    }

    fn five(&mut self, total: i8) {
        self.five = total
    }

    fn six(&mut self, total: i8) {
        self.six = total
    }

    fn total_up(&self) -> i8 {
        let mut total = 0;

        for (_param, val) in &self.iter() {
            total += val;
        }

        if total >= 63 { total += 35 }

        total
    }
}

#[derive(Iterable)]
struct Bottom {
    three_of_kind: i8,
    four_of_kind: i8,
    full_house: i8,
    small_straight: i8,
    large_straight: i8,
    yahtzee: i8,
    bonus_yahtzee: i8,
    chance: i8
}

impl Default for Bottom {
    fn default() -> Bottom {
        Bottom {
            three_of_kind: -1,
            four_of_kind: -1,
            full_house: -1,
            small_straight: -1,
            large_straight: -1,
            yahtzee: -1,
            bonus_yahtzee: -1,
            chance: -1
        }
    }
}

impl Bottom {
    fn three_of_kind(&mut self, succeed: bool, total: i8) {
        if succeed { self.three_of_kind = total } else { self.three_of_kind = 0 }
    }

    fn four_of_kind(&mut self, succeed: bool, total: i8) {
        if succeed { self.four_of_kind = total } else { self.four_of_kind = 0 }
    }

    fn full_house(&mut self, succeed: bool) {
        if succeed { self.full_house = 25 } else { self.full_house = 0 }
    }

    fn small_straight(&mut self, succeed: bool) {
        if succeed { self.small_straight = 30 } else { self.small_straight = 0 }
    }

    fn large_straight(&mut self, succeed: bool) {
        if succeed { self.large_straight = 40 } else { self.large_straight = 0 }
    }

    fn yahtzee(&mut self, succeed: bool) {
        if succeed { self.yahtzee = 50 } else { self.yahtzee = 0 }
    }

    fn bonus_yahtzee(&mut self, succeed: bool) {
        if succeed {
            if self.bonus_yahtzee == -1 { self.bonus_yahtzee = 100 } else { self.bonus_yahtzee += 100 }
        } else {
            self.bonus_yahtzee = 0
        }
    }

    fn chance(&mut self, succeed: bool, total: i8) {
        if succeed { self.chance = total } else { self.chance = 0 }
    }
}

struct GameState {
    turn: u8,
    players: Players,
}

impl GameState {
    fn new(players_playing: u8) -> GameState {
        GameState {
            turn: 0,
            players: Players::new(players_playing)
        }
    }
}

#[derive(Iterable)]
struct Card {
    top: Top,
    bottom: Bottom
}

impl Card {
    fn new() -> Card {
        Card {
            top: Top::default(),
            bottom: Bottom::default()
        }
    }

    fn check_if_complete(&self) -> bool {
        let mut tf = false;
        let mut bf = false;

        for (param, val) in &self.iter() {
            for (_inner_param, inner_val) in val.iter() {
                if (*inner_val == -1) {
                    if param == "top" { tf = true } else { bf = false }
                }
            }
        }

        if tf & bf { true } else { false }
    }
}

fn roll_dice(mut rng: ThreadRng) -> u32 {
    rng.gen_range(1..6)
}

fn default_players() -> u8 { 4 }

fn default_quantity() -> u8 { 4 }

#[derive(FromArgs)]
/// Exercise in Yahtzee
struct Args {
    /// an optional parameter for players. Default is 4
    #[argh(option, short = 'p', default = "default_players()")]
    players: u8,

    /// an optional parameter for games to play. Default is 1
    #[argh(option, short = 'q', default = "default_quantity()")]
    quantity: u32
}

fn main() {
    println!("Hello, world!");
}
