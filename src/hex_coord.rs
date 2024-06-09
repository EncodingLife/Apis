use glam::{Vec2, Vec3};

use crate::{hex_trait::HexCoordinate, HexLayout};
use core::fmt::Debug;
use std::{fmt::Display, ops::{Add, Sub}};

static F0: f32 = 3.0/2.0;
const F1: f32 = 0.0;
const F2: f32 = 0.86602540378; // f32::sqrt(3.0) / 2.0;
const F3: f32 = 1.73205080757; // f32::sqrt(3.0);


#[derive(Debug, Eq, Clone, Copy, Hash)]
pub struct HexCoord([i32;3]);

impl HexCoord {
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        assert_eq!(q+r+s, 0, "QRS must add up to 0");
        Self([q,r,s])
    }
}

impl HexCoordinate<i32> for HexCoord {
    fn dist(&self, rhs: Self) -> i32 {
        let d = *self - rhs;
        (d.0[0].abs() + d.0[1].abs() + d.0[2].abs()) / 2
    }

    fn to_world(&self, layout: HexLayout) -> Vec2 {
        let q = self.0[0] as f32;
        let r = self.0[1] as f32;

        let x = (F0 * q + F1 * r) * layout.size.x;
        let y = (F2 * q + F3 * r) * layout.size.y;
        Vec2::new(x,y)
    }

    fn to_world_v3(&self, layout: HexLayout) -> Vec3 {
        let v2 = self.to_world(layout);
        Vec3::new(v2.x, v2.y, 0.0)
    }

    fn qrs(&self) -> (i32,i32,i32) {
        (self.0[0], self.0[1], self.0[2])
    }
}


impl Display for HexCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0[0], self.0[1], self.0[2])
    }
}

impl PartialEq for HexCoord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add for HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2] + rhs.0[2]])
    }
}

impl Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2] - rhs.0[2]])
    }
}

#[macro_export]
macro_rules! hex {
    ($q:literal, $r:literal) => {
        HexCoord::new($q,$r, -$q-$r)
    };
    ($q:ident, $r:ident) => {
        HexCoord::new($q,$r, -$q-$r)
    };
    ($q:literal, $r:literal, $s:literal) => {
        HexCoord::new($q,$r, $s)
    };
    ($q:ident, $r:ident, $s:ident) => {
        HexCoord::new($q,$r, $s)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_macro() {
        assert_eq!(hex!(1,-1, 0), hex!(1,-1,0));
        assert_eq!(hex!(1,-1), hex!(1,-1,0));
        let r = 5;
        let s = 8;
        let q = -r-s;
        assert_eq!(hex!(r,s), hex!(r, s, q));
    }

    #[test]
    fn equality_are_equal() {
        assert_eq!(hex!(1,1,-2), hex!(1,1,-2));
    }

    #[test]
    fn add() {
        assert_eq!(hex!(1,1,-2) + hex!(1,0,-1), hex!(2,1,-3));
    }

    #[test]
    fn sub() {
        assert_eq!(hex!(1,-1,0) - hex!(1,2,-3), hex!(0,-3, 3));
    }

    #[test]
    fn dist() {
        assert_eq!(hex!(2,1,-3).dist(hex!(1,2,-3)), 1);
        assert_eq!(hex!(0,-4,4).dist(hex!(0,4,-4)), 8);
        assert_eq!(hex!(3,-1,-2).dist(hex!(1,0,-1)), 2);
    }
}