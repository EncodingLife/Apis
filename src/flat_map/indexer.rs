#[cfg(not(feature="bevy"))]
use log::debug;
#[cfg(feature = "bevy")]
use bevy::log::debug;

use crate::{HexCoord, HexCoordinate, HexWorldShape};

#[derive(Copy, Clone, Debug)]
pub enum MapIndex {
    Hexagon(HexagonIndexer)
}

#[derive(Copy, Clone, Debug)]
pub struct HexagonIndexer {
    radius: usize,
    capacity: i32,
    shift: i32
}

impl MapIndex {
    pub fn new(shape: crate::HexWorldShape) -> Self {
        match shape {
            crate::HexWorldShape::Hexagon(radius) => Self::Hexagon(HexagonIndexer::new(radius)),
            crate::HexWorldShape::Square(_, _) => todo!(),
        }
    }

    #[inline]
    pub fn index(self, coord: HexCoord) -> usize {
        match self {
            MapIndex::Hexagon(indexer) => indexer.index(coord),
        }
    }

    #[inline]
    pub fn capacity(self) -> usize {
        match self {
            MapIndex::Hexagon(indexer) => indexer.capacity(),
        }
    }

    #[inline]
    pub fn coord(self, index: usize) -> HexCoord {
        match self {
            MapIndex::Hexagon(indexer) => indexer.coords(index),
        }
    }
}

impl HexagonIndexer {
    pub fn new(radius: usize) -> Self {
        assert!(radius > 0);
        Self {
            radius,
            capacity: hexagon_shape_area(radius), // #https://observablehq.com/@sanderevers/hexmod-representation
            shift: 3 * (i32::try_from(radius).unwrap() - 1) + 2
        }
    }
}

// https://observablehq.com/@sanderevers/hexmod-representation
fn hex_mod(coords: HexCoord, shift: i32, area: i32) -> usize {
    let (q, r, s) = coords.qrs();
    let t = ((q + (s*shift)) + area) % area;
    match usize::try_from(t) {
        Ok(u) => u,
        Err(_) => {
            println!("error calc hex_mod for {coords:?}, shift:{shift}, a:{area} => {}", (t));
            panic!("");
        }
    }
}

fn inv_hex_mod(index: usize, shift: i32, radius: i32) -> HexCoord {
    let i = i32::try_from(index).unwrap();
    let ms = (i+radius) / shift;
    let mcs = (i+2*radius) / (shift-1);
    let x = ms*(radius+1) + mcs*-radius;
    let y = i + ms*(-2*radius-1) + mcs*(-radius-1);
    let z = -i + ms*radius + mcs*(2*radius+1);
    HexCoord::new(y, z, x)
}

#[inline]
fn hexagon_shape_area(radius: usize) -> i32 {
    if radius <= 0 { return 0; }
    let r = i32::try_from(radius).unwrap() - 1;
    3*(r*r) + 3*(r) + 1
}

pub trait Indexer {
    fn index(&self, coord: HexCoord) -> usize;
    fn capacity(&self) -> usize;
    fn coords(&self, index: usize) -> HexCoord;
}

impl Indexer for HexagonIndexer {
    #[inline]
    fn capacity(&self) -> usize {
        usize::try_from(self.capacity).unwrap()
    }

    #[inline]
    fn index(&self, coords: HexCoord) -> usize {
        hex_mod(coords, self.shift, self.capacity)
    }

    #[inline]
    fn coords(&self, index: usize) -> HexCoord {
        inv_hex_mod(index, self.shift, i32::try_from(self.radius - 1).unwrap())
    }
}