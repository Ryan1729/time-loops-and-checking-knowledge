use platform_types::unscaled;

pub type TileKind = u16;

pub mod tile {
    use super::*;

    pub const WALL_0: TileKind = 0;
    pub const WALL_1: TileKind = 1;
    pub const WALL_2: TileKind = 3;
    pub const WALL_3: TileKind = 14;
    pub const WALL_4: TileKind = 17;
    pub const WALL_5: TileKind = 28;
    pub const WALL_6: TileKind = 39;
    pub const WALL_7: TileKind = 31;
    pub const WALL_8: TileKind = 42;
    pub const WALL_9: TileKind = 43;
    pub const WALL_10: TileKind = 44;
    pub const WALL_11: TileKind = 45;
    pub const WALL_12: TileKind = 56;
    pub const WALL_13: TileKind = 57;
    pub const WALL_14: TileKind = 58;
    pub const WALL_15: TileKind = 59;
    pub const WALL_16: TileKind = 70;
    pub const WALL_17: TileKind = 71;
    pub const WALL_18: TileKind = 72;
    pub const WALL_19: TileKind = 73;
    pub const WALL_20: TileKind = 126;
    pub const WALL_21: TileKind = 127;
    pub const WALL_22: TileKind = 129;
    pub const WALL_23: TileKind = 130;
    pub const WALL_24: TileKind = 132;
    pub const WALL_25: TileKind = 133;
    pub const WALL_26: TileKind = 135;
    pub const DOOR_0: TileKind = 2;
    pub const DOOR_1: TileKind = 30;
    pub const DOOR_2: TileKind = 33;
    pub const DOOR_3: TileKind = 128;
    pub const DOOR_4: TileKind = 134;
    pub const CLOSED_DOOR: TileKind = 32;
    pub const OPEN_DOOR: TileKind = DOOR_2;
    pub const PERSON_0: TileKind = 4;
    pub const PERSON_1: TileKind = 5;
    pub const PERSON_2: TileKind = 6;
    pub const PERSON_3: TileKind = 7;
    pub const PERSON_4: TileKind = 8;
    pub const PERSON_5: TileKind = 9;
    pub const ZOMBIE: TileKind = 11;
    pub const PANOPTIKHAN: TileKind = 13; // The floating eyeball head thing
    pub const FLOOR: TileKind = 15;
    pub const DOG: TileKind = 19;
    pub const CRAB: TileKind = 21;
    pub const GHOST_1: TileKind = 22;
    //pub const GHOST_2: TileKind = 23;
    pub const TURTLE: TileKind = 24;
    pub const LARGE_POT: TileKind = 36;
    //pub const SMALL_POT: TileKind = 37;
    pub const STAIRS_DOWN: TileKind = 46;
    pub const GROUND: TileKind = 60;
    pub const GRASS_GROUND: TileKind = 61;
    pub const KEY: TileKind = 80;
    pub const BUTTON_LIT: TileKind = 84;
    pub const BUTTON_DARK: TileKind = 85;
    pub const BUTTON_PRESSED: TileKind = 16;
    pub const EXCLAMATION_BUBBLE: TileKind = 102;
    pub const GRAVE_1: TileKind = 106;
    pub const GRAVE_2: TileKind = 107;
    pub const SPECIAL_GRAVE: TileKind = GRAVE_2; // TODO unique graphic?
    pub const PORTAL: TileKind = 113;
}

// TODO I think this is being used as both world xy and screen xy,
//      and we should make them distinct types if that becomes an issue
pub mod xy {
    use super::*;

    pub type Inner = u8;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct X(Inner);

    /// Clamps to the valid range
    pub const fn x(x: Inner) -> X {
        X(if x > MAX_W_INNER { MAX_W_INNER } else { x })
    }

    pub const MAX_W_INNER: Inner = 0xF0;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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


    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Y(Inner);

    pub const MAX_H_INNER: Inner = 0xF0;

    /// Clamps to the valid range
    pub const fn y(y: Inner) -> Y {
        Y(if y > MAX_H_INNER { MAX_H_INNER } else { y })
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn eight_neighbors(x: X, y: Y) -> [(X, Y); 8] {
        let mut output: [(X, Y); 8] = <_>::default();

        for i in 0..8 {
            output[i] = match i {
                1 => (x + W::ONE, y - H::ONE),
                2 => (x, y - H::ONE),
                3 => (x - W::ONE, y - H::ONE),
                4 => (x - W::ONE, y),
                5 => (x - W::ONE, y + H::ONE),
                6 => (x, y + H::ONE),
                7 => (x + W::ONE, y + H::ONE),
                _ => (x + W::ONE, y),
            };
        }

        output
    }

}
pub use xy::{X, Y, W, H, Rect};