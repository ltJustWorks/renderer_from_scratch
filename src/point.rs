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
    let u = Vec3f {
        x: (pts[2].x - pts[0].x) as f32,
        y: (pts[1].x - pts[0].x) as f32,
        z: (pts[0].x - p.x) as f32,
    } 
    .cross_product(Vec3f {
        x: (pts[2].y - pts[0].y) as f32,
        y: (pts[1].y - pts[0].y) as f32,
        z: (pts[0].y - p.y) as f32,
    });

    if u.z.abs() < 1.0 {
        return Vec3f { x: -1.0, y: 1.0, z: 1.0 };
    }

    Vec3f {
        x: 1.0 - (u.x + u.y) / u.z,
        y: u.y / u.z,
        z: u.x / u.z,
    }
}

trait CrossProduct {
    fn cross_product(self, other: Self) -> Self;
}

impl CrossProduct for Vec3f {
    fn cross_product(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}