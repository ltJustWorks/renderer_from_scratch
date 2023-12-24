use tgaimage::{TGAImage, TGAColor};

pub fn draw(image: &mut TGAImage, color: &TGAColor, x0: i32, x1: i32, y0: i32, y1: i32) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let x_inc = if x1 >= x0 { 1 } else { -1 };
    let y_inc = if y1 >= y0 { 1 } else { -1 };
    let mut error = if dx > dy { dx } else { -dy } / 2;
    let mut current_error;

    while x != x1 || y != y1 {
        image.set(x as usize, y as usize, color); // Set pixel with the provided color

        current_error = error;
        if current_error > -dx {
            error -= dy;
            x += x_inc;
        }
        if current_error < dy {
            error += dx;
            y += y_inc;
        }
    }
}