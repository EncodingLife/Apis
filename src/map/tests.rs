use crate::{hex, HexCoord, map::map::HexMap};

#[test]
fn create_map() {
    let map = HexMap::<bool>::new(6);
    assert_eq!(map.node_count(), 0);
}

#[test]
fn get_cell() {
    let mut map = HexMap::<bool>::new(6);
    assert_eq!(map.get(hex!(0,0)), None);

    let coord = hex!(0,0);
    map.set(coord, true);
    assert_eq!(map.get(coord), Some(&true))
}
