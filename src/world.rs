use crate::{constants::*, HexCoord, HexCoordinate, HexOrientation};

#[derive(Copy, Clone)]
pub enum HexWorldShape {
    // Hexagon with it's radius in cells incl origin
    Hexagon(usize),
    // Rectangle with it's width and height in cells
    Rectangle(usize, usize),
    // Rectangle with equal width and height
    Square(usize),
}
#[cfg(not(feature = "bevy"))]
type Vec2 = glam::Vec2;
#[cfg(feature = "bevy")]
type Vec2 = bevy::math::Vec2;
#[cfg(not(feature = "bevy"))]
type Vec3 = glam::Vec3;
#[cfg(feature = "bevy")]
type Vec3 = bevy::math::Vec3;

#[derive(Copy, Clone)]
pub struct HexWorld<U>
where
    U: Copy,
{
    pub orientation: HexOrientation,
    // #[cfg(not(feature="bevy"))]
    // pub cell_size: glam::Vec2,
    // #[cfg(feature = "bevy")]
    // pub cell_size: bevy::math::Vec2,
    pub cell_size: U,
    pub world_shape: HexWorldShape,
}

impl<U> HexWorld<U>
where
    U: Copy,
{
    pub fn new(orientation: HexOrientation, cell_size: U, world_shape: HexWorldShape) -> Self {
        Self {
            orientation,
            cell_size,
            world_shape,
        }
    }
}

impl<U> HexWorld<U>
where
    U: Copy,
    f32: std::convert::From<U>,
    U: std::convert::From<f32>,
{
    pub fn coord_to_world(&self, coord: HexCoord) -> Vec2 {
        let (q, r, _) = coord.qrs_f32();

        let size: f32 = self.cell_size.into();

        let x = (F0 * q + F1 * r) * size;
        let y = (F2 * q + F3 * r) * size;

        Vec2::new(x, -y)
    }

    pub fn coord_to_world_v3(&self, coord: HexCoord) -> Vec3 {
        let v = self.coord_to_world(coord);
        Vec3::new(v.x, v.y, 0.0)
    }
}
