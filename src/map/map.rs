use std::collections::HashMap;

use petgraph::{graph::NodeIndex, stable_graph::StableUnGraph};

use crate::{map::neighbourhood, Edge, HexCoord, HexCoordinate};

use super::{entry::{NodeRef, NodeRefMut, UnsafeNodeCell}, neighbourhood::Neighbourhood};

pub struct HexMap<T> {
    graph: StableUnGraph<T, Edge, u32>,
    map: HashMap<HexCoord, NodeIndex<u32>>
}

impl<T> HexMap<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity < u32::MAX.try_into().unwrap());
        Self {
            graph: StableUnGraph::with_capacity(capacity, 6),
            map: HashMap::with_capacity(capacity)
        }
    }

    // TODO: This may not be here long term
    pub fn set(&mut self, coords: HexCoord, value: T) {
        let index = self.graph.add_node(value);
        self.map.insert(coords, index);
    }

    pub fn get(&self, coord: HexCoord) -> Option<&T> {
        match self.get_index(coord) {
            Some(i) => self.graph.node_weight(i),
            None => None,
        }
    }

    pub fn get_mut(&mut self, coord: HexCoord) -> Option<&mut T> {
        let index = self.get_index(coord);
        match index {
            Some(i) => self.graph.node_weight_mut(i),
            None => None,
        }
    }

    pub fn contains(&self, coord:HexCoord) -> bool {
        self.map.get(&coord).is_some()
    }

    pub fn node_count(self) -> usize {
        self.map.iter().count()
    }

    pub fn get_node_ref_mut(&mut self, coords:HexCoord) -> Option<NodeRefMut<T>> {
        let index = self.get_index(coords)?;
        let node_cell = UnsafeNodeCell::new(self.as_unsafe_map_mut_cell(), coords, index);
        let node_ref = unsafe { NodeRefMut::new(node_cell) };
        Some(node_ref)
    }

    pub fn get_node_ref(&self, coords:HexCoord) -> Option<NodeRef<T>> {
        let index = self.get_index(coords)?;
        let node_cell = UnsafeNodeCell::new(self.as_unsafe_map_cell(), coords, index);
        let node_ref = unsafe { NodeRef::new(node_cell) };
        Some(node_ref)
    }

    pub fn get_neighbourhood(&self, coords: HexCoord) -> Neighbourhood<T> {
        let cell = self.get(coords);
        let mut neighbours = [None;6];
        for edge in [Edge::Q, Edge::QR] {
            neighbours[edge.index()] = self.get(coords.neighbour(edge));
        }
        Neighbourhood {
            cell,
            neighbours
        }
    }

    fn get_index(&self, coord: HexCoord) -> Option<NodeIndex<u32>> {
        match self.map.get(&coord) {
            Some(&i) => Some(i),
            None => None
        }
    }
}