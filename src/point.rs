use std::collections::VecDeque;
use std::ops::Index;
use ndarray::{Array2};

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

impl Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3f"),
        }
    }
}

pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

pub fn barycentric(pts: &[Vec3f; 3], p: &Vec3f) -> Vec3f {
    let u = Vec3f {
        x: (pts[2].x - pts[0].x) as f32,
        y: (pts[1].x - pts[0].x) as f32,
        z: (pts[0].x - p.x) as f32,
    } 
    .cross_product(&Vec3f {
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

impl Vec3f {
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {Self{x:self.x,y:self.y,z:self.z}}
    }
}

pub fn lookat(eye: &Vec3f, center: &Vec3f, up: &Vec3f) -> Array2<f32> {
    let z = (eye.subtract(&center)).normalize();
    let x = (up.cross_product(&z)).normalize();
    let y = (&z.cross_product(&x)).normalize();

    let mut Minv = Array2::eye(4);
    let Tr = Array2::eye(4);

    for i in 0..3 {
        Minv[[0, i]] = x[i] as f32;
    }

    Minv.dot(&Tr)
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32, depth: i32) -> Array2<f32> {
    let mut m = Array2::eye(4);
    m[[0,3]] = (x as f32) + (w as f32)/2.0;
    m[[1,3]] = (y as f32) + (h as f32)/2.0;
    m[[0,3]] = (depth as f32)/2.0;

    m[[0,0]] = (w as f32)/2.0;
    m[[1,1]] = (h as f32)/2.0;
    m[[2,2]] = (depth as f32)/2.0;

    m
}