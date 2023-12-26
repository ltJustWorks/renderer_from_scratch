use minifb::{Window, WindowOptions};
use point::Point2D;
use tgaimage::{TGAImage, TGAColor};

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
    let width = 200;
    let height = 200;

    let mut image = TGAImage::new(width, height, 4);
    // do stuff here

    //draw_model(width, height, &mut image, &white);
    
    let t = [Point2D::new(10, 10),   Point2D::new(100, 30),  Point2D::new(190, 160)]; 
    triangle::draw(&mut image, &red, &t); 

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

fn draw_model(width: usize, height: usize, image: &mut TGAImage, color: &TGAColor) {
    let model = wavefront::read_obj_file("src/obj/african_head.obj").unwrap();

    for i in 0..model.faces.len() {
        let face = &model.faces[i];

        for j in 0..3 {
            let v0 = &model.vertices[face.vertices[j]];
            let v1 = &model.vertices[face.vertices[(j + 1) % 3]];

            let x0 = ((v0.x + 1.) * width as f32 / 2.) as i32;
            let y0 = ((v0.y + 1.) * height as f32 / 2.) as i32;
            let x1 = ((v1.x + 1.) * width as f32 / 2.) as i32;
            let y1 = ((v1.y + 1.) * height as f32 / 2.) as i32;

            line::draw(image, color, x0, x1, y0, y1);
        }
    }
}