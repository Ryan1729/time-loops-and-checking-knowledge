use models::{TileKind};
use platform_types::{command, unscaled};
use xs::{Xs, Seed};

pub mod xy {
    use super::*;

    pub type Inner = u8;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct X(Inner);

    /// Clamps to the valid range
    pub fn x(x: Inner) -> X {
        X(if x > MAX_W_INNER { MAX_W_INNER } else { x })
    }

    pub const MAX_W_INNER: Inner = 0xF0;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct W(Inner);

    pub fn w(w: Inner) -> W {
        W(if w > MAX_W_INNER { MAX_W_INNER } else { w })
    }

    impl core::ops::SubAssign<W> for W {
        fn sub_assign(&mut self, other: W) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    impl core::ops::Sub<W> for W {
        type Output = Self;

        fn sub(mut self, other: W) -> Self::Output {
            self -= other;
            self
        }
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

    impl core::ops::Sub<X> for X {
        type Output = W;

        fn sub(self, other: X) -> Self::Output {
            W(self.0.saturating_sub(other.0))
        }
    }


    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Y(Inner);

    pub const MAX_H_INNER: Inner = 0xF0;

    /// Clamps to the valid range
    pub fn y(y: Inner) -> Y {
        Y(if y > MAX_H_INNER { MAX_H_INNER } else { y })
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct H(Inner);

    pub fn h(h: Inner) -> H {
        H(if h > MAX_H_INNER { MAX_H_INNER } else { h })
    }

    impl core::ops::SubAssign<H> for H {
        fn sub_assign(&mut self, other: H) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    impl core::ops::Sub<H> for H {
        type Output = Self;

        fn sub(mut self, other: H) -> Self::Output {
            self -= other;
            self
        }
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

    impl core::ops::Sub<Y> for Y {
        type Output = H;

        fn sub(self, other: Y) -> Self::Output {
            H(self.0.saturating_sub(other.0))
        }
    }

    macro_rules! shared_impl {
        ($($name: ident)+) => {
            $(
                impl $name {
                    pub const ZERO: Self = Self(0);
                    pub const ONE: Self = Self(1);

                    pub fn get(self) -> unscaled::$name {
                        unscaled::$name(self.0.into())
                    }

                    pub fn usize(self) -> usize {
                        self.0.into()
                    }

                    pub fn halve(self) -> Self {
                        Self(self.0 >> 1)
                    }
                }
            )+
        }
    }

    shared_impl!{
        X Y W H
    }
}
pub use xy::{X, Y, W, H};

#[derive(Clone, Default)]
pub struct Entity {
    pub kind: TileKind,
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Debug, Default)]
pub struct Tile {
    pub kind: TileKind,
    pub x: X,
    pub y: Y,
}

type Map = &'static maps::Map;

#[derive(Clone)]
pub struct State {
    pub rng: Xs,
    pub player: Entity,
    pub map: Map
}

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_entity(entity: &mut Entity, map: Map, dir: Dir) {
    use Dir::*;
    match dir {
        Up => { entity.y -= H::ONE; },
        Down => { entity.y += H::ONE; },
        Left => { entity.x -= W::ONE; },
        Right => { entity.x += W::ONE; },
    }

    let max_map_x = xy::x((map.width.saturating_sub(1)) as _);
    let max_map_y = xy::y((map.height.saturating_sub(1)) as _);

    entity.x = entity.x.clamp(X::ZERO, max_map_x);
    entity.y = entity.y.clamp(Y::ZERO, max_map_y);
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut rng = xs::from_seed(seed);

        let mut player = Entity::default();
        player.kind = 9;
        player.x = xy::x(2);

        let map = match xs::range(&mut rng, 0..2 as u32) {
            1 => &maps::HOUSES,
            _ => &maps::STRUCTURED_ART,
        };

        State {
            rng,
            player,
            map,
        }
    }

    pub fn move_player(&mut self, dir: Dir) {
        move_entity(&mut self.player, self.map, dir);
    }

    pub fn current_tiles(&self) -> (impl Iterator<Item = Tile>, [Tile; 1]) {
        let map_w = xy::w(self.map.width as _);
        let map_h = xy::h(self.map.width as _);

        let output_width = xy::w(32).clamp(W::ZERO, map_w);
        let output_height = xy::h(24).clamp(H::ZERO, map_h);

        let mut offset_x: W = self.player.x - (X::ZERO + output_width.halve());
        let mut offset_y: H = self.player.y - (Y::ZERO + output_height.halve());

        // Want to clamp the offset such that we never see the edge of the world.
        // So when output_width == self.map.width, we want the offset to always
        // be zero. But, when output_width + 1 == self.map.width we want the
        // offset to sometimes be one. Hence self.map.width - output_width

        offset_x = offset_x.clamp(W::ZERO, map_w - output_width);
        offset_y = offset_y.clamp(H::ZERO, map_h - output_height);

        let sprites = [
            Tile {
                kind: self.player.kind,
                x: self.player.x - offset_x,
                y: self.player.y - offset_y,
            },
        ];

        (
            CameraIter {
                map: self.map,
                offset_x,
                offset_y,
                output_width,
                output_height,
                done: false,
                tile: Tile::default(),
            },
            sprites
        )
    }
}

struct CameraIter {
    tile: Tile,
    done: bool,
    map: Map,
    offset_x: xy::W,
    offset_y: xy::H,
    output_width: xy::W,
    output_height: xy::H,
}

impl Iterator for CameraIter {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        let tiles_index =
            (self.tile.y + self.offset_y).usize() * self.map.width as usize
            + (self.tile.x + self.offset_x).usize();
        if let Some(tile_kind) = self.map.tiles.get(tiles_index) {
            self.tile.kind = *tile_kind;

            let output = self.tile.clone();

            self.tile.x += xy::w(1);

            let right_x = X::ZERO + self.output_width;

            if self.tile.x + self.offset_x >= right_x {
                self.tile.x = X::ZERO;
                self.tile.y += xy::h(1);

                let bottom_y = Y::ZERO + self.output_height;

                if self.tile.y + self.offset_y >= bottom_y {
                    // Ensure we hit return None next time
                    self.done = true;
                }
            }

            return Some(output)
        }

        None
    }
}