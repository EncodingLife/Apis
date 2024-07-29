use std::{ops::Index, slice::Iter};

use crate::{CellBucket, HexCoord};

#[derive(Copy, Clone)]
pub struct NeighbourhoodNode<T> {
    pub coords: HexCoord,
    pub value: Option<T>
}

impl<T: ?Sized + Default + Copy> NeighbourhoodNode<T> {
    pub fn new(coords: HexCoord, value: Option<T>) -> Self {
        Self {
            coords,
            value
        }
    }

    pub fn from_bucket(coords: HexCoord, bucket: CellBucket<T>) -> Self {
        Self {
            coords,
            value: match bucket {
                CellBucket::Occupied(_, v) => Some(v),
                CellBucket::Empty => None,
            }
        }
    }
}

pub struct Neighbourhood<T> {
    arr: [NeighbourhoodNode<T>; 7]
}

impl<T: ?Sized + Default + Copy> Neighbourhood<T> {
    pub(crate) fn new(top: [NeighbourhoodNode<T>;2], middle: [NeighbourhoodNode<T>;3], bottom: [NeighbourhoodNode<T>;2]) -> Self {
        Self {
            arr: [top[0], top[1], middle[0], middle[1], middle[2], bottom[0], bottom[1]]
        }
    }

    #[inline]
    pub fn center(&self) -> NeighbourhoodNode<T> {
        self.arr[3]
    }

    pub fn iter(&self) -> Iter<'_, NeighbourhoodNode<T>> {
        self.arr.iter()
    }
}

impl<T> Index<usize> for Neighbourhood<T> {
    type Output = NeighbourhoodNode<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}