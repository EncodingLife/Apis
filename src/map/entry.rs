use std::{cell::UnsafeCell, marker::PhantomData, ptr};

use petgraph::graph::NodeIndex;

use crate::HexCoord;

use super::map::HexMap;


pub struct UnsafeMapCell<'a, T>(*mut HexMap<T>, PhantomData<(&'a HexMap<T>, &'a UnsafeCell<HexMap<T>>)>);

impl<'a, T> UnsafeMapCell<'a, T> {
    pub(crate) fn new_mutable(map: &'a mut HexMap<T>) -> Self {
        Self(ptr::from_mut(map), PhantomData)
    }

    pub(crate) fn new(map: &'a HexMap<T>) -> Self {
        Self(ptr::from_ref(map).cast_mut(), PhantomData)
    }
}

pub struct UnsafeNodeCell<'a, T> {
    map: UnsafeMapCell<'a, T>,
    coord: HexCoord,
    index: NodeIndex<u32>
}

impl<'a, T> UnsafeNodeCell<'a, T> {
    pub(crate) fn new(map: UnsafeMapCell<'a, T>, coord: HexCoord, index: NodeIndex<u32>) -> Self {
        UnsafeNodeCell {
            map,
            coord,
            index
        }
    }
}

pub struct NodeRef<'a, T>(UnsafeNodeCell<'a, T>);

impl<'a, T> NodeRef<'a, T> {
    pub(crate) unsafe fn new(cell: UnsafeNodeCell<'a,T>) -> Self {
        Self(cell)
    }
}

pub struct NodeRefMut<'a, T>(UnsafeNodeCell<'a, T>);

impl<'a, T> NodeRefMut<'a, T> {
    pub(crate) unsafe fn new(cell: UnsafeNodeCell<'a,T>) -> Self {
        Self(cell)
    }
}

impl<T> HexMap<T> {
    pub(crate) fn as_unsafe_map_mut_cell(&mut self) -> UnsafeMapCell<T> {
        UnsafeMapCell::new_mutable(self)
    }

    pub(crate) fn as_unsafe_map_cell(&self) -> UnsafeMapCell<T> {
        UnsafeMapCell::new(self)
    }
}

pub enum MapEntry<'a, T> {
    Occupied(OccupiedMapEntry<'a, T>),
    Vacant(VacantMapEntry<'a, T>)
}

pub struct OccupiedMapEntry<'a, T> {
    node_ref: &'a mut NodeRefMut<'a, T>,
    _marker: PhantomData<T>
}

impl<'a, T> OccupiedMapEntry<'a, T> {

}

pub struct VacantMapEntry<'a, T> {
    node_ref: &'a mut NodeRefMut<'a, T>,
    _marker: PhantomData<T>
}

impl<'a, T> VacantMapEntry<'a, T> {

}
