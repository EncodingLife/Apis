use crate::{map::{neighbourhood, HexMap}, Edge, HexCoord};


#[test]
fn only_cell_populated() {
    let mut map = HexMap::<bool>::new(6);
    let coords = HexCoord::new(1,-1,0);
    map.set(coords, false);

    let neighbourhood = map.get_neighbourhood(coords);

    assert_eq!(neighbourhood.cell, Some(&false));
    assert!(!neighbourhood.neighbours.iter().any(|n| n.is_some()));

}

#[test]
fn target_cell_unpopulated_with_populated_neighbours() {
    let coords = HexCoord::new(-2,1,1);

    let mut map = HexMap::new(64);
    map.set(coords, 6u8);

    let neighbourhood = map.get_neighbourhood(HexCoord::new(-2,2,0));

    assert_eq!(neighbourhood.cell, None);
    assert_eq!(neighbourhood.neighbours[Edge::Q.index()], Some(&6));

}