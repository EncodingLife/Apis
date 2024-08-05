use std::f32::consts::PI;

#[cfg(not(feature = "bevy"))]
type Vec2 = glam::Vec2;
#[cfg(feature = "bevy")]
type Vec2 = bevy::math::Vec2;

#[derive(Copy, Clone, Debug)]
pub enum HexOrientation {
    Flat,
    Pointy,
}

impl HexOrientation {
    pub fn start_angle(self) -> f32 {
        match self {
            HexOrientation::Flat => 0.0,
            HexOrientation::Pointy => todo!(),
        }
    }

    pub fn face_vec(&self, index: usize) -> Vec2 {
        self.corner_vec(index).rotate(Vec2::from_angle(PI / 2.0))
    }

    pub fn corner_vec(&self, index: usize) -> Vec2 {
        assert!(index < 6);
        match self {
            HexOrientation::Flat => Vec2::from_angle(index as f32 * (PI / -3.0)),
            HexOrientation::Pointy => todo!(),
        }
    }
}
