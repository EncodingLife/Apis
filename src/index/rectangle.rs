use crate::{HexCoord, HexCoordinate};

use super::Indexer;

#[derive(Copy, Clone, Debug)]
pub struct RectangleIndexer {
    height: i32,
    capacity: i32,
}

impl RectangleIndexer {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0);

        Self {
            height: height as i32,
            capacity: (width * height) as i32,
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
        let column = coords.q();
        let row = coords.r();
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
        let start_offset = column / 2;
        HexCoord::from_qr(column, -start_offset + row)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 2, 4)]
    #[test_case(3, 3, 9)]
    pub fn rectangle_indexer_capacity(width: usize, height: usize, expected: usize) {
        let indexer = RectangleIndexer::new(width, height);
        assert_eq!(indexer.capacity(), expected)
    }

    #[test_case(2, 2, 0, 0, 0)]
    #[test_case(3, 3, 0, 0, 0)]
    pub fn rectangle_indexer_coords(
        width: usize,
        height: usize,
        index: usize,
        expected_q: i32,
        expected_r: i32,
    ) {
        let indexer = RectangleIndexer::new(width, height);
        let r = indexer.coords(index);
        let e = HexCoord::from_qr(expected_q, expected_r);
        assert_eq!(r, e, "Index {index} mapping to incorrect coord")
    }

    #[test]
    pub fn try_index_does_not_error() {
        let indexer = RectangleIndexer::new(3, 3);
        assert_eq!(indexer.try_index(HexCoord::from_qr(100, 100)), None);
    }
}
