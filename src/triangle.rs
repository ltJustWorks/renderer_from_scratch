use tgaimage::{TGAImage, TGAColor};

use crate::{point::Point2D, line};

pub fn draw(image: &mut TGAImage, color: &TGAColor, t0: &Point2D, t1: &Point2D, t2: &Point2D) {
    line::draw_from_points(image, color, t0, t1);
    line::draw_from_points(image, color, t1, t2);
    line::draw_from_points(image, color, t2, t0);
}