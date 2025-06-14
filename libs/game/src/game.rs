use models::{tile, TileKind};
use platform_types::{SFX, command, unscaled};
use xs::{Xs, Seed};

use std::collections::HashMap;

pub mod xy {
    use super::*;

    pub type Inner = u8;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct X(Inner);

    /// Clamps to the valid range
    pub const fn x(x: Inner) -> X {
        X(if x > MAX_W_INNER { MAX_W_INNER } else { x })
    }

    pub const MAX_W_INNER: Inner = 0xF0;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct W(Inner);

    pub const fn w(w: Inner) -> W {
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
    pub const fn y(y: Inner) -> Y {
        Y(if y > MAX_H_INNER { MAX_H_INNER } else { y })
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
    pub struct H(Inner);

    pub const fn h(h: Inner) -> H {
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

    pub struct Rect {
        pub min_x: X,
        pub min_y: Y,
        pub max_x: X,
        pub max_y: Y,
    }
}
pub use xy::{X, Y, W, H, Rect};

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

fn xy_to_i(map: Map, x: X, y: Y) -> usize {
    y.usize() * map.width as usize + x.usize()
}

#[derive(Clone, Copy, Default)]
pub enum Screen {
    #[default]
    Gameplay,
    Congraturation,
}

type Entities = HashMap<usize, Entity>;

type ButtonCount = usize;

#[derive(Clone, Debug)]
pub struct PasswordLock<const N: ButtonCount = 4> {
    // TODO? not really a good reason to do SoA here huh? Switch to regular AoS?
    xs: [X; N],
    ys: [Y; N],
    open: [bool; N],
    press_count: ButtonCount,
}

impl <const N: ButtonCount> PasswordLock<N> {
    fn new(mut buttons: [(X, Y); N], rng: &mut Xs) -> Self {
        xs::shuffle(rng, &mut buttons);

        let mut output = Self {
            xs: [<_>::default(); N],
            ys: [<_>::default(); N],
            open: [false; N],
            press_count: 0,
        };
        for i in 0..N {
            let (x, y) = buttons[i];
            output.xs[i] = x;
            output.ys[i] = y;
        }
        dbg!(&output);
        output
    }

    fn reset(&mut self) {
        for i in 0..N {
            self.open[i] = false;
        }
        self.press_count = 0;
    }
}

// Plan:
// Have a short 4 direction password you can type out on switches on the ground to win the game.
//
// Steps:
// * Make walking over a tile show a "congraturation this story is happy end" screen (done)
// * Make walking over the key open the door (done)
// * Make the key require pressing all the buttons down (done)
// * Make a fixed password reveal the key (done)
// * Make the password be randomized (done)
// * Make walking into the portal reset time (done)
// * Make the people each reveal part of the password, but get angry if you asked someone else already
// * Make in-game time eventually reset over some number of frames
//     * Show current frame count first
// * Have a character move around, based on the frame time
// * Have the moving character push something, (a large pot I guess?) in front of a door so a room is not reachable at
//   certain in-game times

fn get_effective_tile(map: Map, entities: &Entities, index: usize) -> Option<TileKind> {
    entities.get(&index)
        .map(|e| e.kind)
        .or_else(|| {
            map.tiles.get(index).copied()
        })
}

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_entity(entity: &mut Entity, map: Map, entities: &Entities, dir: Dir) {
    let mut new_x = entity.x;
    let mut new_y = entity.y;

    use Dir::*;
    match dir {
        Up => { new_y -= H::ONE; },
        Down => { new_y += H::ONE; },
        Left => { new_x -= W::ONE; },
        Right => { new_x += W::ONE; },
    }

    if let Some(tile_kind) = get_effective_tile(map, entities, xy_to_i(map, new_x, new_y)) {
        match tile_kind {
            tile::FLOOR
            | tile::GROUND
            | tile::GRASS_GROUND
            | tile::DOOR_0
            | tile::DOOR_1
            | tile::DOOR_2
            | tile::DOOR_3
            | tile::DOOR_4
            | tile::STAIRS_DOWN
            | tile::KEY
            | tile::BUTTON_LIT
            | tile::BUTTON_DARK
            | tile::BUTTON_PRESSED
            | tile::PORTAL => {} // Allow move to happen
            _ => return, // Don't allow move to happen
        }
    } else {
        // If we're glitched out of bounds, let movement through so they can maybe get unstuck.
    }

    let max_map_x = xy::x((map.width.saturating_sub(1)) as _);
    let max_map_y = xy::y((map.height.saturating_sub(1)) as _);

    entity.x = new_x.clamp(X::ZERO, max_map_x);
    entity.y = new_y.clamp(Y::ZERO, max_map_y);
}

pub struct State {
    pub rng: Xs,
    pub player: Entity,
    pub map: Map,
    pub screen: Screen,
    pub entities: Entities,
    pub password_lock: PasswordLock,
}


impl State {
    pub fn new(seed: Seed) -> State {
        let mut rng = xs::from_seed(seed);

        let player = Entity {
            kind: 9,
            x: xy::x(2),
            y: xy::y(5),
        };

        let (map, buttons) = if cfg!(feature = "structured_art_mode") {
            (
                &maps::STRUCTURED_ART,
                // Bogus values we don't expect to affect anything
                // TODO Adjust data model so we don't need these bogus values
                // (Heap allocate these buttons? Or just use an enum?)
                [
                    (xy::x(200), xy::y(200)),
                    (xy::x(200), xy::y(200)),
                    (xy::x(200), xy::y(200)),
                    (xy::x(200), xy::y(200)),
                ]
            )
        } else {
            (
                &maps::HOUSES,
                [
                    (xy::x(6), xy::y(13)),
                    (xy::x(7), xy::y(14)),
                    (xy::x(6), xy::y(15)),
                    (xy::x(5), xy::y(14)),
                ],
            )
        };

        State {
            rng,
            player,
            map,
            screen: Screen::default(),
            entities: Entities::default(),
            password_lock: PasswordLock::new(
                buttons,
                &mut rng
            ),
        }
    }

    fn reset_time(&mut self) {
        let mut password_lock = self.password_lock.clone();
        // Retain the combination for this game across resets.
        password_lock.reset();

        // New seed for the rng, so different resets are slightly different.
        *self = State::new(
            xs::new_seed(&mut self.rng)
        );

        self.password_lock = password_lock;
    }

    fn add_entity(&mut self, entity: Entity) {
        let index = xy_to_i(self.map, entity.x, entity.y);

        self.entities.insert(index, entity);
    }

    fn remove_entity(&mut self, x: X, y: Y) -> Option<Entity> {
        let index = xy_to_i(self.map, x, y);

        self.entities.remove(&index)
    }

    /// Returns the tile after any entities have replaced it, as opposed to the initial set of tiles.
    fn get_effective_tile(&mut self, x: X, y: Y) -> Option<TileKind> {
        get_effective_tile(self.map, &self.entities, xy_to_i(self.map, x, y))
    }

    #[must_use]
    pub fn move_player(&mut self, dir: Dir) -> Option<SFX> {
        let mut output = None;

        match self.screen {
            Screen::Gameplay => {},
            Screen::Congraturation => return output,
        }

        move_entity(&mut self.player, self.map, &self.entities, dir);

        match self.map {
             m if core::ptr::eq(m, &maps::HOUSES)=> {
                let (locked_door_x, locked_door_y) = (xy::x(2), xy::y(3));
                let (key_x, key_y) = (xy::x(6), xy::y(14));

                match self.get_effective_tile(self.player.x, self.player.y) {
                    Some(tile::PORTAL) => {
                        output = Some(SFX::CardPlace);

                        self.reset_time();
                    }
                    Some(tile::STAIRS_DOWN) => {
                        self.screen = Screen::Congraturation;
                    }
                    Some(tile::KEY) => {
                        output = Some(SFX::CardSlide);

                        self.add_entity(Entity {
                            kind: tile::DOOR_2,
                            x: locked_door_x,
                            y: locked_door_y,
                        });

                        self.add_entity(Entity {
                            kind: tile::FLOOR,
                            x: self.player.x,
                            y: self.player.y,
                        });
                    }
                    Some(tile::BUTTON_LIT) => {
                        output = Some(SFX::ButtonPress);

                        let button_count = self.password_lock.open.len() as ButtonCount;
                        for i in 0..button_count {
                            if self.password_lock.open[i] {
                                continue
                            }

                            if self.password_lock.xs[i] == self.player.x
                            && self.password_lock.ys[i] == self.player.y
                            {
                                if self.password_lock.press_count == i {
                                    self.password_lock.open[i] = true;
                                }
                                self.password_lock.press_count += 1;

                                self.add_entity(Entity {
                                    kind: tile::BUTTON_DARK,
                                    x: self.player.x,
                                    y: self.player.y,
                                });

                                break
                            }
                        }

                        // If lock is open
                        if self.password_lock.open.iter().all(|&b| b) {
                            self.add_entity(Entity {
                                kind: tile::KEY,
                                x: key_x,
                                y: key_y,
                            });
                        } else {
                            // If all the buttons were pressed without unlocking
                            if self.password_lock.press_count >= button_count {
                                // Reset all the buttons because a mistake was made
                                // entering it.
                                self.password_lock.reset();

                                for i in 0..button_count {
                                    self.remove_entity(
                                        self.password_lock.xs[i],
                                        self.password_lock.ys[i],
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                }
            },
            _ => {},
        }

        output
    }
}

pub struct Segment {
    pub text: &'static [u8],
    pub x: X,
    pub y: Y,
}

static CONGRATURATION_LINES: [Segment; 2] =
    [
        Segment {
            text: b"congraturation",
            x: xy::x(12),
            y: xy::y(4),
        },
        Segment {
            text: b"this story is happy end",
            x: xy::x(4),
            y: xy::y(8),
        },
    ];


pub struct RenderInfo<'tiles> {
    pub tiles: CurrentTiles<'tiles>,
    pub text_boxes: TextBoxes,
    pub message_segments: MessageSegments,
}

pub type TextBoxes = core::option::IntoIter<Rect>;
pub type MessageSegments = std::slice::Iter<'static, Segment>;

impl State {
    pub fn render_info(&self) -> RenderInfo<'_> {
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

        let mut camera = CameraIter {
            map: self.map,
            entities: &self.entities,
            offset_x,
            offset_y,
            output_width,
            output_height,
            done: false,
            tile: Tile::default(),
        };

        let text_box = match self.screen {
            Screen::Gameplay => {
                // TODO Show this conditionally, and in the right size based on the text
                Some(Rect {
                    min_x: xy::x(0),
                    min_y: xy::y(40),
                    max_x: xy::x(40),
                    max_y: xy::y(42),
                })
            },
            Screen::Congraturation => {
                // No tiles needed
                camera.done = true;
                None
            },
        };

        let player = Some(Tile {
            kind: self.player.kind,
            x: self.player.x - offset_x,
            y: self.player.y - offset_y,
        });

        let message_segments: &'static [Segment] = match self.screen {
            Screen::Gameplay => &[],
            Screen::Congraturation => &CONGRATURATION_LINES,
        };

        RenderInfo {
            tiles: CurrentTiles {
                camera,
                player,
            },
            text_boxes: text_box.into_iter(),
            message_segments: message_segments.into_iter(),
        }
    }
}

pub struct CurrentTiles<'camera> {
    camera: CameraIter<'camera>,
    player: Option<Tile>,
}

impl Iterator for CurrentTiles<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.camera.next() {
            return Some(t)
        }

        if let Some(t) = self.player.take() {
            return Some(t)
        }

        None
    }
}

struct CameraIter<'entities> {
    tile: Tile,
    done: bool,
    map: Map,
    entities: &'entities Entities,
    offset_x: xy::W,
    offset_y: xy::H,
    output_width: xy::W,
    output_height: xy::H,
}

impl Iterator for CameraIter<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        let tiles_index =
            xy_to_i(self.map, self.tile.x + self.offset_x, self.tile.y + self.offset_y);
        if let Some(tile_kind) = get_effective_tile(self.map, self.entities, tiles_index) {
            self.tile.kind = tile_kind;

            let output = self.tile.clone();

            self.tile.x += xy::w(1);

            let right_x = X::ZERO + self.output_width;

            if self.tile.x >= right_x {
                self.tile.x = X::ZERO;
                self.tile.y += xy::h(1);

                let bottom_y = Y::ZERO + self.output_height;

                if self.tile.y >= bottom_y {
                    // Ensure we hit return None next time
                    self.done = true;
                }
            }

            return Some(output)
        }

        None
    }
}