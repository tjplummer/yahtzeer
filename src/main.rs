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

struct Player {
    id: u8,
    card: Card,
}

impl Player {
    fn new(id: u8, card: Card) -> Player {
        Player {
            id,
            card
        }
    }
}

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
}

fn roll_dice(mut rng: ThreadRng) -> u32 {
    rng.gen_range(1..6)
}

#[derive(FromArgs)]
/// Exercise in Trouble
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
