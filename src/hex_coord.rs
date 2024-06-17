use glam::{Vec2, Vec3};

use crate::{edge, hex_trait::HexCoordinate, Edge, HexOrientation};
use core::fmt::Debug;
use std::{fmt::Display, ops::{Add, Sub}};



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

    fn qrs(&self) -> (i32,i32,i32) {
        (self.0[0], self.0[1], self.0[2])
    }

    fn qrs_f32(&self) -> (f32,f32,f32) {
        (self.0[0] as f32, self.0[1] as f32, self.0[2] as f32)
    }

    fn neighbour(self, edge: Edge) -> Self {
        self + edge.offset_flat()
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