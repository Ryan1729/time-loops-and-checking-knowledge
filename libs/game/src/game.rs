use models::{TileKind};
use maps::houses;
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

    #[derive(Clone, Copy, Default)]
    pub struct W(Inner);

    impl W {
        pub const ONE: Self = Self(1);

        pub fn usize(self) -> usize {
            self.0.into()
        }
    }

    pub fn w(w: Inner) -> W {
        W(if w > MAX_W_INNER { MAX_W_INNER } else { w })
    }

    impl core::ops::AddAssign<W> for X {
        fn add_assign(&mut self, other: W) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_W_INNER {
                self.0 = MAX_W_INNER;
            }
        }
    }

    impl core::ops::Add<W> for X {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign<W> for X {
        fn sub_assign(&mut self, other: W) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    impl core::ops::Sub<W> for X {
        type Output = Self;

        fn sub(mut self, other: W) -> Self::Output {
            self -= other;
            self
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

    #[derive(Clone, Copy, Default)]
    pub struct H(Inner);

    impl H {
        pub const ONE: Self = Self(1);

        pub fn usize(self) -> usize {
            self.0.into()
        }
    }

    pub fn h(h: Inner) -> H {
        H(if h > MAX_H_INNER { MAX_H_INNER } else { h })
    }

    impl core::ops::AddAssign<H> for Y {
        fn add_assign(&mut self, other: H) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_H_INNER {
                self.0 = MAX_H_INNER;
            }
        }
    }

    impl core::ops::Add<H> for Y {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign<H> for Y {
        fn sub_assign(&mut self, other: H) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    impl core::ops::Sub<H> for Y {
        type Output = Self;

        fn sub(mut self, other: H) -> Self::Output {
            self -= other;
            self
        }
    }
}
pub use xy::{X, Y, W, H};

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
    pub scratch_tile: Tile,
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
        Up => { entity.y -= H::ONE; },
        Down => { entity.y += H::ONE; },
        Left => { entity.x -= W::ONE; },
        Right => { entity.x += W::ONE; },
    }
}

impl State {
    pub fn new(seed: Seed) -> State {
        use maps::{houses};

        let rng = xs::from_seed(seed);

        let mut player = Entity::default();
        player.kind = 9;
        player.x = xy::x(2);

        State {
            rng,
            player,
            .. <_>::default()
        }
    }

    pub fn move_player(&mut self, dir: Dir) {
        move_entity(&mut self.player, dir);
    }

    pub fn current_tiles(&self) -> (impl Iterator<Item = Tile>, [Tile; 1]) {
        let output_width = xy::w(32);
        let output_height = xy::h(24);

        let tiles_width: usize = houses::WIDTH as _;
        let tiles_height: usize = houses::HEIGHT as _;

        let mut offset_x = self.player.x.get().get() as isize - output_width.usize() as isize / 2;
        let mut offset_y = self.player.y.get().get() as isize - output_height.usize() as isize / 2;

        offset_x = offset_x.clamp(0, output_width.usize() as isize);
        offset_y = offset_y.clamp(0, output_height.usize() as isize);

        // TODO avoid unneeded conversions

        let sprites = [
            Tile {
                kind: self.player.kind,
                x: self.player.x - xy::w(offset_x.try_into().unwrap()), // TODO avoid this unwrap
                y: self.player.y - xy::h(offset_y.try_into().unwrap()),
            },
        ];

        (
            CameraIter {
                tiles: &houses::TILES,
                tiles_width,
                offset_x: offset_x as usize,
                offset_y: offset_y as usize,
                output_width,
                output_height,
                done: false,
                tile: Tile::default(),
            },
            sprites
        )
    }
}

struct CameraIter<'tiles> {
    tile: Tile,
    tiles: &'tiles [TileKind],
    tiles_width: usize,
    done: bool,
    offset_x: usize,
    offset_y: usize,
    output_width: xy::W,
    output_height: xy::H,
}

impl Iterator for CameraIter<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        let tiles_index =
            (self.tile.y.get().get() as usize + self.offset_y) * self.tiles_width
            + (self.tile.x.get().get() as usize + self.offset_x);
        if let Some(tile_kind) = self.tiles.get(tiles_index) {
            self.tile.kind = *tile_kind;

            let output = self.tile.clone();

            self.tile.x += xy::w(1);

            let right_x = self.output_width.usize();

            if self.tile.x.get().get() as usize > right_x {
                self.tile.x = xy::x(0);
                self.tile.y += xy::h(1);

                let bottom_y = self.output_height.usize();

                if self.tile.y.get().get() as usize > bottom_y {
                    // Ensure we hit return None next time
                    self.done = true;
                }
            }

            return Some(output)
        }

        None
    }
}