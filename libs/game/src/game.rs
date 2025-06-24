use models::{tile, TileKind};
use platform_types::{Button, Input, Speaker, SFX, command, unscaled};
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

    pub const fn const_add_assign_w(x: &mut X, w: W) {
        x.0 = x.0.saturating_add(w.0);
        if x.0 > MAX_W_INNER {
            x.0 = MAX_W_INNER;
        }
    }

    impl core::ops::AddAssign<W> for X {
        fn add_assign(&mut self, w: W) {
            const_add_assign_w(self, w)
        }
    }

    pub const fn const_add_w(mut x: X, w: W) -> X {
        const_add_assign_w(&mut x, w);
        x
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

    pub const fn const_add_assign_h(y: &mut Y, h: H) {
        y.0 = y.0.saturating_add(h.0);
        if y.0 > MAX_H_INNER {
            y.0 = MAX_H_INNER;
        }
    }

    impl core::ops::AddAssign<H> for Y {
        fn add_assign(&mut self, h: H) {
            const_add_assign_h(self, h)
        }
    }

    pub const fn const_add_h(mut y: Y, h: H) -> Y {
        const_add_assign_h(&mut y, h);
        y
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

#[derive(Default)]
pub struct Entities {
    pub player: Entity,
    pub dynamic: HashMap<usize, Entity>,
    // Houses map specific
    pub turtle: Entity,
    pub crab: Entity,
    pub large_pot: Entity,
}

macro_rules! mobs {
    (& $self: ident) => ({
        [
            & $self.player,
            & $self.turtle,
            & $self.crab,
            & $self.large_pot,
        ]
    });
    (& mut $self: ident) => ({
        [
            &mut $self.player,
            &mut $self.turtle,
            &mut $self.crab,
            &mut $self.large_pot,
        ]
    });
}

impl Entities {
    fn mobs(&self) -> [&Entity; 4] {
        mobs!(&self)
    }

    fn mobs_mut(&mut self) -> [&mut Entity; 4] {
        mobs!(&mut self)
    }

    fn get_mut(&mut self, map: Map, x: X, y: Y) -> Option<&mut Entity> {
        for mob in mobs!(&mut self) {
            if x == mob.x
            && y == mob.y {
                return Some(mob);
            }
        }

        let index = xy_to_i(map, x, y);

        self.dynamic.get_mut(&index)
    }
}

type ButtonIndex = usize;
type ButtonCount = usize;

#[derive(Clone, Debug)]
pub struct PasswordLock<const N: ButtonCount = 4> {
    // TODO? not really a good reason to do SoA here huh? Switch to regular AoS?
    xs: [X; N],
    ys: [Y; N],
    names: [&'static str; N],
    open: [bool; N],
    press_count: ButtonCount,
}

impl <const N: ButtonCount> PasswordLock<N> {
    fn new(mut buttons: [(X, Y, &'static str); N], rng: &mut Xs) -> Self {
        xs::shuffle(rng, &mut buttons);

        let mut output = Self {
            xs: [<_>::default(); N],
            ys: [<_>::default(); N],
            names: ["unlabeled button"; N],
            open: [false; N],
            press_count: 0,
        };
        for i in 0..N {
            let (x, y, name) = buttons[i];
            output.xs[i] = x;
            output.ys[i] = y;
            output.names[i] = name;
        }
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
// * Make the people each reveal part of the password, but get angry if you asked someone else already (done)
// * Make in-game time eventually reset over some number of frames (done)
//     * Show current frame count first
// * Have a character move around, based on the frame time (done)
// * Have the moving character push something, (a large pot I guess?) in front of a door so a room is not reachable at
//   certain in-game times
// * And an NPC that has multiple dialogs that tells you which of a whole bunch of graves in a graveyard you can push
//   over to reveal stairs or a treasure
// * Make a new map with that graveyard in it, and a way to get there
//    * Walk to edge of the map, but make the edges that don't have anything say a message instead?
// * Allow moving that gravestone and put something under it
//    * Allow moving all the gravestones?

// Future ideas:
// * Ways to get rewards
//     * A chained trading/fetch quest
//        * This isn't that much of a knowledge check though?
//            * Final reward can be useful knowledge, akin to a password
//                * figure out the reward first
//            * Could be passing information back and forth between people
//     * A character that yammers on and on and eventually tells you about where something hidden is
//        * Could punt for now and just have you tell you after like two text boxes.
//        * One idea for this: An elderly person tells you that a boulder used to be somewhere else, so then that tips
//          you off that you need to move it. Maybe something is buried beneath it?
//     * A timed event that you can figure out by observing what happens
//         * Maybe some information, like a door combination, gets destroyed unless you intervene. Say a pie with a
//           bit of paper stuck to it gets given to someone else. So you need to get in line at the right time
// * Types of rewards
//    * Literal Password
//        * Like numbers or whatever
//        * Cpudl be an switch puzzle but where an alternate state that woudln't otherwise be a solution opens another door
//    * A hidden mechanic: Something works in a non-obvious way
//        * A mob moves in reaction to the player doing something specific
//            * staying exactly n spaces away
//                * following right behind as a subtype
//        * You can pick up a thing that is really close to a different thing and use it on something in a non-obvious way
//            * Pulling a kick-me sign out of the garbage maybe? That then causes a chain reaction when you place it?
//    * Figartive password
//        * knock on this specific unmarked tile, then this specific other one
//        * knowledge that an NPC will react a specific way to you doing something in their presence


type TileFlags = u8;

const NO_MOBS: TileFlags = 0x01;

fn get_effective_tile(map: Map, entities: &Entities, x: X, y: Y) -> Option<TileKind> {
    get_effective_tile_custom(map, entities, x, y, 0)
}

fn get_effective_tile_custom(map: Map, entities: &Entities, x: X, y: Y, flags: TileFlags) -> Option<TileKind> {
    for mob in entities.mobs() {
        if x == mob.x
        && y == mob.y {
            if flags & NO_MOBS == 0 {
                return Some(mob.kind);
            }
        }
    }

    let index = xy_to_i(map, x, y);

    entities.dynamic.get(&index)
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

fn xy_in_dir(dir: Dir, mut x: X, mut y: Y) -> (X, Y) {
    use Dir::*;
    match dir {
        Up => { y -= H::ONE; },
        Down => { y += H::ONE; },
        Left => { x -= W::ONE; },
        Right => { x += W::ONE; },
    }

    (x, y)
}

mod movement {
    use super::*;

    #[derive(Default)]
    pub struct Plan {
        old_x: X,
        old_y: Y,
        new_x: X,
        new_y: Y,
    }

    type PlannedLength = u8;

    #[derive(Default)]
    pub struct Planned {
        plans: [Plan; 15],
        length: PlannedLength,
    }

    impl Planned {
        fn push(&mut self, plan: Plan) {
            // TODO? report back if we ran out of room?
            if let Some(p) = self.plans.get_mut(self.length as usize) {
                *p = plan;
                self.length += 1;
            }
        }

        pub fn len(&self) -> PlannedLength {
            self.length
        }
    }

    pub fn plan(entity_x: X, entity_y: Y, map: Map, entities: &Entities, dir: Dir) -> Planned {
        let mut planned = Planned::default();

        plan_helper(&mut planned, entity_x, entity_y, map, entities, dir);

        planned
    }

    fn plan_helper(planned: &mut Planned, entity_x: X, entity_y: Y, map: Map, entities: &Entities, dir: Dir) {
        let (new_x, new_y) = xy_in_dir(dir, entity_x, entity_y);

        if new_x == entity_x && new_y == entity_y {
            // Don't ever recurse forever.
            *planned = Planned::default();
            return
        }

        let initial_length = planned.length;

        if let Some(tile_kind) = get_effective_tile(map, entities, new_x, new_y) {
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
                tile::LARGE_POT => {
                    // Attempt to push it
                    plan_helper(planned, new_x, new_y, map, entities, dir);

                    if initial_length == planned.length {
                        // If something can't move, cancel all the other moves
                        *planned = Planned::default();
                        return
                    }
                }
                _ => return, // Don't allow move to happen
            }
        } else {
            // If we're glitched out of bounds, let movement through so they can maybe get unstuck.
        }

        planned.push(Plan {
            old_x: entity_x,
            old_y: entity_y,
            new_x,
            new_y,
        })
    }

    pub fn perform(entities: &mut Entities, map: Map, Planned { plans, .. }: Planned) {
        let max_map_x = xy::x((map.width.saturating_sub(1)) as _);
        let max_map_y = xy::y((map.height.saturating_sub(1)) as _);

        for Plan { old_x, old_y, new_x, new_y, } in plans {
            if let Some(entity) = entities.get_mut(map, old_x, old_y) {
                entity.x = new_x.clamp(X::ZERO, max_map_x);
                entity.y = new_y.clamp(Y::ZERO, max_map_y);
            } else {
                // TODO? Send a signal back here that something went wrong?
            }
        }
    }
}

// TODO add a way to pull things, like pots. Does having them try to move you just work?
fn move_entity(entity_x: X, entity_y: Y, entities: &mut Entities, map: Map, dir: Dir) {
    movement::perform(entities, map, movement::plan(entity_x, entity_y, map, entities, dir));
}

#[derive(Default)]
pub enum MessageInfo {
    #[default]
    NoMessage,
    PasswordReveal {
        index: ButtonIndex,
    },
    PasswordRevealRefused,
}

/// 65536 distinct frames ought to be enough for anybody!
type FrameCount = u16;

pub struct State {
    pub frame_count: FrameCount,
    pub rng: Xs,
    pub map: Map,
    pub screen: Screen,
    pub entities: Entities,
    pub password_lock: PasswordLock,
    pub message_info: MessageInfo,
    pub previous_password_reveal_index: Option<usize>,
    pub hud_prints: [Print; 1],
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut rng = xs::from_seed(seed);

        let mut entities = Entities::default();

        entities.player = Entity {
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
                    (xy::x(200), xy::y(200), "???"),
                    (xy::x(200), xy::y(200), "???"),
                    (xy::x(200), xy::y(200), "???"),
                    (xy::x(200), xy::y(200), "???"),
                ]
            )
        } else {
            entities.turtle.x = xy::x(16);
            entities.turtle.y = xy::y(4);
            entities.turtle.kind = tile::TURTLE;

            entities.crab.x = xy::x(60);
            entities.crab.y = xy::y(4);
            entities.crab.kind = tile::CRAB;

            entities.large_pot.x = xy::x(59);
            entities.large_pot.y = xy::y(4);
            entities.large_pot.kind = tile::LARGE_POT;

            (
                &maps::HOUSES,
                [
                    (xy::x(6), xy::y(13), "north"),
                    (xy::x(7), xy::y(14), "east"),
                    (xy::x(6), xy::y(15), "south"),
                    (xy::x(5), xy::y(14), "west"),
                ],
            )
        };

        State {
            frame_count: 0,
            rng,
            map,
            screen: Screen::default(),
            entities,
            password_lock: PasswordLock::new(
                buttons,
                &mut rng
            ),
            message_info: MessageInfo::default(),
            previous_password_reveal_index: <_>::default(),
            hud_prints: <_>::default(),
        }
    }

    pub fn frame(&mut self, input: Input, speaker: &mut Speaker) {
        let mut sfx_opt = None;

        // Turtle movement
        if self.frame_count & 0b1111 == 0 {
            let planned = movement::plan(
                self.entities.turtle.x,
                self.entities.turtle.y,
                self.map,
                &self.entities,
                match self.frame_count >> 6 & 0b11 {
                    0b01 => Dir::Down,
                    0b10 => Dir::Right,
                    0b11 => Dir::Up,
                    _ => Dir::Left,
                }
            );

            if planned.len() > 0 {
                movement::perform(&mut self.entities, self.map, planned);

                match get_effective_tile_custom(self.map, &self.entities, self.entities.turtle.x, self.entities.turtle.y, NO_MOBS) {
                    Some(tile::BUTTON_LIT) => {
                        if let Some(sfx) = self.entity_on_button(self.entities.turtle.x, self.entities.turtle.y) {
                            speaker.request_sfx(sfx);
                        }
                    }
                    _ => {}
                }
            };
        }

        // Crab movement
        if self.frame_count & 0b111 == 0 {
            let planned = movement::plan(
                self.entities.crab.x,
                self.entities.crab.y,
                self.map,
                &self.entities,
                match self.frame_count >> 6 & 0b11 {
                    0b01 => Dir::Right,
                    _ => Dir::Left,
                }
            );
            if planned.len() > 0 {
                movement::perform(&mut self.entities, self.map, planned);

                match get_effective_tile_custom(self.map, &self.entities, self.entities.crab.x, self.entities.crab.y, NO_MOBS) {
                    Some(tile::BUTTON_LIT) => {
                        if let Some(sfx) = self.entity_on_button(self.entities.crab.x, self.entities.crab.y) {
                            speaker.request_sfx(sfx);
                        }
                    }
                    _ => {}
                }
            };
        }

        sfx_opt = if input.pressed_this_frame(Button::UP) {
            self.move_player(Dir::Up)
        } else if input.pressed_this_frame(Button::DOWN) {
            self.move_player(Dir::Down)
        } else if input.pressed_this_frame(Button::LEFT) {
            self.move_player(Dir::Left)
        } else if input.pressed_this_frame(Button::RIGHT) {
            self.move_player(Dir::Right)
        } else {
            None
        };

        if input.pressed_this_frame(Button::A) {
            if input.gamepad.contains(Button::UP) {
                self.interact(Dir::Up)
            } else if input.gamepad.contains(Button::DOWN) {
                self.interact(Dir::Down)
            } else if input.gamepad.contains(Button::LEFT) {
                self.interact(Dir::Left)
            } else if input.gamepad.contains(Button::RIGHT) {
                self.interact(Dir::Right)
            }
        }

        if let Some(sfx) = sfx_opt {
            speaker.request_sfx(sfx);
        }

        match self.frame_count.checked_add(1) {
            Some(count) => {
                self.frame_count = count;
            }
            None => {
                self.reset_time();
            }
        }

        use std::io::Write;
        let _ = write!(&mut self.hud_prints[0].text[..], "{}", self.frame_count);
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

        self.entities.dynamic.insert(index, entity);
    }

    fn remove_entity(&mut self, x: X, y: Y) -> Option<Entity> {
        let index = xy_to_i(self.map, x, y);

        self.entities.dynamic.remove(&index)
    }

    /// Returns the tile after any entities have replaced it, as opposed to the initial set of tiles.
    fn get_effective_tile(&mut self, x: X, y: Y) -> Option<TileKind> {
        get_effective_tile(self.map, &self.entities, x, y)
    }

    fn interact(&mut self, dir: Dir) {
        let (target_x, target_y) = xy_in_dir(dir, self.entities.player.x, self.entities.player.y);

        macro_rules! ask_for_password {
            ($index: expr) => {
                if self.previous_password_reveal_index == Some($index)
                || self.previous_password_reveal_index == None {
                    self.message_info = MessageInfo::PasswordReveal {
                        index: $index,
                    };
                    self.previous_password_reveal_index = Some($index);
                } else {
                    self.message_info = MessageInfo::PasswordRevealRefused;
                }
            }
        }

        match self.get_effective_tile(target_x, target_y) {
            Some(tile::PERSON_0) => {
                ask_for_password!(0)
            }
            Some(tile::PERSON_1) => {
                ask_for_password!(1)
            }
            Some(tile::PERSON_2) => {}
            Some(tile::PERSON_3) => {
                ask_for_password!(2)
            }
            Some(tile::PERSON_4) => {
                ask_for_password!(3)
            }
            None => {}
            _ => {}
        }
    }

    #[must_use]
    fn entity_on_button(&mut self, x: X, y: Y) -> Option<SFX> {
        let (key_x, key_y) = (xy::x(6), xy::y(14));

        let output = Some(SFX::ButtonPress);

        let button_count = self.password_lock.open.len() as ButtonCount;
        for i in 0..button_count {
            if self.password_lock.open[i] {
                continue
            }

            if self.password_lock.xs[i] == x
            && self.password_lock.ys[i] == y
            {
                if self.password_lock.press_count == i {
                    self.password_lock.open[i] = true;
                }
                self.password_lock.press_count += 1;

                self.add_entity(Entity {
                    kind: tile::BUTTON_DARK,
                    x,
                    y,
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

        output
    }

    #[must_use]
    fn move_player(&mut self, dir: Dir) -> Option<SFX> {
        let mut output = None;

        self.message_info = MessageInfo::NoMessage;

        match self.screen {
            Screen::Gameplay => {},
            Screen::Congraturation => return output,
        }

        move_entity(
            self.entities.player.x,
            self.entities.player.y,
            &mut self.entities,
            self.map,
            dir,
        );

        match self.map {
             m if core::ptr::eq(m, &maps::HOUSES)=> {
                let (locked_door_x, locked_door_y) = (xy::x(2), xy::y(3));

                match get_effective_tile_custom(self.map, &self.entities, self.entities.player.x, self.entities.player.y, NO_MOBS) {
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
                            x: self.entities.player.x,
                            y: self.entities.player.y,
                        });
                    }
                    Some(tile::BUTTON_LIT) => {
                        output = self.entity_on_button(self.entities.player.x, self.entities.player.y);
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
    pub start: usize,
    pub end: usize,
    pub x: X,
    pub y: Y,
}

impl Segment {
    const DEFAULT: Self = Self {
        text: b"",
        start: 0,
        end: 0,
        x: xy::x(0),
        y: xy::y(0),
    };
}

macro_rules! segment_literal {
    (
        text: $s: literal,
        x:  $x: expr,
        y:  $y: expr $(,)?
    ) => ({
        Segment {
            text: $s,
            start: 0,
            end: $s.len(),
            x: $x,
            y: $y,
        }
    })
}


static CONGRATURATION_LINES: [Segment; 2] =
    [
        segment_literal!(
            text: b"congraturation",
            x: xy::x(12),
            y: xy::y(4),
        ),
        segment_literal!(
            text: b"this story is happy end",
            x: xy::x(4),
            y: xy::y(8),
        ),
    ];

/// This group of constants is only for the HOUSES map.
// TODO make this depend on the map, to be less foot-gunny
const TEXT_BOX_TOP: Y = xy::y(24);
const TEXT_BOX_FIRST_COLUMN: X = xy::x(1);
const TEXT_BOX_FIRST_LINE: Y = xy::y(25);
const TEXT_BOX_USUABLE_WIDTH: usize = 30;


struct SegmentSlice {
    segments: [Segment; 16],
    length: usize,
}

impl SegmentSlice {
    fn as_slice(&self) -> &[Segment] {
        &self.segments[..self.length]
    }
}

static NORTH_0_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the north button first");
static NORTH_1_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the north button second");
static NORTH_2_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the north button third");
static NORTH_3_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the north button fourth");
static EAST_0_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the east button first");
static EAST_1_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the east button second");
static EAST_2_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the east button third");
static EAST_3_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the east button fourth");
static SOUTH_0_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the south button first");
static SOUTH_1_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the south button second");
static SOUTH_2_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the south button third");
static SOUTH_3_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the south button fourth");
static WEST_0_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the west button first");
static WEST_1_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the west button second");
static WEST_2_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the west button third");
static WEST_3_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"push the west button fourth");
static PASSWORD_REVEAL_REFUSAL_MESSAGE: SegmentSlice = fit_in_text_box(b"someone else told already. i won't.");
static MISSING_PASSWORD_REVEAL_MESSAGE: SegmentSlice = fit_in_text_box(b"missing_password_reveal_message");

const fn fit_in_text_box(s: &'static [u8]) -> SegmentSlice {
    let mut segments = [Segment::DEFAULT; 16];
    let mut length = 0;

    // This curently panics if we have too many segments!

    let mut line_start_index = 0;
    let mut y = TEXT_BOX_FIRST_LINE;
    // Iterating like this assumes we are dealing with ASCII, not Unicode!
    let mut i = 0;
    while i < s.len() {
        if s[i] == b' ' {
            let mut end_of_word = i + 1;
            while end_of_word < s.len() {
                if s[end_of_word] == b' ' {
                    break
                }
                end_of_word += 1;
            }
            let width = end_of_word - line_start_index;
            if width >= TEXT_BOX_USUABLE_WIDTH {
                let end = i.saturating_sub(1);
                segments[length] = Segment {
                    text: s,
                    start: line_start_index,
                    end,
                    x: TEXT_BOX_FIRST_COLUMN,
                    y,
                };
                length += 1;
                y = xy::const_add_h(y, H::ONE);
                line_start_index = end;
            }
        }

        i += 1;
    }
    segments[length] = Segment {
        text: s,
        start: line_start_index,
        end: s.len(),
        x: TEXT_BOX_FIRST_COLUMN,
        y,
    };
    length += 1;

    SegmentSlice {
        segments,
        length,
    }
}

pub struct RenderInfo<'state> {
    pub tiles: CurrentTiles<'state>,
    pub text_boxes: TextBoxes,
    pub message_segments: MessageSegments,
    pub hud: Hud<'state>,
}

#[derive(Default)]
pub struct Print {
    pub text: [u8; 16],
    pub x: unscaled::X,
    pub y: unscaled::Y,
}

pub struct Hud<'prints> {
    pub prints: &'prints [Print],
}

pub type TextBoxes = core::option::IntoIter<Rect>;
pub type MessageSegments = std::slice::Iter<'static, Segment>;

impl State {
    pub fn render_info(&self) -> RenderInfo<'_> {
        let map_w = xy::w(self.map.width as _);
        let map_h = xy::h(self.map.width as _);

        let output_width = xy::w(32).clamp(W::ZERO, map_w);
        let output_height = (TEXT_BOX_TOP - xy::y(0)).clamp(H::ZERO, map_h);

        let mut offset_x: W = self.entities.player.x - (X::ZERO + output_width.halve());
        let mut offset_y: H = self.entities.player.y - (Y::ZERO + output_height.halve());

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

        let message_segments: &'static [Segment] = match (self.screen, &self.message_info) {
            (Screen::Congraturation, _) => &CONGRATURATION_LINES,
            (Screen::Gameplay, &MessageInfo::NoMessage) => {&[]},
            (Screen::Gameplay, &MessageInfo::PasswordReveal { index, }) => {
                match (self.password_lock.names[index], index) {
                    ("north", 0) => NORTH_0_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("north", 1) => NORTH_1_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("north", 2) => NORTH_2_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("north", 3) => NORTH_3_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("west", 0) => WEST_0_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("west", 1) => WEST_1_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("west", 2) => WEST_2_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("west", 3) => WEST_3_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("south", 0) => SOUTH_0_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("south", 1) => SOUTH_1_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("south", 2) => SOUTH_2_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("south", 3) => SOUTH_3_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("east", 0) => EAST_0_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("east", 1) => EAST_1_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("east", 2) => EAST_2_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    ("east", 3) => EAST_3_PASSWORD_REVEAL_MESSAGE.as_slice(),
                    _ => MISSING_PASSWORD_REVEAL_MESSAGE.as_slice(),
                }
            },
            (Screen::Gameplay, &MessageInfo::PasswordRevealRefused) => {
                    PASSWORD_REVEAL_REFUSAL_MESSAGE.as_slice()
            },
        };

        let text_box = match self.screen {
            Screen::Gameplay => {
                match self.message_info {
                    MessageInfo::NoMessage => None,
                    _ => {
                        // TODO? Modify rect size based on the text
                        let min_y = xy::y(0) + output_height;
                        Some(Rect {
                            min_x: xy::x(0),
                            min_y,
                            max_x: xy::x(0) + output_width,
                            max_y: min_y + xy::h(7),
                        })
                    }
                }
            },
            Screen::Congraturation => {
                // No tiles needed
                camera.done = true;
                None
            },
        };

        let player = Some(Tile {
            kind: self.entities.player.kind,
            x: self.entities.player.x - offset_x,
            y: self.entities.player.y - offset_y,
        });

        RenderInfo {
            tiles: CurrentTiles {
                camera,
                player,
            },
            text_boxes: text_box.into_iter(),
            message_segments: message_segments.into_iter(),
            hud: Hud {
                prints: &self.hud_prints,
            }
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

        let x = self.tile.x + self.offset_x;
        let y = self.tile.y + self.offset_y;

        if let Some(tile_kind) = get_effective_tile(self.map, self.entities, x, y) {
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