use crate::Edge;

pub trait HexCoordinate<T> {
    fn q(&self) -> T;
    fn r(&self) -> T;
    fn s(&self) -> T;
    fn qrs(&self) -> (T,T,T);
    fn qrs_f32(&self) -> (f32,f32,f32);
    fn dist(&self, rhs: Self) -> T;
    fn neighbour(self, edge: Edge) -> Self;
    fn ring(self, radius: usize) -> Vec<Self> where Self: Sized;
    fn reflect_q(&self) -> Self;
    fn reflect_r(&self) -> Self;
    fn reflect_s(&self) -> Self;
}