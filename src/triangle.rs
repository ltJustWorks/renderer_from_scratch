use std::mem::swap;

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

    for y in t_min.y..=t_mid.y {
        let segment_height = t_mid.y - t_min.y + 1;
        let a = (y - t_min.y) as f32 / height as f32;
        let b = (y - t_min.y) as f32 / segment_height as f32;

        let a_x = t_min.x + (a*((t_max.x - t_min.x) as f32)) as i32;
        let b_x = t_min.x + (b*((t_mid.x - t_min.x) as f32)) as i32;

        image.set(a_x as usize, y as usize, color);
        image.set(b_x as usize, y as usize, color);
    }
}