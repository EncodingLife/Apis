


pub struct FractionalHexCoord([f32;3]);

// impl HexCoordinate<f32> for FractionalHexCoord {
//     fn new(q: f32, r: f32, s: f32) -> Self {
//         assert_eq!(q+r+s, 0.0, "QRS must add up to 0.0");
//         Self([q,r,s])
//     }

//     fn dist(&self, rhs: Self) -> f32 {
//         todo!()
//     }

//     fn polygon_corners(self) -> [[i32;3]; 6] {
//         todo!()
//     }
// }