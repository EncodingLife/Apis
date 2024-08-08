use crate::{Edge, FlatMap, HexCoord, HexCoordinate, HexOrientation, HexWorldShape};

// Neighbourhoods
#[test]
pub fn neighbours_returns_correct_coords() {
    let map: FlatMap<bool> = FlatMap::new(HexWorldShape::Hexagon(2, HexOrientation::Flat));
    let center = HexCoord::from_axial(0, 0);

    let neighbourhood = map.neighbourhood(center);

    assert_eq!(neighbourhood[0].coords, center.neighbour(Edge::QS));
    assert_eq!(neighbourhood[1].coords, center.neighbour(Edge::Q));
    assert_eq!(neighbourhood[2].coords, center.neighbour(Edge::S));
    assert_eq!(neighbourhood[3].coords, center);
    assert_eq!(neighbourhood.center().coords, center);
    assert_eq!(neighbourhood[4].coords, center.neighbour(Edge::QR));
    assert_eq!(neighbourhood[5].coords, center.neighbour(Edge::RS));
    assert_eq!(neighbourhood[6].coords, center.neighbour(Edge::R));

    assert_eq!(neighbourhood.center().value, None);
}

#[test]
pub fn neighbours_on_map_edge_returns_correct_coords() {
    let map: FlatMap<bool> = FlatMap::new(HexWorldShape::Hexagon(2, HexOrientation::Flat));
    let center = HexCoord::from_axial(1, -1);

    let neighbourhood = map.neighbourhood(center);

    assert_eq!(neighbourhood[0].coords, center.neighbour(Edge::QS));
    assert_eq!(neighbourhood[1].coords, center.neighbour(Edge::Q));
    assert_eq!(neighbourhood[2].coords, center.neighbour(Edge::S));
    assert_eq!(neighbourhood[3].coords, center);
    assert_eq!(neighbourhood.center().coords, center);
    assert_eq!(neighbourhood[4].coords, center.neighbour(Edge::QR));
    assert_eq!(neighbourhood[5].coords, center.neighbour(Edge::RS));
    assert_eq!(neighbourhood[6].coords, center.neighbour(Edge::R));

    assert_eq!(neighbourhood.center().value, None);
}

#[test]
pub fn neighbourhood_with_values() {
    let mut map: FlatMap<bool> = FlatMap::new(HexWorldShape::Hexagon(2, HexOrientation::Flat));
    let center = HexCoord::from_axial(-1, 0);

    map.set(center.neighbour(Edge::RS), Some(true));

    let neighbourhood = map.neighbourhood(center);

    assert_eq!(neighbourhood.center().value, None);
    assert_eq!(neighbourhood[5].value, Some(true));
}
