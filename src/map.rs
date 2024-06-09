use std::collections::HashMap;

use petgraph::{adj::NodeIndex, stable_graph::{StableGraph, StableUnGraph}, EdgeType, Undirected};

use crate::{edge::Edge, HexCoord};

type HexGraphIndex = NodeIndex<u32>;
type HexGraph<T> = StableUnGraph<T, Edge, HexGraphIndex>;
type HexHashMap = HashMap<HexCoord, NodeIndex>;

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

    pub fn set(&mut self, coord: HexCoord, val: T) -> Option<u32>{
        let ind = self.graph.add_node(val);
        self.map.insert(coord, ind.index().try_into().unwrap())
    }

    // Should probably implement some kind of Entry Pattern similar to HashMap
    pub fn get(&self, coord: HexCoord) -> T {
        todo!()
    }

    // TODO: When adding a node to the table we need to link all the neighbours that exist
    fn connect_neighbours() {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::hex;

    use super::*;

    #[test]
    fn add_to_coord() {
        // Arrange
        let mut map = HexMap::<i32>::new(10);

        let coord = hex!(0,0);
        let value = 5;

        // Act
        let r = map.set(coord, value);

        // Assert
        assert!(r.is_none())
    }
}