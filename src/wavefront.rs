use std::{io::{self, BufRead}, fs::File};

use crate::point::Vec3f;

pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Face {
    pub vertices: Vec<usize>,
}

pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Face>,
}

pub fn read_obj_file(file_path: &str) -> Result<Model, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut vertices: Vec<Vec3f> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

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
                }
                "f" => {
                    let mut face = Face {
                        vertices: Vec::new(),
                    };

                    for i in 1..tokens.len() {
                        let indices: Vec<&str> = tokens[i].split('/').collect();
                        let index: usize = indices[0].parse().unwrap();
                        face.vertices.push(index - 1);
                    }
                    faces.push(face);
                }
                _ => (),
            }
        }
    }

    Ok(Model {vertices, faces})
}
