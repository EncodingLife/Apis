use std::{mem, slice::Iter};

#[cfg(feature = "bevy")]
use bevy::log::debug;
#[cfg(not(feature = "bevy"))]
use log::debug;

use crate::{
    flat_map::neighbourhood::NeighbourhoodNode, index::map_index::MapIndex, Edge, HexCoord,
    HexCoordinate, HexWorldShape,
};

use super::neighbourhood::Neighbourhood;

#[derive(Copy, Clone)]
pub enum CellBucket<T> {
    Occupied(HexCoord, T),
    Empty,
}

impl<T> CellBucket<T> {
    #[allow(dead_code)]
    fn unwrap(&self) -> (&HexCoord, &T) {
        match self {
            CellBucket::Occupied(c, v) => (c, v),
            CellBucket::Empty => panic!("Cannot unwrap empty CellBucket"),
        }
    }
}

impl<T> Default for CellBucket<T> {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct FlatMap<T> {
    index: MapIndex,
    store: Vec<CellBucket<T>>,
}

impl<T: ?Sized + Default + Copy> FlatMap<T> {
    pub fn new(shape: HexWorldShape) -> Self {
        let indexer = MapIndex::new(shape);
        let mut vec = Vec::new();
        vec.resize(indexer.capacity(), CellBucket::Empty);

        Self {
            index: indexer,
            store: vec,
        }
    }

    pub fn set(&mut self, coords: HexCoord, value: Option<T>) -> Option<T> {
        let index = self.index.index(coords);
        self.set_internal(index, coords, value)
    }

    pub fn set_index(&mut self, index: usize, value: Option<T>) -> Option<T> {
        let coords = self.index.coord(index);
        self.set_internal(index, coords, value)
    }

    pub fn init_with<F>(shape: HexWorldShape, f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        let mut r = Self::new(shape);

        let v = f();

        for i in 0..r.index.capacity() {
            r.set_index(i, Some(v));
        }

        r
    }

    fn set_internal(&mut self, index: usize, coords: HexCoord, value: Option<T>) -> Option<T> {
        let prev = *self.store.get(index).unwrap();

        self.store[index] = match value {
            Some(v) => CellBucket::Occupied(coords, v),
            None => CellBucket::Empty,
        };

        match prev {
            CellBucket::Occupied(_, v) => Some(v),
            CellBucket::Empty => None,
        }
    }

    pub fn footprint(&self) -> usize {
        mem::size_of::<CellBucket<T>>() * &self.store.len() + mem::size_of::<MapIndex>()
    }

    #[inline]
    pub fn index(&self) -> MapIndex {
        self.index
    }

    pub fn iter(&self) -> Iter<'_, CellBucket<T>> {
        self.store.iter()
    }

    pub fn neighbourhood(&self, coords: HexCoord) -> Neighbourhood<T> {
        let t = self.get_segment(coords.neighbour(Edge::QS));
        let top = [
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::QS), t[0]),
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::Q), t[1]),
        ];

        let m = self.get_segment(coords.neighbour(Edge::S));
        let middle = [
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::S), m[0]),
            NeighbourhoodNode::from_bucket(coords, m[1]),
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::QR), m[2]),
        ];

        let b = self.get_segment(coords.neighbour(Edge::RS));
        let bottom = [
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::RS), b[0]),
            NeighbourhoodNode::from_bucket(coords.neighbour(Edge::R), b[1]),
        ];

        Neighbourhood::new(top, middle, bottom)
    }

    fn get_segment(&self, coords: HexCoord) -> [CellBucket<T>; 3] {
        let i = self.index().index(coords);
        let end = coords.neighbour(Edge::QR).neighbour(Edge::QR);
        let j = self.index().index(end);
        let d = self.index().capacity() - i;
        match d {
            1 => {
                let s = &self.store[0..=j];
                [self.store[i], s[0], s[1]]
            }
            2 => {
                let s = &self.store[i..];
                [s[0], s[1], self.store[j]]
            }
            _ => {
                let mut o = [CellBucket::default(); 3];
                o.copy_from_slice(&self.store[i..=j]);
                o
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{FlatMap, HexCoord, HexOrientation, HexWorldShape};

    #[test]
    fn get_segment_start_is_index_capacity() {
        let map: FlatMap<bool> = FlatMap::init_with(HexWorldShape::Hexagon(5, HexOrientation::Flat), || true);

        let s = map.get_segment(HexCoord::from_axial(-1, 1));

        assert_eq!(*s[0].unwrap().0, HexCoord::from_axial(-1, 1));
        assert_eq!(*s[1].unwrap().0, HexCoord::from_axial(0, 0));
        assert_eq!(*s[2].unwrap().0, HexCoord::from_axial(1, -1));
    }

    #[test]
    fn get_segment_start_is_one_less_than_index_capacity() {
        let map: FlatMap<bool> = FlatMap::init_with(HexWorldShape::Hexagon(5, HexOrientation::Flat), || true);

        let s = map.get_segment(HexCoord::from_axial(-2, 2));

        assert_eq!(*s[0].unwrap().0, HexCoord::from_axial(-2, 2));
        assert_eq!(*s[1].unwrap().0, HexCoord::from_axial(-1, 1));
        assert_eq!(*s[2].unwrap().0, HexCoord::from_axial(0, 0));
    }

    #[test]
    fn get_segment_start_coord_0_0() {
        let map: FlatMap<bool> = FlatMap::init_with(HexWorldShape::Hexagon(5, HexOrientation::Flat), || true);

        let s = map.get_segment(HexCoord::from_axial(0, 0));

        assert_eq!(*s[0].unwrap().0, HexCoord::from_axial(0, 0));
        assert_eq!(*s[1].unwrap().0, HexCoord::from_axial(1, -1));
        assert_eq!(*s[2].unwrap().0, HexCoord::from_axial(2, -2));
    }
}
