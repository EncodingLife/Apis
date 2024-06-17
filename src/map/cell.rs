use crate::HexCoord;

pub struct MapCell<'a, T> {
    coord: HexCoord,
    value: &'a T
}

impl<'a, T> MapCell<'a, T> {
    pub fn new(coord: HexCoord, value: &'a T) -> Self {
        MapCell { coord, value }
    }
}