use crate::hex;
use crate::HexCoord;

use super::map::HexMap;

#[test]
fn create_map() {
    let map = HexMap::<bool>::new(6);
    assert_eq!(map.node_count(), 0);
}

#[test]
fn insert_node_and_get() {
    let mut map = HexMap::new(6);
    let hex = hex!(0,0);
    map.insert(hex!(0,0), 1);

    assert_eq!(map.get(hex), Some(&1));
    assert_eq!(map.get(hex!(1,0)), None);
}

#[test]
fn use_vacant_entry() {
    let mut map = HexMap::new(6);
    let hex = hex!(0,0);

    let entry = map.entry(hex).or_insert(3);
    assert_eq!(*entry, 3);

    *entry = 5;
    assert_eq!(map.get(hex), Some(&5));
}