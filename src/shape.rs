#![cfg(feature = "bevy")]

use bevy::{prelude::Primitive2d, render::{mesh::{Indices, Mesh, Meshable, PrimitiveTopology}, render_asset::RenderAssetUsages}};
use bevy::math::{Vec2,Vec3,Vec4};

use crate::HexLayout;

#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    hex_layout: HexLayout
}

impl Hexagon {
    pub fn new(hex_layout: HexLayout) -> Self {
        Self { hex_layout }
    }
}

impl Primitive2d for Hexagon {}

impl Meshable for Hexagon {
    type Output = HexagonMeshBuilder;
    fn mesh(&self) -> Self::Output {
        HexagonMeshBuilder { hex_layout: self.hex_layout}
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HexagonMeshBuilder {
    hex_layout: HexLayout
}

impl HexagonMeshBuilder {
    pub fn build(&self) -> Mesh {
        let verts = self.hex_layout.polygon_corners().to_vec();

        let pos : Vec<Vec3> = verts.iter().map(|v| Vec3::new(v.x, v.y, 0.0)).collect();
        let normals = vec![[0.0,0.0,1.0]; 6];

        let uvs: Vec<Vec2> = (0..6).map(|i| {
            let a = HexLayout::get_corner_angle(i);
            Vec2::new(0.5 * (a.cos() + 1.0), 1.0 - 0.5 * (a.sin() + 1.0))
        }).collect();

        let colors: Vec<Vec4> = (0..6).map(|_| Vec4::new(1.0,1.0,1.0,1.0)).collect();

        let indices = vec![
            0,2,1,
            3,5,4,
            0,5,3,
            3,2,0];

        Mesh::new(PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_indices(Indices::U32(indices))
    }
}

impl From<Hexagon> for Mesh {
    fn from(hex: Hexagon) -> Self {
        hex.mesh().build()
    }
}