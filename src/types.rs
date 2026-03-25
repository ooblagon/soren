
struct TPosition {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Color{
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8
}

#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
    pub color: Color
}
pub struct Camera {
    position: TPosition,
    perspective: TPosition,
}
impl Camera {
    pub fn new(position: [i32; 3], perspective: [i32; 3]) -> Camera {
        Camera {
            position: TPosition {
                x: position[0],
                y: position[1],
                z: position[2],
            },
            perspective: TPosition {
                x: position[0],
                y: position[1],
                z: position[2],
            },
        }
    }
}