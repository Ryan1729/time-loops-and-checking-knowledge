use models::{TileKind};
use platform_types::{command, unscaled};
use xs::{Xs, Seed};

pub mod xy {
    use super::*;

    pub type Inner = u8;

    #[derive(Clone, Copy, Default)]
    pub struct X(Inner);

    /// Clamps to the valid range
    pub fn x(x: Inner) -> X {
        X(if x > MAX_W_INNER { MAX_W_INNER } else { x })
    }

    pub const MAX_W_INNER: Inner = 0xF0;

    impl X {
        pub const ONE: Self = Self(1);

        pub fn get(self) -> unscaled::X {
            unscaled::X(self.0.into())
        }
    }

    impl core::ops::AddAssign for X {
        fn add_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_W_INNER {
                self.0 = MAX_W_INNER;
            }
        }
    }

    impl core::ops::SubAssign for X {
        fn sub_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct Y(Inner);

    pub const MAX_H_INNER: Inner = 0xF0;

    /// Clamps to the valid range
    pub fn y(y: Inner) -> Y {
        Y(if y > MAX_H_INNER { MAX_H_INNER } else { y })
    }

    impl Y {
        pub const ONE: Self = Self(1);

        pub fn get(self) -> unscaled::Y {
            unscaled::Y(self.0.into())
        }
    }

    impl core::ops::AddAssign for Y {
        fn add_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_H_INNER {
                self.0 = MAX_H_INNER;
            }
        }
    }

    impl core::ops::SubAssign for Y {
        fn sub_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }
}
pub use xy::{X, Y};

#[derive(Clone, Default)]
pub struct Entity {
    pub kind: TileKind,
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Default)]
pub struct Tile {
    pub kind: TileKind,
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Default)]
pub struct State {
    pub rng: Xs,
    pub player: Entity,
    pub tiles: Vec<Tile>,
}

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_entity(entity: &mut Entity, dir: Dir) {
    use Dir::*;
    match dir {
        Up => { entity.y -= Y::ONE; },
        Down => { entity.y += Y::ONE; },
        Left => { entity.x -= X::ONE; },
        Right => { entity.x += X::ONE; },
    }
}

impl State {
    pub fn new(seed: Seed) -> State {
        let rng = xs::from_seed(seed);

        let mut player = Entity::default();
        player.kind = 9;
        player.x = xy::x(2);

        // TODO define a whole fixed map
        //    Maybe define macros or something to make it nicer?
        //    Actually probably take an array of kinds or something, then add the coords.
        //        Or maybe make it a dense map of tiles?
        let mut tiles = vec![
            Tile {
                kind: 0,
                x: xy::x(0),
                y: xy::y(0),
            },
            Tile {
                kind: 1,
                x: xy::x(1),
                y: xy::y(0),
            },
            Tile {
                kind: 2,
                x: xy::x(2),
                y: xy::y(0),
            },
            Tile {
                kind: 3,
                x: xy::x(3),
                y: xy::y(0),
            },
            Tile {
                kind: 14,
                x: xy::x(0),
                y: xy::y(1),
            },
            Tile {
                kind: 15,
                x: xy::x(1),
                y: xy::y(1),
            },
            Tile {
                kind: 15, // On purpose!
                x: xy::x(2),
                y: xy::y(1),
            },
            Tile {
                kind: 17,
                x: xy::x(3),
                y: xy::y(1),
            },
        ];

        State {
            rng,
            tiles,
            player,
            .. <_>::default()
        }
    }

    pub fn move_player(&mut self, dir: Dir) {
        move_entity(&mut self.player, dir);
    }

    pub fn current_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }
}