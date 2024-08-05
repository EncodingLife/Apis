#[cfg(feature = "bevy")]
use bevy::log::debug;
#[cfg(not(feature = "bevy"))]
use log::debug;

use crate::HexCoord;

use super::{hexagon::HexagonIndexer, rectangle::RectangleIndexer, Indexer};

#[derive(Copy, Clone, Debug)]
pub enum MapIndex {
    Hexagon(HexagonIndexer),
    Rectangle(RectangleIndexer),
}

impl MapIndex {
    pub fn new(shape: crate::HexWorldShape) -> Self {
        match shape {
            crate::HexWorldShape::Hexagon(radius) => Self::Hexagon(HexagonIndexer::new(radius)),
            crate::HexWorldShape::Rectangle(width, height) => {
                Self::Rectangle(RectangleIndexer::new(width, height))
            }
            crate::HexWorldShape::Square(width) => {
                Self::Rectangle(RectangleIndexer::new(width, width))
            }
        }
    }

    #[inline]
    pub fn index(self, coord: HexCoord) -> usize {
        match self {
            MapIndex::Hexagon(indexer) => indexer.index(coord),
            MapIndex::Rectangle(indexer) => indexer.index(coord),
        }
    }
    #[inline]

    pub fn try_index(self, coord: HexCoord) -> Option<usize> {
        match self {
            MapIndex::Hexagon(indexer) => indexer.try_index(coord),
            MapIndex::Rectangle(indexer) => indexer.try_index(coord),
        }
    }

    #[inline]
    pub fn capacity(self) -> usize {
        match self {
            MapIndex::Hexagon(indexer) => indexer.capacity(),
            MapIndex::Rectangle(indexer) => indexer.capacity(),
        }
    }

    #[inline]
    pub fn coord(self, index: usize) -> HexCoord {
        match self {
            MapIndex::Hexagon(indexer) => indexer.coords(index),
            MapIndex::Rectangle(indexer) => indexer.coords(index),
        }
    }
}
