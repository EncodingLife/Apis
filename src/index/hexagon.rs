use std::num::TryFromIntError;

use crate::{HexCoord, HexCoordinate, HexOrientation};

use super::Indexer;

#[derive(Copy, Clone, Debug)]
pub struct HexagonIndexer {
    radius: usize,
    capacity: i32,
    shift: i32,
    orientation: HexOrientation
}

impl HexagonIndexer {
    pub fn new(radius: usize, orientation: HexOrientation) -> Self {
        assert!(radius > 0);
        Self {
            radius,
            capacity: hexagon_shape_area(radius), // #https://observablehq.com/@sanderevers/hexmod-representation
            shift: 3 * (i32::try_from(radius).unwrap() - 1) + 2,
            orientation
        }
    }
}

impl Indexer for HexagonIndexer {
    #[inline]
    fn capacity(&self) -> usize {
        usize::try_from(self.capacity).unwrap()
    }

    #[inline]
    fn index(&self, coords: HexCoord) -> usize {
        self.try_index(coords).unwrap()
    }

    #[inline]
    fn try_index(&self, coords: HexCoord) -> Option<usize> {
        match self.orientation {
            HexOrientation::Flat => hex_mod(coords, self.shift, self.capacity).ok(),
            HexOrientation::Pointy => todo!(),
        }
    }

    #[inline]
    fn coords(&self, index: usize) -> HexCoord {
        match self.orientation {
            HexOrientation::Flat => inv_hex_mod(index, self.shift, i32::try_from(self.radius - 1).unwrap()),
            HexOrientation::Pointy => todo!(),
        }
    }

    fn offset_coord(&self, _col: i32, _row: i32) -> HexCoord {
        todo!()
    }
}

// https://observablehq.com/@sanderevers/hexmod-representation
fn hex_mod(coords: HexCoord, shift: i32, area: i32) -> Result<usize, TryFromIntError> {
    let (q, _, s) = coords.qrs();
    let t = ((q + (s * shift)) + area) % area;
    usize::try_from(t)
}

fn inv_hex_mod(index: usize, shift: i32, radius: i32) -> HexCoord {
    let i = i32::try_from(index).unwrap();
    let ms = (i + radius) / shift;
    let mcs = (i + 2 * radius) / (shift - 1);
    let x = ms * (radius + 1) + mcs * -radius;
    let y = i + ms * (-2 * radius - 1) + mcs * (-radius - 1);
    let z = -i + ms * radius + mcs * (2 * radius + 1);
    HexCoord::new(y, z, x)
}

#[inline]
fn hexagon_shape_area(radius: usize) -> i32 {
    if radius <= 0 {
        return 0;
    }
    let r = i32::try_from(radius).unwrap() - 1;
    3 * (r * r) + 3 * (r) + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1)]
    #[test_case(2, 7)]
    #[test_case(3, 19)]
    #[test_case(4, 37)]
    #[test_case(5, 61)]
    pub fn hexagon_indexer_capacity(radius: usize, expected: usize) {
        let indexer = HexagonIndexer::new(radius, HexOrientation::Flat);
        assert_eq!(indexer.capacity(), expected)
    }

    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(5)]
    #[test_case(200)]

    pub fn hexagon_indexer_index_at_center_always_0(radius: usize) {
        let indexer = HexagonIndexer::new(radius, HexOrientation::Flat);
        assert_eq!(indexer.index(HexCoord::from_axial(0, 0)), 0)
    }

    #[test_case(2, 0, 0, 0)]
    #[test_case(2,1,-1,1)]
    #[test_case(2, 0, 1, 2)]
    #[test_case(2, 1, 0, 3)]
    #[test_case(2,-1,0,4)]
    #[test_case(2,0,-1,5)]
    #[test_case(2,-1,1,6)]
    pub fn hexagon_indexer_index(radius: usize, q: i32, r: i32, expected: usize) {
        let indexer = HexagonIndexer::new(radius, HexOrientation::Flat);
        let coords = HexCoord::from_axial(q, r);
        assert_eq!(indexer.index(coords), expected);
    }

    #[test_case(2, 0, 0, 0)]
    #[test_case(2,1,1,-1)]
    #[test_case(2,4,-1,0)]

    pub fn hexagon_indexer_coords(radius: usize, index: usize, expected_q: i32, expected_r: i32) {
        let indexer = HexagonIndexer::new(radius, HexOrientation::Flat);
        let coords = indexer.coords(index);
        assert_eq!(coords, HexCoord::from_axial(expected_q, expected_r))
    }

    #[test_case(1, 0, 0)]
    #[test_case(3, 1, -1)]
    pub fn hexagon_indexer_hex_to_index_to_hex(radius: usize, q: i32, r: i32) {
        let indexer = HexagonIndexer::new(radius, HexOrientation::Flat);
        let coords = HexCoord::from_axial(q, r);
        assert_eq!(indexer.coords(indexer.index(coords)), coords);
    }

    #[test]
    pub fn try_index_does_not_error() {
        let indexer = HexagonIndexer::new(3, HexOrientation::Flat);
        assert_eq!(indexer.try_index(HexCoord::from_axial(100, 100)), None);
    }
}
