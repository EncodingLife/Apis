use crate::Edge;

pub trait HexCoordinate<T> {
    fn qrs(&self) -> (T,T,T);
    fn qrs_f32(&self) -> (f32,f32,f32);
    fn dist(&self, rhs: Self) -> T;
    fn neighbour(self, edge: Edge) -> Self;
}