use glam::{Vec2, Vec3};

use crate::HexLayout;

pub trait HexCoordinate<T> {
    fn qrs(&self) -> (T,T,T);

    fn dist(&self, rhs: Self) -> T;
    fn to_world(&self, layout: HexLayout) -> Vec2;
    fn to_world_v3(&self, layout: HexLayout) -> Vec3;
}