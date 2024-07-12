#![cfg(feature = "bevy")]

use std::f32::consts::PI;

use bevy::{prelude::Primitive2d, render::{mesh::{Indices, Mesh, MeshBuilder, Meshable, PrimitiveTopology}, render_asset::RenderAssetUsages}};
use bevy::math::{Vec2,Vec3,Vec4};

use crate::{HexOrientation, HexWorld};


#[derive(Clone, Copy, Debug)]
pub struct Hexagon {
    orientation: HexOrientation,
    size: Vec2
}

impl Hexagon {
    pub fn new(orientation: HexOrientation, size: f32) -> Self {
        Self { orientation, size: Vec2::new(size, size) }
    }
}

impl Primitive2d for Hexagon {}

impl Meshable for Hexagon {
    type Output = HexagonMeshBuilder;
    fn mesh(&self) -> Self::Output {
        HexagonMeshBuilder { hexagon: *self }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HexagonMeshBuilder {
    hexagon: Hexagon
}

impl MeshBuilder for HexagonMeshBuilder {
    fn build(&self) -> Mesh {
        let verts = self.hexagon.polygon_corners().to_vec();

        let pos : Vec<Vec3> = verts.iter().map(|v| Vec3::new(v.x, v.y, 0.0)).collect();
        let normals = vec![[0.0,0.0,1.0]; 6];

        let uvs: Vec<Vec2> = (0..6).map(|i| {
            let a = self.hexagon.get_corner_angle(i);
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

impl<U> HexWorld<U> where f32: std::convert::From<U>, U: std::convert::From<f32>, U: Copy  {
    pub fn get_shape(&self) -> Hexagon {
        Hexagon::new(self.orientation, self.cell_size.into())
    }

    pub fn get_mesh_builder(&self) -> HexagonMeshBuilder {
        let hex = self.get_shape();
        hex.mesh()
    }
}

impl Hexagon {
    fn polygon_corners(self) -> [Vec2; 6] {
        let mut corners = [Vec2::default();6];
        for i in 0u8..6u8 {
            corners[i as usize] = self.hex_corner_offset(self.size, i);
        }
        corners
    }

    fn get_corner_angle(self, corner: u8) -> f32 {
        // 'as f32' may cause problems but for now its fine
        2.0 * PI * (self.orientation.start_angle() + corner as f32) / 6.0
    }

    fn hex_corner_offset(self, size: Vec2, corner: u8) -> Vec2 {
        let a = self.get_corner_angle(corner);
        Vec2::new(size.x * a.cos(), size.y * a.sin())
    }
}
