use minifb::{Window, WindowOptions};
use tgaimage::{TGAImage, TGAColor};

fn convert_buf(image: &TGAImage) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    buffer
}

fn main() {
    let white = TGAColor::rgba(255, 255, 255, 255);
    let red = TGAColor::rgba(255, 0, 0, 255);

    let width = 100;
    let height = 100;

    let mut image = TGAImage::new(100, 100, 4);
    image.set(52, 41, &red);
    image.flip_vertically();
    image.write_tga_file("output.tga", false);

    /*
    // Create a window matching the image dimensions
    let mut window = Window::new("View", width, height, WindowOptions::default()).expect("unable to open window");

    let mut buffer: Vec<u32> = convert_buf(&image);

    // Display the image in the window
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height);
    }
    */
}