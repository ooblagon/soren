use crate::types::*;
use crate::triangle::*;


pub fn set_pixel(buffer: &mut Vec<u8>, width: usize, height: usize, point: &Point2) {
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
pub fn rasterize(buffer: &mut Vec<u8>, width: usize, height: usize, triangle: Triangle, fill: bool){
    if fill{
        for point1 in triangle.bounds_v1_v2.unwrap(){
            for point2 in triangle.bounds_v3_v1.clone().unwrap(){
                let points = determine_bounds(&point1, &point2);
                for point in points{
                    set_pixel(buffer, width, height, &point);
                }
            }
        }

    } else {
        for point in triangle.bounds_v1_v2.unwrap(){
            set_pixel(buffer, width, height, &point);
        }
        for point in triangle.bounds_v2_v3.unwrap(){
            set_pixel(buffer, width, height, &point);
        }
        for point in triangle.bounds_v3_v1.unwrap(){
            set_pixel(buffer, width, height, &point);
        }
    }

}
