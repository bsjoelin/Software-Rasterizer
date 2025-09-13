use rand::{rng, rngs::ThreadRng, Rng};
use std::fs::{read_to_string, write};
use std::io;
use std::iter::zip;

mod formats;
mod objects;
mod rendering;
mod vector_math;

use crate::objects::Model;
use crate::rendering::bitmap::image_to_bmp_buffer;
use crate::rendering::RenderTarget;
use crate::rendering::pipeline;
use crate::rendering::transforms::Transform;
use crate::vector_math::triangle::*;
use crate::vector_math::vector::*;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

struct Scene {
    vertices: Vec<Float2>,
    vertex_velocities: Vec<Float2>,
    triangle_colors: Vec<Float3>
}

fn main() {
    let mut model = load_suzanne_model();
    let mut render_target = RenderTarget::new(WIDTH, HEIGHT);
    let fov = 60.0;

    for i in 0..20 {
        pipeline::render3d(&model, &mut render_target, fov);
    
        // Save the current stage of the image buffer to a bitmap
        let file_name = format!("images/monkey_frame_{:03}.bmp", i);
        match write_image_to_file(&render_target.image_buffer, file_name.to_string()) {
            Err(why) => panic!("Failed to write frame {} to file {}: {}", 0, file_name, why),
            Ok(_) => (),
        };

        model.transform.yaw += 0.1;
        model.transform.pitch += 0.02;
        render_target.clear();
    }
}

#[allow(dead_code)]
fn old_main() {
    let mut scene = create_test_images();
    let mut render_target = RenderTarget::new(WIDTH, HEIGHT);

    for i in 0..5 {
        pipeline::render2d(&scene.vertices, &scene.triangle_colors, &mut render_target);
    
        // Save the current stage of the image buffer to a bitmap
        let file_name = format!("images/test_frame_{:03}.bmp", i);
        match write_image_to_file(&render_target.image_buffer, file_name.to_string()) {
            Err(why) => panic!("Failed to write frame {} to file {}: {}", 0, file_name, why),
            Ok(_) => (),
        };

        update(&mut scene.vertices, &mut scene.vertex_velocities, 0.25);
        render_target.clear();
    }
}

fn update(vertices: &mut Vec<Float2>, velocities: &mut Vec<Float2>, delta_t: f64) {
    for (vert, vel) in zip(vertices, velocities) {
        *vert += &*vel * delta_t;
        // Flip the velocities, if the points end up outside the render box
        if vert.x < 0f64 || vert.x > WIDTH as f64{
            vel.x *= -1f64;
        }
        if vert.y < 0f64 || vert.y > HEIGHT as f64 {
            vel.y *= -1f64;
        }
    }
}

#[allow(dead_code)]
fn load_cube_model() -> Model {
    let mut model = load_model("models/cube.obj");
    model.transform.position += Float3::new(0.0, 0.0, 5.0);
    model
}

fn load_suzanne_model() -> Model {
    let mut model = load_model("models/suzanne.obj");
    model.transform.position += Float3::new(0.0, 0.0, 3.0);
    model
}

fn load_model(obj_file: &str) -> Model {
    let model_vertices = match read_to_string(obj_file) {
        Ok(obj_str) => crate::formats::obj_format::load_obj_file(obj_str),
        Err(why) => match why.kind() {
            io::ErrorKind::NotFound => panic!("The path {} is non-existent! Make sure the folder structure exists.", obj_file),
            io::ErrorKind::PermissionDenied => panic!("You don't have permissions to write to file \"{}\"", obj_file),
            _ => panic!("Failed to open file {}: {}", obj_file, why),
        }
    };

    // Randomize the triangle colors
    let mut g = rng();
    let mut triangle_colors: Vec<Float3> = Vec::new();
    for _ in 0..(model_vertices.len() / 3) {
        triangle_colors.push(random_color(&mut g))
    }

    Model { 
        vertices: model_vertices, 
        triangle_colors, 
        transform: Transform::empty()
    }
}

#[allow(dead_code)]
/// Generate randomly initialized triangles
fn create_test_images() -> Scene {
    // Get the random vertices, triangle velocities and colors
    let (points, velocities, triangle_colors) = setup_triangles(WIDTH, HEIGHT);

    Scene { vertices: points, vertex_velocities: velocities, triangle_colors: triangle_colors }
}

/// Initialize triangles with random positions, velocities and colors. Returns the flattened vertices vector, the velocities and the triangle colors.
fn setup_triangles(screen_width: usize, screen_height: usize) -> (Vec<Float2>, Vec<Float2>, Vec<Float3>) {
    const TRIANGLE_COUNT: usize = 50;

    // Initialize data arrays with 0-vectors
    let mut points = vec![Float2::zeros(); TRIANGLE_COUNT * 3];
    let mut velocities = vec![Float2::zeros(); points.len()];
    let mut triangle_colors = vec![Float3::zeros(); TRIANGLE_COUNT];

    // Compute the image center
    let center = Float2::new((screen_width / 2) as f64, (screen_height / 2) as f64);

    // Initialize a randomizer
    let mut g = rng();

    // Generate the random positions of each vertex in the x/y range 0.35..0.65 (widths/heights)
    for p in points.iter_mut() {
        let random_point = random_float2(&mut g, screen_width, screen_height);
        let offset = (&random_point - &center) * 0.3;
        *p = &center + &offset;
    }

    // Generate random velocities for each triangle
    for i in (0..velocities.len()).step_by(3) {
        let random_velocity = random_float2(&mut g, screen_width, screen_height);
        let velocity = (&random_velocity - &center) * 0.5;
        velocities[i + 0] = velocity.clone();
        velocities[i + 1] = velocity.clone();
        velocities[i + 2] = velocity;
    }

    // Assign a random color to each triangle
    for c in triangle_colors.iter_mut() {
        *c = random_color(&mut g);
    }

    // Return the vertices, velocities and colors
    (points, velocities, triangle_colors)
}

/// Generate a random Float2 with x/y ranges `0..width` and `0..height`.
fn random_float2(rng: &mut ThreadRng, width: usize, height: usize) -> Float2 {
    Float2::new(rng.random_range(0..width) as f64, rng.random_range(0..height) as f64)
}

/// Generate a random RGB color.
fn random_color(rng: &mut ThreadRng) -> Float3 {
    Float3::new(
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0)
    )
}

#[allow(dead_code)]
fn create_test_image() -> () {
    let mut image = rendering::image::ImageBuffer::new(WIDTH, HEIGHT);
    
    let a = Float2::new(0.2 * WIDTH as f64, 0.2 * HEIGHT as f64);
    let b = Float2::new(0.7 * WIDTH as f64, 0.4 * HEIGHT as f64);
    let c = Float2::new(0.4 * WIDTH as f64, 0.8 * HEIGHT as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let p = Float2::new(x as f64, y as f64);

            let (inside, _) = point_in_triangle(&a, &b, &c, &p);
            if inside {
                image[[x, y]] = Float3::new(0.0, 0.0, 1.0);
            }
        }
    }

    let file_name = "test_image.bmp";
    match write_image_to_file(&image, file_name.to_string()) {
        Ok(_) => (),
        Err(why) => match why.kind() {
            io::ErrorKind::NotFound => panic!("The path {} is non-existent! Make sure the folder structure exists.", file_name.to_string()),
            io::ErrorKind::PermissionDenied => panic!("You don't have permissions to write to file \"{}\"", file_name.to_string()),
            _ => panic!("Failed to write to file {}: {}", file_name, why),
        }
    };
}

fn write_image_to_file(image: &rendering::image::ImageBuffer, name: String) -> Result<(), io::Error> {
    let Ok(bmp_buffer) = image_to_bmp_buffer(&image) else {
        panic!("Failed to convert image to bitmap!");
    };
    write(name, &bmp_buffer)?;
    Ok(())
}