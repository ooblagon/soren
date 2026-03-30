use std::collections::HashMap;

use crate::triangle;
use crate::types::*;
use crate::triangle::*;


pub fn set_pixel(buffer: &mut [u8], width: usize, height: usize, point: &Point2) {
    let (x, y)= screen_fixing(width, height, point).unwrap();
    let i = (y as usize * width + x as usize) * 4;
    buffer[i] = point.color.b.to_owned();
    buffer[i + 1] = point.color.g;
    buffer[i + 2] = point.color.r;
    buffer[i + 3] = point.color.a;
}
pub fn screen_fixing(screen_width: usize, screen_height: usize, point: &Point2) -> Option<(i32, i32)> {
    let sx = point.x + (screen_width as i32 / 2);
    let sy = (screen_height as i32 / 2) - point.y;
    if sx < 0 || sy < 0 || sx >= screen_width as i32 || sy >= screen_height as i32 {
        None
    } else {
        Some((sx, sy))
    }
}
//name might be changed later
pub fn determine_bounds(point1: &Point2, point2: &Point2) -> Vec<Point2>{
    let mut x = point1.x;
    let mut y = point1.y;
    let dx = (point2.x - point1.x).abs();
    let dy = (point2.y - point1.y).abs();
    let sx = if point1.x < point2.x {1} else {-1};
    let sy = if point1.y < point2.y {1} else {-1};
    let mut err = dx - dy;
    let mut points: Vec<Point2> = Vec::new();

    loop{
        let point = Point2{x: x, y: y, color: Color { b: point1.color.b, g: point1.color.g, r: point1.color.r, a: point1.color.a }};
        points.push(point);
        if x == point2.x && y == point2.y{
            return points;
        }

        let e2 = err * 2;

        if e2 > -dy{
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
pub fn rasterize(buffer: &mut [u8], width: usize, height: usize, triangle: &Triangle, fill: bool){
    if fill{

        let min_y = triangle.vertex1.y.min(triangle.vertex2.y).min(triangle.vertex3.y);
        let max_y = triangle.vertex1.y.max(triangle.vertex2.y).max(triangle.vertex3.y);

        for y in min_y..=max_y{
            if let Some(points) = triangle.edge_map.get(&y){
                let min_x = points.iter().map(|p| p.x).min().unwrap();
                let max_x = points.iter().map(|p| p.x).max().unwrap();

                for x in min_x..=max_x{
                    let point = Point2 {x, y, color: triangle.vertex1.color};
                    set_pixel(buffer,width, height, &point);
                }
            }
        }
    } else {
        for point in triangle.bounds_v1_v2.as_ref().unwrap(){
            set_pixel(buffer, width, height, &point);
        }
        for point in triangle.bounds_v2_v3.as_ref().unwrap(){
            set_pixel(buffer, width, height, &point);
        }
        for point in triangle.bounds_v3_v1.as_ref().unwrap(){
            set_pixel(buffer, width, height, &point);
        }
    }

}
pub fn rasterize_polygon(buffer: &mut [u8], width: usize, height: usize, vertices: &[Point2], fill: bool){
    for i in 1..vertices.len() -1 {
        let tri = Triangle::new(vertices[0], vertices[i], vertices[i+1]);
        rasterize(buffer, width, height, &tri, fill);
    }

}
