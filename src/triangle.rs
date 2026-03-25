use crate::types::*;
use crate::rasterizer::*;
pub struct Triangle {
    pub vertex1: Point2,
    pub vertex2: Point2,
    pub vertex3: Point2,
    pub bounds_v1_v2: Option<Vec<Point2>>,
    pub bounds_v2_v3: Option<Vec<Point2>>,
    pub bounds_v3_v1: Option<Vec<Point2>>
}
impl Triangle{
    pub fn new(v1: Point2, v2: Point2, v3: Point2) -> Triangle{
        let p1_p2 = determine_bounds(&v1, &v2);
        let p2_p3 = determine_bounds(&v2, &v3);
        let p3_p1 = determine_bounds(&v3, &v1);
        return Triangle{vertex1: v1, vertex2: v2, vertex3: v3, bounds_v1_v2: Some(p1_p2), bounds_v2_v3: Some(p2_p3), bounds_v3_v1: Some(p3_p1)};
    }

    pub fn draw(self, buffer: &mut Vec<u8>, width: usize, height: usize, fill: bool){
        rasterize(buffer, width, height, self, fill);
    }
}
