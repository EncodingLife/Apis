use crate::HexCoord;

mod hexagon;
pub mod map_index;
mod rectangle;

#[cfg(test)]
mod tests;

pub trait Indexer {
    fn index(&self, coord: HexCoord) -> usize;
    fn try_index(&self, coord: HexCoord) -> Option<usize>;
    fn capacity(&self) -> usize;
    fn coords(&self, index: usize) -> HexCoord;
    fn offset_coord(&self, col: i32, row: i32) -> HexCoord;
}
