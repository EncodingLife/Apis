use std::{cell::Cell, slice::Iter};

#[cfg(not(feature="bevy"))]
use log::debug;
#[cfg(feature = "bevy")]
use bevy::log::debug;


use crate::{HexCoord, HexWorldShape};

use super::indexer::{Indexer, MapIndex};

#[derive(Copy, Clone)]
pub enum CellBucket<T> {
    Occupied(HexCoord, T),
    Empty
}

impl<T> Default for CellBucket<T> {
    fn default() -> Self {
        Self::Empty
    }
}

pub struct FlatMap<T> {
    index: MapIndex,
    store: Vec<CellBucket<T>>
}

impl<T: ?Sized + Default + Copy> FlatMap<T> {
    pub fn new(shape: HexWorldShape) -> Self {
        let indexer = MapIndex::new(shape);
        let mut vec = Vec::new();
        vec.resize(indexer.capacity(), CellBucket::Empty);

        Self {
            index: indexer,
            store: vec
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

    pub fn init_with<F>(shape: HexWorldShape, f: F)  -> Self where
    F: FnOnce() -> T,    {
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

    #[inline]
    pub fn index(&self) -> MapIndex {
        self.index
    }

    pub fn iter(&self) -> Iter<'_, CellBucket<T>>  {
        self.store.iter()
    }
}