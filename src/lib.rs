mod edge;
mod static_map;
mod hex_coord;
mod hex_trait;
mod index;
mod orientation;
mod world;

#[cfg(feature = "bevy")]
pub mod bevy;

pub use crate::edge::Edge;
pub use crate::static_map::*;
pub use crate::hex_coord::HexCoord;
pub use crate::hex_trait::HexCoordinate;
pub use crate::index::map_index::MapIndex;
pub use crate::orientation::HexOrientation;
pub use crate::world::{HexWorld, HexWorldShape};

#[cfg(feature = "bevy")]
pub use crate::bevy::*;
#[cfg(not(feature = "bevy"))]
pub use glam::{Vec2, Vec3};

pub mod prelude {
    pub use crate::{
        edge::Edge,
        hex_coord::HexCoord,
        hex_trait::HexCoordinate,
        index::map_index::MapIndex,
        orientation::HexOrientation,
        world::{HexWorld, HexWorldShape}
    };
}
