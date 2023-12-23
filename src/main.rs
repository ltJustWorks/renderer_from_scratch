use minifb::{Window, WindowOptions};
use tgaimage::{TGAImage, TGAColor, TGAColorRGBA};

fn convert_buf(image: &TGAImage) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let rgba: TGAColorRGBA = match image.get(x, y) {
                TGAColor::Rgb(rgb) => TGAColorRGBA{r: rgb.r, g: rgb.g, b: rgb.b, a: 255},
                TGAColor::Rgba(rgba) => rgba,
            };
            let pixel_value = ((rgba.a as u32) << 24) | ((rgba.r as u32) << 16) | ((rgba.g as u32) << 8) | (rgba.b as u32);
            buffer.push(pixel_value);
        }
    }

    buffer
}

fn main() {
    let white = TGAColor::rgba(255, 255, 255, 255);
    let red = TGAColor::rgba(255, 0, 0, 255);

    // TODO: Fix window w/h issue
    let width = 100;
    let height = 100;

    let mut image = TGAImage::new(100, 100, 4);
    image.set(52, 41, &red);
    image.flip_vertically();
    image.write_tga_file("output.tga", false);

    // Create a window matching the image dimensions
    let mut window = Window::new("View", width, height, WindowOptions::default()).expect("unable to open window");

    let mut buffer: Vec<u32> = convert_buf(&image);

    // Display the image in the window
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height);
    }
}