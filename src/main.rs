use minifb::{Window, WindowOptions};
use point::Point2D;
use tgaimage::{TGAImage, TGAColor, TGAColorRGBA};
use rand::Rng;

mod image_processing;
mod line;
mod triangle;
mod point;
mod wavefront;

fn main() {
    let white = TGAColor::rgba(255, 255, 255, 255);
    let red = TGAColor::rgba(255, 0, 0, 255);
    let green = TGAColor::rgba(0, 255, 0, 255);

    // TODO: Fix window w/h issue
    let width = 600;
    let height = 600;

    let mut image = TGAImage::new(width, height, 4);
    // do stuff here

    draw_model(width, height, &mut image);
    
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

fn draw_model(width: usize, height: usize, image: &mut TGAImage) {
    let model = wavefront::read_obj_file("src/obj/african_head.obj").unwrap();
    let mut rng = rand::thread_rng();

    for i in 0..model.faces.len() {
        let face = &model.faces[i];

        let mut screen_coords = [Point2D { x: 0, y: 0 }, Point2D { x: 0, y: 0 }, Point2D { x: 0, y: 0 }];

        for j in 0..3 {
            let world_coords = &model.vertices[face.vertices[j]]; // Assuming model has a method vert(i) that returns Vec3f
            let x = ((world_coords.x + 1.0) * width as f32 / 2.0) as i32; // Assuming width and height are known
            let y = ((world_coords.y + 1.0) * height as f32 / 2.0) as i32; // and represent the screen size

            screen_coords[j] = Point2D { x, y };
        }

        let color = TGAColor::rgba(
            rand::thread_rng().gen_range(0..=255),
            rand::thread_rng().gen_range(0..=255),
            rand::thread_rng().gen_range(0..=255),
            255, // Assuming alpha is 255 for opaque color
        );

        triangle::draw(image, &color, &screen_coords);

    }
}