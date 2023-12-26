use std::collections::VecDeque;

pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self {
        Point2D {
            x, y
        }
    }
}

pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn barycentric(pts: &[Point2D; 3], p: &Point2D) -> Vec3f {
    let v0 = Point2D::new(
        pts[1].x - pts[0].x,
        pts[1].y - pts[0].y,
    );
    let v1 = Point2D::new(
        pts[2].x - pts[0].x,
        pts[2].y - pts[0].y,
    );
    let v2 = Point2D::new(
        p.x - pts[0].x,
        p.y - pts[0].y,
    );

    let d00 = v0.x * v0.x + v0.y * v0.y;
    let d01 = v0.x * v1.x + v0.y * v1.y;
    let d11 = v1.x * v1.x + v1.y * v1.y;
    let d20 = v2.x * v0.x + v2.y * v0.y;
    let d21 = v2.x * v1.x + v2.y * v1.y;

    let denom = d00 * d11 - d01 * d01;

    let v = (d11 * d20 - d01 * d21) as f32 / denom as f32;
    let w = (d00 * d21 - d01 * d20) as f32 / denom as f32;
    let u = 1.0 - v - w;

    if denom.abs() < 1 {return Vec3f {x:-1.0, y:1.0, z:1.0};}
    Vec3f { x: u, y: v, z: w }
}