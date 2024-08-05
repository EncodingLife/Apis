use crate::{HexCoord, HexCoordinate, HexOrientation};

use super::Indexer;

#[derive(Copy, Clone, Debug)]
pub struct RectangleIndexer {
    height: i32,
    capacity: i32,
    orientation: HexOrientation,
}

impl RectangleIndexer {
    pub fn new(width: usize, height: usize, orientation: HexOrientation) -> Self {
        assert!(width > 0 && height > 0);

        Self {
            height: height as i32,
            capacity: (width * height) as i32,
            orientation,
        }
    }
}

impl Indexer for RectangleIndexer {
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
        let row = match self.orientation {
            HexOrientation::Flat => coords.r() + (coords.q()/2),
            HexOrientation::Pointy => coords.r(),
        };
        let column = match self.orientation {
            HexOrientation::Flat => coords.q(),
            HexOrientation::Pointy => coords.q() + (coords.r()/2),
        };
        let i = (column * self.height) + row;
        if i > self.capacity {
            None
        } else {
            usize::try_from((column * self.height) + row).ok()
        }
    }

    #[inline]
    fn coords(&self, index: usize) -> HexCoord {
        assert!(index < self.capacity as usize);
        let column = index as i32 / self.height;
        let row = index as i32 % self.height;
        match self.orientation {
            HexOrientation::Flat => {
                let start_offset = column / 2;
                HexCoord::from_qr(column, -start_offset + row)
            },
            HexOrientation::Pointy => {
                let start_offset = row/2;
                HexCoord::from_qr(-start_offset+column, row)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::orientation;

    use super::*;
    use test_case::test_case;

    #[test_case(HexOrientation::Flat, 2, 2, 4)]
    #[test_case(HexOrientation::Flat, 3, 3, 9)]
    pub fn rectangle_indexer_capacity(
        orientation: HexOrientation,
        width: usize,
        height: usize,
        expected: usize,
    ) {
        let indexer = RectangleIndexer::new(width, height, orientation);
        assert_eq!(indexer.capacity(), expected)
    }

    #[test_case(HexOrientation::Flat, 2, 2, 0, 0, 0)]
    #[test_case(HexOrientation::Flat, 3, 3, 0, 0, 0)]
    pub fn rectangle_indexer_coords(
        orientation: HexOrientation,
        width: usize,
        height: usize,
        index: usize,
        expected_q: i32,
        expected_r: i32,
    ) {
        let indexer = RectangleIndexer::new(width, height, orientation);
        let r = indexer.coords(index);
        let e = HexCoord::from_qr(expected_q, expected_r);
        assert_eq!(r, e, "Index {index} mapping to incorrect coord")
    }


    #[test_case(0, HexCoord::from_qr(0,0))]
    #[test_case(1, HexCoord::from_qr(0,1))]
    #[test_case(2, HexCoord::from_qr(0,2))]
    #[test_case(8, HexCoord::from_qr(1,3))]
    #[test_case(17, HexCoord::from_qr(3,1))]
    #[test_case(21, HexCoord::from_qr(4,-1))]
    #[test_case(23, HexCoord::from_qr(4,1))]
    fn flat_orientation_5_by_5(index: usize, coords: HexCoord) {
        let indexer = RectangleIndexer::new(5,5,HexOrientation::Flat);
        assert_eq!(indexer.coords(index), coords);
        assert_eq!(indexer.index(coords), index);
    }

    #[test_case(0, HexCoord::from_qr(0,0))]
    #[test_case(1, HexCoord::from_qr(0,1))]
    #[test_case(2, HexCoord::from_qr(-1,2))]
    #[test_case(8, HexCoord::from_qr(1,2))]
    #[test_case(6, HexCoord::from_qr(2,0))]
    fn pointy_orientation_3_by_3(index: usize, coords: HexCoord) {
        let indexer = RectangleIndexer::new(3,3,HexOrientation::Pointy);
        assert_eq!(indexer.coords(index), coords);
        assert_eq!(indexer.index(coords), index);
    }

    #[test]
    pub fn try_index_does_not_error() {
        let indexer = RectangleIndexer::new(3, 3, HexOrientation::Flat);
        assert_eq!(indexer.try_index(HexCoord::from_qr(100, 100)), None);
    }
}
