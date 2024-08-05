mod constants;
mod edge;
mod flat_map;
mod hex_coord;
mod hex_trait;
mod index;
mod orientation;
mod shape;
mod world;

pub use crate::edge::Edge;
pub use crate::flat_map::*;
pub use crate::hex_coord::HexCoord;
pub use crate::hex_trait::HexCoordinate;
pub use crate::index::map_index::MapIndex;
pub use crate::orientation::HexOrientation;
#[cfg(feature = "bevy")]
pub use crate::shape::*;
pub use crate::world::{HexWorld, HexWorldShape};

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
