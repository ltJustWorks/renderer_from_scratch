use std::{io::{self, BufRead}, fs::File};

use tgaimage::{TGAImage, TGAColor};

use crate::point::{Vec3f, Vec2f};

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Face {
    pub vertices: Vec<usize>,
    pub textures: Vec<usize>,
}

pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Face>,
    pub textures: Vec<Vec2f>,
}

pub fn read_obj_file(file_path: &str) -> Result<Model, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut vertices: Vec<Vec3f> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();
    let mut textures: Vec<Vec2f> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let tokens: Vec<&str> = line.split_whitespace().collect();

            if tokens.is_empty() {
                println!("empty line");
                continue;
            }

            match tokens[0] {
                "v" => {
                    let x: f32 = tokens[1].parse().unwrap();
                    let y: f32 = tokens[2].parse().unwrap();
                    let z: f32 = tokens[3].parse().unwrap();
                    vertices.push(Vec3f { x, y, z });
                },
                "vt" => {
                    let x: f32 = tokens[1].parse().unwrap();
                    let y: f32 = tokens[2].parse().unwrap();
                    textures.push(Vec2f { x, y });
                },
                "f" => {
                    let mut face = Face {
                        vertices: Vec::new(),
                        textures: Vec::new(),
                    };

                    for i in 1..tokens.len() {
                        let indices: Vec<&str> = tokens[i].split('/').collect();
                        face.vertices.push(indices[0].parse().unwrap());
                        if indices.len() > 1 && !indices[1].is_empty() {
                            face.textures.push(indices[1].parse().unwrap());
                        }
                    }
                    faces.push(face);
                }
                _ => (),
            }
        }
    }

    Ok(Model {vertices, faces, textures})
}

pub fn interpolate_tex_coord(tex_coords: [&Vec2f; 3], barycentric: &Vec3f) -> Vec2f {
    let mut tex_coord = Vec2f {x: 0.0, y: 0.0};
    for i in 0..3 {
        tex_coord.x += tex_coords[i].x * barycentric[i];
        tex_coord.y += tex_coords[i].y * barycentric[i];
    }
    tex_coord
} 

pub fn sample_texture(texture: &TGAImage, coord: &Vec2f, intensity: f32) -> TGAColor {
    println!("{}", intensity);
    // Convert texture coordinates to pixel coordinates
    let width = texture.width();
    let height = texture.height();
    let x = (coord.x * (width as f32 - 1.0)).round() as usize;
    let y = (coord.y * (height as f32 - 1.0)).round() as usize;

    // Ensure the coordinates are within bounds
    let x = x.clamp(0, width - 1);
    let y = y.clamp(0, height - 1);

    match texture.get(x, y) {
        TGAColor::Rgba(rgba) => TGAColor::rgba(((rgba.r as f32)*intensity) as u8, ((rgba.g as f32)*intensity) as u8, ((rgba.b as f32)*intensity) as u8, 255),
        TGAColor::Rgb(rgb) => TGAColor::rgba(((rgb.r as f32)*intensity) as u8, ((rgb.g as f32)*intensity) as u8, ((rgb.b as f32)*intensity) as u8, 255),
    }
}