use crate::{HexCoord, HexCoordinate, HexOrientation, MapIndex};

#[derive(Copy, Clone)]
pub enum HexWorldShape {
    // Hexagon with it's radius in cells incl origin
    Hexagon(usize, HexOrientation),
    // Rectangle with it's width and height in cells
    Rectangle(usize, usize, HexOrientation),
    // Rectangle with equal width and height
    Square(usize, HexOrientation),
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
    // #[cfg(not(feature="bevy"))]
    // pub cell_size: glam::Vec2,
    // #[cfg(feature = "bevy")]
    // pub cell_size: bevy::math::Vec2,
    pub cell_size: U,
    pub world_shape: HexWorldShape,
    pub indexer: MapIndex
}

impl<U> HexWorld<U>
where
    U: Copy,
{
    pub fn new(world_shape: HexWorldShape, cell_size: U) -> Self {
        Self {
            cell_size,
            world_shape,
            indexer: MapIndex::new(world_shape)
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

        let x = (self.world_shape.orientation().f0() * q + self.world_shape.orientation().f1() * r) * size;
        let y = (self.world_shape.orientation().f2() * q + self.world_shape.orientation().f3() * r) * size;

        Vec2::new(x, -y)
    }

    pub fn coord_to_world_v3(&self, coord: HexCoord) -> Vec3 {
        let v = self.coord_to_world(coord);
        Vec3::new(v.x, v.y, 0.0)
    }

    pub fn center(&self) -> HexCoord {
        match self.world_shape {
            HexWorldShape::Hexagon(_, _) => HexCoord::new(0,0,0),
            HexWorldShape::Rectangle(_, _, _) | HexWorldShape::Square(_,_) => self.indexer.coord(self.indexer.capacity() / 2)
        }
    }
}

impl HexWorldShape {
    pub fn orientation(&self) -> &HexOrientation {
        match self {
            HexWorldShape::Hexagon(_, o) |
            HexWorldShape::Rectangle(_, _, o) |
            HexWorldShape::Square(_, o) => o,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(HexWorldShape::Square(5, HexOrientation::Pointy), HexCoord::from_axial(1,2))]
    #[test_case(HexWorldShape::Square(5, HexOrientation::Flat), HexCoord::from_axial(2,1))]
    fn world_center_tests(world_shape: HexWorldShape, expected: HexCoord) {
        let world = HexWorld::new(world_shape, 1.0);
        assert_eq!(world.center(), expected)
    }
}
