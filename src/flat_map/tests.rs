use test_case::test_case;

// Indexer

use crate::{flat_map::indexer::Indexer, HexCoord};

use super::indexer::HexagonIndexer;

#[test_case(1, 1)]
#[test_case(2, 7)]
#[test_case(3, 19)]
#[test_case(4, 37)]
#[test_case(5, 61)]
pub fn hexagon_indexer_capacity(radius: usize, expected: usize) {
    let indexer = HexagonIndexer::new(radius);
    assert_eq!(indexer.capacity(), expected)
}

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(5)]
#[test_case(200)]

pub fn hexagon_indexer_index_at_center_always_0(radius: usize) {
    let indexer = HexagonIndexer::new(radius);
    assert_eq!(indexer.index(HexCoord::from_qr(0,0)), 0)
}

#[test_case(2,0,0,0)]
#[test_case(2,1,-1,1)]
#[test_case(2,0,1,2)]
#[test_case(2,1,0,3)]
#[test_case(2,-1,0,4)]
#[test_case(2,0,-1,5)]
#[test_case(2,-1,1,6)]
pub fn hexagon_indexer_index(radius: usize, q: i32, r: i32, expected: usize) {
    let indexer = HexagonIndexer::new(radius);
    let coords = HexCoord::from_qr(q, r);
    assert_eq!(indexer.index(coords), expected);
}

#[test_case(2,0,0,0)]
#[test_case(2,1,1,-1)]
#[test_case(2,4,-1,0)]

pub fn hexagon_indexer_coords(radius: usize, index: usize, expected_q: i32, expected_r: i32) {
    let indexer = HexagonIndexer::new(radius);
    let coords = indexer.coords(index);
    assert_eq!(coords, HexCoord::from_qr(expected_q, expected_r))
}

#[test_case(1, 0, 0)]
#[test_case(3, 1, -1)]
pub fn hexagon_indexer_hex_to_index_to_hex(radius: usize, q: i32, r: i32) {
    let indexer = HexagonIndexer::new(radius);
    let coords = HexCoord::from_qr(q, r);
    assert_eq!(indexer.coords(indexer.index(coords)), coords);
}