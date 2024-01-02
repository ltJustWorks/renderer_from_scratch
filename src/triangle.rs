use std::{mem::swap, f32::consts::E};

use tgaimage::{TGAImage, TGAColor, TGAColorRGB};

use crate::{point::{Point2D, Vec3f, barycentric, Vec2f}, line, wavefront::{interpolate_tex_coord, sample_texture}};

pub fn draw(image: &mut TGAImage, zbuffer: &mut [f32], texture: &TGAImage, tex_coords: [&Vec2f; 3], world_coords: [&Vec3f; 3], pts: &[Vec3f; 3], light_dir: &Vec3f, normals: [&Vec3f; 3]) {
    let intensities = calculate_vertex_intensities(world_coords, light_dir, &normals);

    for intensity in intensities {
        if intensity <= 0.0 {return;}
    }

    let mut bboxmin = Point2D {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };
    let mut bboxmax = Point2D { x: 0, y: 0 };
    let clamp = Point2D {
        x: (image.width() - 1) as i32,
        y: (image.height() - 1) as i32,
    };

    for i in 0..3 {
        bboxmin.x = std::cmp::max(0, std::cmp::min(bboxmin.x, pts[i].x as i32));
        bboxmin.y = std::cmp::max(0, std::cmp::min(bboxmin.y, pts[i].y as i32));

        bboxmax.x = std::cmp::min(clamp.x, std::cmp::max(bboxmax.x, pts[i].x as i32));
        bboxmax.y = std::cmp::min(clamp.y, std::cmp::max(bboxmax.y, pts[i].y as i32));
    }

    
    let mut p = Vec3f { x: 0.0, y: 0.0, z: 0.0 };

    for i in (bboxmin.x as usize)..=(bboxmax.x as usize) {
        p.x = i as f32;
        for j in (bboxmin.y as usize)..=(bboxmax.y as usize) {
            p.y = j as f32;
            let bc_screen = barycentric(&pts, &p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            p.z = 0.0;
            for i in 0..3 {
                p.z += pts[i].z * bc_screen[i];
            }

            let index = (p.x + p.y * image.width() as f32) as usize;
            if zbuffer[index] < p.z {
                let interpolated_tex_coord = interpolate_tex_coord(tex_coords, &bc_screen);
                let interpolated_intensity = interpolate_intensity(&intensities, &bc_screen);
                let color = sample_texture(texture, &interpolated_tex_coord, interpolated_intensity);

                zbuffer[index] = p.z;
                image.set(p.x as usize, p.y as usize, &color);
            }
        }
    }
}

/*
fn calculate_intensity(vertex: &Vec3f, normal: &Vec3f, light_dir: &Vec3f) -> f32 {
    let normalized_light_dir = light_dir;
    let normalized_normal = normal;

    let dir_to_vertex = Vec3f{
        x: vertex.x - light_dir.x, 
        y: vertex.y - light_dir.y,
        z: vertex.z - light_dir.z - 3.0, // TODO: fix
    }.normalize();

    let intensity = normalized_light_dir.dot_product(&normalized_normal) * dir_to_vertex.dot_product(&normalized_normal);
    intensity.max(0.0)
}
 */

fn calculate_normal(pts: [&Vec3f; 3]) -> Vec3f {
    let edge1 = pts[1].subtract(&pts[0]);
    let edge2 = pts[2].subtract(&pts[0]);

    // Calculate the cross product of edge1 and edge2 to get the normal vector
    let normal = edge1.cross_product(&edge2);

    // Return the normalized normal vector
    normal.normalize()
}

fn calculate_vertex_intensities(triangle_pts: [&Vec3f; 3], light_dir: &Vec3f, normals: &[&Vec3f; 3]) -> [f32; 3] {
    let mut pt_intensities = [f32::default(); 3];
    for i in 0..3 {
        let pt = &triangle_pts[i];
        let to_light_dir = (light_dir.subtract(pt)).normalize(); 
        let cos_angle = (normals[i].dot_product(&to_light_dir)).max(0.0);
        pt_intensities[i] = cos_angle
    }
    pt_intensities
}

fn interpolate_intensity(intensities: &[f32; 3], bc_screen: &Vec3f) -> f32 {
    let mut intensity = 0.0;
    for i in 0..3 {
        intensity += intensities[i] * bc_screen[i];
    }
    intensity
}