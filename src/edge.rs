/*
        q
      _____
  qs /     \ qr
    /       \
    \       /
  s  \_____/  r
        sr
*/

use crate::HexCoord;

pub const Edges: [Edge; 6] = [Edge::Q, Edge::QR, Edge::R, Edge::RS, Edge::S, Edge::QS];

#[derive(Copy, Clone)]
pub enum Edge {
    Q,
    QR,
    R,
    RS,
    S,
    QS,
}

impl Edge {
    pub fn offset_flat(self) -> HexCoord {
        match self {
            Edge::Q => HexCoord::new(0, -1, 1),
            Edge::QR => HexCoord::new(1, -1, 0),
            Edge::R => HexCoord::new(1, 0, -1),
            Edge::RS => HexCoord::new(0, 1, -1),
            Edge::S => HexCoord::new(-1, 1, 0),
            Edge::QS => HexCoord::new(-1, 0, 1),
        }
    }

    pub fn index(self) -> usize {
        match self {
            Edge::Q => 0,
            Edge::QR => 1,
            Edge::R => 2,
            Edge::RS => 3,
            Edge::S => 4,
            Edge::QS => 5,
        }
    }
}
