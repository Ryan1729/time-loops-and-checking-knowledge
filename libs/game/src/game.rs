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

    impl core::ops::SubAssign<W> for X {
        fn sub_assign(&mut self, other: W) {
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

    impl core::ops::SubAssign<H> for Y {
        fn sub_assign(&mut self, other: H) {
            self.0 = self.0.saturating_sub(other.0);
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

    pub fn current_tiles(&self) -> impl Iterator<Item = Tile> {
        let top_left = 0; // TODO set based on player position

        CameraIter {
            tiles: &houses::TILES,
            tiles_width: houses::WIDTH as _,
            top_left, 
            output_width: xy::w(32),
            output_height: xy::h(24),
            tiles_index: top_left,
            tile: Tile::default(),
        }
    }
}

struct CameraIter<'tiles> {
    tile: Tile,
    tiles: &'tiles [TileKind],
    tiles_width: usize,
    tiles_index: usize,
    top_left: usize,
    output_width: xy::W,
    output_height: xy::H,
}

impl Iterator for CameraIter<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tile_kind) = self.tiles.get(self.tiles_index) {
            self.tile.kind = *tile_kind;

            let output = self.tile.clone();

            self.tiles_index += 1;
            self.tile.x += xy::w(1);

            // The x position in the entire field of tiles
            let tile_x = self.tiles_index % self.tiles_width;

            // The x position of the left edge of the view
            let left_x = self.top_left % self.tiles_width;
            let right_x = left_x + self.output_width.usize();

            if tile_x > right_x {
                self.tiles_index = 
                    // Move back to the start of the row
                    self.tiles_index - (self.output_width.usize() + 1)
                    // Move down a row
                    + self.tiles_width as usize;
                self.tile.x = xy::x(0);
                self.tile.y += xy::h(1);
            }

            // The y position in the entire field of tiles
            let tile_y = self.tiles_index / self.tiles_width;

            // The y position of the top edge of the view
            let top_y = self.top_left / self.tiles_width;
            let bottom_y = top_y + self.output_height.usize();

            if tile_y > bottom_y {
                // Ensure we hit return None next time
                self.tiles_index = self.tiles.len();
            }

            return Some(output)
        }

        None
    }
}