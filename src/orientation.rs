use std::f32::consts::{PI};

#[cfg(not(feature = "bevy"))]
type Vec2 = glam::Vec2;
#[cfg(feature = "bevy")]
type Vec2 = bevy::math::Vec2;

const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
const FA: f32 = SQRT_3;
const FB: f32 = SQRT_3 / 2.0;
const FC: f32 = 0.0;
const FD: f32 = 3.0 / 2.0;

#[derive(Copy, Clone, Debug)]
pub enum HexOrientation {
    Flat,
    Pointy,
}

impl HexOrientation {
    pub fn start_angle(self) -> f32 {
        match self {
            HexOrientation::Flat => 0.0,
            HexOrientation::Pointy => 0.5,
        }
    }

    pub fn face_vec(&self, index: usize) -> Vec2 {
        self.corner_vec(index).rotate(Vec2::from_angle(PI / 2.0))
    }

    pub fn corner_vec(&self, index: usize) -> Vec2 {
        assert!(index < 6);
        let step = PI / -3.0; // 60°
        match self {
            HexOrientation::Flat => Vec2::from_angle(index as f32 * step),
            HexOrientation::Pointy => Vec2::from_angle(index as f32 * step - (step/2.0)),
        }
    }

    #[inline]
    pub fn f0(self) -> f32 {
        match self {
            HexOrientation::Pointy => FA,
            HexOrientation::Flat => FD,
        }
    }

    #[inline]
    pub fn f1(self) -> f32 {
        match self {
            HexOrientation::Pointy => FB,
            HexOrientation::Flat => FC,
        }
    }

    #[inline]
    pub fn f2(self) -> f32 {
        match self {
            HexOrientation::Pointy => FC,
            HexOrientation::Flat => FB,
        }
    }

    #[inline]
    pub fn f3(self) -> f32 {
        match self {
            HexOrientation::Pointy => FD,
            HexOrientation::Flat => FA,
        }
    }
}
