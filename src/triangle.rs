use std::{mem::swap, f32::consts::E};

use tgaimage::{TGAImage, TGAColor};

use crate::{point::{Point2D, Vec3f, barycentric}, line};

pub fn draw(image: &mut TGAImage, color: &TGAColor, pts: &[Point2D; 3]) {
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
        bboxmin.x = std::cmp::max(0, std::cmp::min(bboxmin.x, pts[i].x));
        bboxmin.y = std::cmp::max(0, std::cmp::min(bboxmin.y, pts[i].y));

        bboxmax.x = std::cmp::min(clamp.x, std::cmp::max(bboxmax.x, pts[i].x));
        bboxmax.y = std::cmp::min(clamp.y, std::cmp::max(bboxmax.y, pts[i].y));
    }

    println!("{} {} {} {}", bboxmin.x, bboxmin.y, bboxmax.x, bboxmax.y);

    let mut p = Point2D { x: 0, y: 0 };

    for x in bboxmin.x..=bboxmax.x {
        for y in bboxmin.y..=bboxmax.y {
            p.x = x;
            p.y = y;
            let bc_screen = barycentric(&pts, &p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(x as usize, y as usize, color);
        }
    }
}