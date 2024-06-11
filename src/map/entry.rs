use crate::HexCoord;

use super::{map::{HexIndex, HexMap}, node_ref::NodeRef};

pub enum MapEntry<'a, T: 'a> {
    Occupied(OccupiedMapEntry<'a, T>),
    Vacant(VacantMapEntry<'a, T>)
}

pub struct OccupiedMapEntry<'a, T> {
    pub(super) coord: HexCoord,
    pub(super) node: NodeRef<T>,
    pub(super) map: &'a mut HexMap<T>
}

pub struct VacantMapEntry<'a, T> {
    pub(super) coord: HexCoord,
    pub(super) map: &'a mut HexMap<T>
}

// MapEntry

impl<'m, T> MapEntry<'m, T> {
    pub fn or_insert(self, default: T) -> &'m mut T {
        match self {
            MapEntry::Occupied(o) => o.into_mut(),
            MapEntry::Vacant(v) => v.insert(default),
        }
    }
}

// Occupied

impl<'m, T> OccupiedMapEntry<'m, T> {
    pub fn into_mut(self) -> &'m mut T {
        unsafe { self.node.as_mut() }
    }
}

// Vacant

impl<'m, T> VacantMapEntry<'m, T> {
    pub fn insert(self, value: T) -> &'m mut T {
        unsafe {
            let node = self.map.insert_and_get_ref(self.coord, value);
            node.as_mut()
        }
    }
}