use crate::triangle::Triangle;
use crate::types::*;
use crate::rasterizer::*;


pub struct Rectangle{
    pub vertex1: Point2,
    pub vertex2: Point2,
    pub vertex3: Point2,
    pub vertex4: Point2,
}
impl Rectangle{
    pub fn new(v1: Point2, v2: Point2, v3: Point2, v4: Point2) -> Rectangle{
        let tri1 = Triangle::new(v1, v2, v3);
        let tri2 = Triangle::new(v2, v3, v4);
        Rectangle { vertex1: v1, vertex2: v2, vertex3: v3, vertex4: v4 }
    }
    pub fn draw(&self, buffer: &mut [u8], width: usize, height: usize, fill: bool){
        let vertices = &[self.vertex1, self.vertex2, self.vertex3, self.vertex4];
        rasterize_polygon(buffer, width, height, vertices, fill);
    }
}