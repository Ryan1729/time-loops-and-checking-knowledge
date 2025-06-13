use xs::Xs;

pub type TileKind = u16;

pub mod tile {
    use super::*;

    pub const DOOR_0: TileKind = 2;
    pub const DOOR_1: TileKind = 30;
    pub const DOOR_2: TileKind = 33;
    pub const DOOR_3: TileKind = 128;
    pub const DOOR_4: TileKind = 134;
    pub const FLOOR: TileKind = 15;
    pub const STAIRS_DOWN: TileKind = 46;
    pub const GROUND: TileKind = 60;
    pub const GRASS_GROUND: TileKind = 61;
    pub const KEY: TileKind = 80;
    pub const BUTTON_LIT: TileKind = 84;
    pub const BUTTON_DARK: TileKind = 85;
    pub const BUTTON_PRESSED: TileKind = 16;
    pub const PORTAL: TileKind = 113;
}

pub const RANK_COUNT: u8 = 13;
pub const SUIT_COUNT: u8 = 4;
pub const DECK_SIZE: u8 = RANK_COUNT * SUIT_COUNT;

pub type Card = u8;

pub fn gen_card(rng: &mut Xs) -> Card {
    xs::range(rng, 0..DECK_SIZE as _) as Card
}

pub type Suit = u8;

pub mod suits {
    use super::*;

    pub const CLUBS: Suit = 0;
    pub const DIAMONDS: Suit = 1;
    pub const HEARTS: Suit = 2;
    pub const SPADES: Suit = 3;
}

pub fn get_suit(card: Card) -> Suit {
    card / RANK_COUNT
}

pub type Rank = u8;

pub fn get_rank(card: Card) -> Rank {
    card % RANK_COUNT
}