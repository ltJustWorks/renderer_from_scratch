use minifb::{Window, WindowOptions};
use point::{Point2D, Vec3f, Vec2f};
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

fn world_to_screen(v: &Vec3f, width: usize, height: usize) -> Vec3f {
    Vec3f {
        x: (v.x + 1.0)*(width as f32)/2.0 + 0.5,
        y: (v.y + 1.0)*(height as f32)/2.0 + 0.5,
        z: v.z,
    }
}

fn draw_model(width: usize, height: usize, image: &mut TGAImage) {
    let model = wavefront::read_obj_file("src/obj/african_head.obj").unwrap();
    let mut texture = TGAImage::from_tga_file("src/textures/african_head_diffuse.tga");
    texture.flip_vertically();

    let mut rng = rand::thread_rng();
    let light_dir = Vec3f {x:0.0, y:0.0, z:1.0};
    let mut zbuffer = vec![0.0; width*height];

    for i in 0..model.faces.len() {
        let face = &model.faces[i];

        let mut world_coords = [&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0}];
        let mut screen_coords = [Vec3f {x: 0.0, y: 0.0, z: 0.0},Vec3f {x: 0.0, y: 0.0, z: 0.0},Vec3f {x: 0.0, y: 0.0, z: 0.0}];
        let mut tex_coords = [
            &Vec2f {x:0.0,y:0.0},
            &Vec2f {x:0.0,y:0.0},
            &Vec2f {x:0.0,y:0.0},
        ];

        for j in 0..3 {
            world_coords[j] = &model.vertices[face.vertices[j]-1];
            screen_coords[j] = world_to_screen(&world_coords[j], width, height);
            tex_coords[j] = &model.textures[face.textures[j]-1];
        }

        if true /*fix this */ {
            triangle::draw(image, &mut zbuffer, &texture, tex_coords, world_coords, &screen_coords, &light_dir);
        }
    }
}

