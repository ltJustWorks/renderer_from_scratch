use minifb::{Window, WindowOptions};
use ndarray::{array, Array, arr1, Array2};
use point::{Point2D, Vec3f, Vec2f, lookat, viewport};
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
    let depth = 255;

    let mut image = TGAImage::new(width, height, 4);
    // do stuff here

    draw_model(width, height, depth, &mut image);
    
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

fn draw_model(width: usize, height: usize, depth: usize, image: &mut TGAImage) {
    let model = wavefront::read_obj_file("src/obj/african_head.obj").unwrap();
    let mut texture = TGAImage::from_tga_file("src/textures/african_head_diffuse.tga");
    texture.flip_vertically();

    let mut rng = rand::thread_rng();
    let light_dir = Vec3f {x:0.0, y:0.0, z:1.0};
    let mut zbuffer = vec![f32::MIN; width*height];

    let eye = Vec3f {x: 1.0, y: 1.0, z: 3.0};
    let center = Vec3f {x: 0.0, y: 0.0, z: 0.0};
    let ModelView = lookat(&eye, &center, &Vec3f { x: 0.0, y: 1.0, z: 0.0 });
    let mut Projection = Array2::eye(4);
    Projection[[3,2]] = -1.0 / (eye.subtract(&center)).length();
    let ViewPort = viewport((width/8) as i32, (height/8) as i32, (width*3/4) as i32, (height*3/4) as i32, depth as i32);
    println!("{}", ViewPort);

    for i in 0..model.faces.len() {
        let face = &model.faces[i];

        let mut world_coords = [&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0}];
        let mut screen_coords = [Vec3f {x: 0.0, y: 0.0, z: 0.0},Vec3f {x: 0.0, y: 0.0, z: 0.0},Vec3f {x: 0.0, y: 0.0, z: 0.0}];
        let mut tex_coords = [
            &Vec2f {x:0.0,y:0.0},
            &Vec2f {x:0.0,y:0.0},
            &Vec2f {x:0.0,y:0.0},
        ];
        let mut normals = [&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0},&Vec3f {x: 0.0, y: 0.0, z: 0.0}];



        for j in 0..3 {
            let v = &model.vertices[face.vertices[j]-1];

            //let mut v_mat = Array::from_diag(&arr1(&[v.x, v.y, v.z]));
            let mut v_mat = Array2::eye(4);
            v_mat[[0, 0]] = v.x;
            v_mat[[1, 1]] = v.y;
            v_mat[[2, 2]] = v.z;
            v_mat = ViewPort.dot(&Projection).dot(&ModelView).dot(&v_mat);

            world_coords[j] = v;
            screen_coords[j] = Vec3f {
                x: v_mat[[0, 0]],
                y: v_mat[[1, 1]],
                z: v_mat[[2, 2]],
            };
            /*
            screen_coords[j]= world_to_screen(v, width, height);
            */
            println!("screen coords: {} {}", screen_coords[j].x, screen_coords[j].y);
            tex_coords[j] = &model.textures[face.textures[j]-1];
            normals[j] = &model.normals[face.normals[j]-1];
        }

        if true /*fix this */ {
            triangle::draw(image, &mut zbuffer, &texture, tex_coords, world_coords, &screen_coords, &light_dir, normals);
        }
    }
}

