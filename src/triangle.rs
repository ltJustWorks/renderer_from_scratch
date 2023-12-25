use std::{mem::swap, f32::consts::E};

use tgaimage::{TGAImage, TGAColor};

use crate::{point::Point2D, line};

pub fn draw(image: &mut TGAImage, color: &TGAColor, t0: &Point2D, t1: &Point2D, t2: &Point2D) {
    // sort points
    let mut t_min = &t0;
    let mut t_mid = &t1;
    let mut t_max = &t2;
    if t_min.y > t_mid.y {swap(&mut t_min, &mut t_mid)}
    if t_min.y > t_max.y {swap(&mut t_min, &mut t_max)}
    if t_mid.y > t_max.y {swap(&mut t_mid, &mut t_max)}

    let height = t_max.y - t_min.y;

    for y in 0..height {
        let in_second_half: bool = y > (t_mid.y - t_min.y);
        let segment_height = if in_second_half {
            t_max.y - t_mid.y
        } else {
            t_mid.y - t_min.y
        };

        let a = y as f32 / height as f32;
        let b = if in_second_half {
            (y - (t_mid.y - t_min.y)) as f32 / segment_height as f32
        } else {
            y as f32 / segment_height as f32
        } ;

        let mut a_x = t_min.x + (a*((t_max.x - t_min.x) as f32)) as i32;
        let mut b_x = if in_second_half {
            t_mid.x + (b*((t_max.x - t_mid.x) as f32)) as i32
        } else {
            t_min.x + (b*((t_mid.x - t_min.x) as f32)) as i32
        };

        if a_x > b_x {std::mem::swap(&mut a_x, &mut b_x);}

        for x in a_x..=b_x {
            image.set(x as usize, (y + t_min.y) as usize, color);
        }
    }
}