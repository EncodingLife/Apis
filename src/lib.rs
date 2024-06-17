mod constants;
mod hex_trait;
mod fractional_coord;
mod hex_coord;
mod orientation;
mod shape;
mod edge;
mod world;
#[cfg(feature = "map")]
mod map;

pub use crate::hex_coord::HexCoord;
pub use crate::hex_trait::HexCoordinate;
pub use crate::orientation::HexOrientation;
pub use crate::edge::Edge;
pub use crate::world::{HexWorld, HexWorldShape};
#[cfg(feature = "bevy")]
pub use crate::shape::*;
// #[cfg(feature = "map")]
// pub use crate::map2::*;

pub use glam::{Vec2, Vec3};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
