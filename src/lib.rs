mod hex_trait;
mod fractional_coord;
mod hex_coord;
mod layout;
mod shape;
mod map;
mod edge;

pub use crate::hex_coord::HexCoord;
pub use crate::hex_trait::HexCoordinate;
pub use crate::layout::HexLayout;
#[cfg(feature = "bevy")]
pub use crate::shape::*;
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
