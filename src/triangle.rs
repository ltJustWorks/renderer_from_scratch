use std::{mem::swap, f32::consts::E};

use tgaimage::{TGAImage, TGAColor};

use crate::{point::{Point2D, Vec3f, barycentric}, line};

pub fn draw(image: &mut TGAImage, zbuffer: &mut [f32], color: &TGAColor, pts: &[Vec3f; 3]) {
    let mut bboxmin = Point2D {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };
    let mut bboxmax = Point2D { x: 0, y: 0 };
    let clamp = Point2D {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };

    for i in 0..3 {
        bboxmin.x = std::cmp::max(0, std::cmp::min(bboxmin.x, pts[i].x as i32));
        bboxmin.y = std::cmp::max(0, std::cmp::min(bboxmin.y, pts[i].y as i32));

        bboxmax.x = std::cmp::min(clamp.x, std::cmp::max(bboxmax.x, pts[i].x as i32));
        bboxmax.y = std::cmp::min(clamp.y, std::cmp::max(bboxmax.y, pts[i].y as i32));
    }

    println!("{} {} {} {}", bboxmin.x, bboxmin.y, bboxmax.x, bboxmax.y);

    let mut p = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

    for i in (bboxmin.x as usize)..=(bboxmax.x as usize) {
        p.x = i as f32;
        for j in (bboxmin.y as usize)..=(bboxmax.y as usize) {
            p.y = j as f32;
            let bc_screen = barycentric(&pts, &p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            p.z = 0.0;
            for i in 0..3 {
                p.z += pts[i].z * bc_screen[i];
            }

            let index = (p.x + p.y * image.width() as f32) as usize;
            if zbuffer[index] < p.z {
                zbuffer[index] = p.z;
                image.set(p.x as usize, p.y as usize, color);
            }
        }
    }
}