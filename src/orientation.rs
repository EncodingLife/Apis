#[derive(Copy,Clone,Debug)]
pub enum HexOrientation {
    Flat,
    Pointy
}

impl HexOrientation {
    pub fn start_angle(self) -> f32 {
        match self {
            HexOrientation::Flat => 0.0,
            HexOrientation::Pointy => todo!(),
        }
    }
}