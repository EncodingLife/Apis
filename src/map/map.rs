use std::collections::HashMap;

use petgraph::{graph::NodeIndex, stable_graph::StableUnGraph};

use crate::{edge::Edge, HexCoord};

use super::{entry::{MapEntry, OccupiedMapEntry, VacantMapEntry}, node_ref::NodeRef};

pub(crate) type HexIndex = u32;
type HexGraph<T> = StableUnGraph<T, Edge, HexIndex>;
type HexHashMap = HashMap<HexCoord, NodeIndex<HexIndex>>;

pub struct HexMap<T> {
    graph: HexGraph<T>,
    map: HexHashMap
}

impl<T> HexMap<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity < u32::MAX.try_into().unwrap());
        Self {
            graph: HexGraph::with_capacity(capacity, 6),
            map: HexHashMap::new()
        }
    }

    pub fn node_count(self) -> usize {
        self.graph.node_count()
    }

    pub fn get(&self, coord: HexCoord) -> Option<&T> {
        let index = self.map.get(&coord).clone();
        match index {
            Some(&i) => self.graph.node_weight(i),
            None => None
        }
    }

    pub fn insert(&mut self, coord: HexCoord, value: T) -> NodeIndex {
        let index = self.graph.add_node(value);
        self.map.insert(coord, index);
        index
    }

    pub unsafe fn insert_and_get_ref(&mut self, coord: HexCoord, value: T) -> NodeRef<T> {
        let index = self.insert(coord, value);

        let node = self.graph.node_weight_mut(index).unwrap_or_else(|| panic!("Could not retrieve inserted graph node"));

        NodeRef::new(node)
    }

    pub fn entry(&mut self, coord: HexCoord) -> MapEntry<T> {
        let index = self.map.get(&coord).clone();
        match index {
            Some(&i) => {
                let value = self.graph.node_weight_mut(i).unwrap_or_else(|| panic!("Index not present in graph {i:?}"));
                    unsafe { MapEntry::Occupied(OccupiedMapEntry {
                        coord: coord,
                        node: NodeRef::new(value),
                        map: self
                    })
                }
            },
            None => MapEntry::Vacant(VacantMapEntry {
                coord: coord,
                map: self
            })
        }
    }
}