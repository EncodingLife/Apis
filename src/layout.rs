use std::f32::consts::PI;

use glam::Vec2;

const ORIENTATION_START_ANGLE: u8 = 0;

#[derive(Copy,Clone,Debug)]
pub struct HexLayout {
    #[cfg(not(feature="bevy"))]
    pub size: glam::Vec2,
    #[cfg(feature = "bevy")]
    pub size: bevy::math::Vec2
}

impl HexLayout {
    pub fn new(width: f32) -> Self {
        Self {
            size: Vec2::new(width, width) // TODO
        }
    }

    pub fn polygon_corners(&self) -> [Vec2; 6] {
        let mut corners = [Vec2::default();6];
        for i in 0..6 {
            corners[i] = Self::hex_corner_offset(self.size, i.try_into().unwrap());
        }
        corners
    }

    pub fn get_corner_angle(corner: u8) -> f32 {
        2.0 * PI * (ORIENTATION_START_ANGLE + corner) as f32 / 6.0
    }

    pub fn hex_corner_offset(size: Vec2, corner: u8) -> Vec2 {
        let a = Self::get_corner_angle(corner);
        Vec2::new(size.x * a.cos(), size.y * a.sin())
    }
}