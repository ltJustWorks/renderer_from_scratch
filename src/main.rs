use minifb::{Window, WindowOptions};
use tgaimage::{TGAImage, TGAColor};

mod image_processing;
mod line;

fn main() {
    let white = TGAColor::rgba(255, 255, 255, 255);
    let red = TGAColor::rgba(255, 0, 0, 255);

    // TODO: Fix window w/h issue
    let width = 100;
    let height = 100;

    let mut image = TGAImage::new(width, height, 4);
    // do stuff here
    line::draw(&mut image, &white, 13, 20, 80, 40);
    line::draw(&mut image, &red, 20, 13, 40, 80);
    line::draw(&mut image, &red, 80, 40, 13, 20);

    image.flip_vertically();
    //image.write_tga_file("output.tga", false);

    // Create a window matching the image dimensions
    let mut window = Window::new("View", width, height, WindowOptions::default()).expect("unable to open window");

    let mut buffer: Vec<u32> = image_processing::convert_buf(&image);

    // Display the image in the window
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height);
    }
}