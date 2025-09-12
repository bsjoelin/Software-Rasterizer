use rand::{rng, rngs::ThreadRng, Rng};
use std::iter::zip;
use std::{fs::write, io};

mod rendering;
mod vector_math;

use crate::rendering::bitmap::image_to_bmp_buffer;
use crate::vector_math::vector::*;
use crate::vector_math::triangle::*;
use crate::rendering::pipeline::ImageBuffer;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

struct Scene {
    image_buffer: ImageBuffer,
    vertices: Vec<Float2>,
    vertex_velocities: Vec<Float2>,
    triangle_colors: Vec<Float3>
}

fn main() {
    let mut scene = create_test_images();

    for i in 0..5 {
        render(&scene.vertices, &scene.triangle_colors, &mut scene.image_buffer);
    
        // Save the current stage of the image buffer to a bitmap
        let file_name = format!("images\\test_frame_{:03}.bmp", i);
        match write_image_to_file(&scene.image_buffer, file_name.to_string()) {
            Err(why) => panic!("Failed to write frame {} to file {}: {}", 0, file_name, why),
            Ok(_) => (),
        };

        update(&mut scene.vertices, &mut scene.vertex_velocities, 0.25);
        scene.image_buffer.clear();
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

/// Generate a bitmap image of randomly initialized triangles
fn create_test_images() -> Scene {
    // Initialize image buffer
    let image = ImageBuffer::new(WIDTH, HEIGHT);

    // Get the random vertices, triangle velocities and colors
    let (points, velocities, triangle_colors) = setup_triangles(WIDTH, HEIGHT);

    Scene {image_buffer: image, vertices: points, vertex_velocities: velocities, triangle_colors: triangle_colors}

}

/// Render triangles to an image buffer using rasterization
fn render(vertices: &Vec<Float2>, colors: &Vec<Float3>, image: &mut ImageBuffer) -> () {
    // For now, zero-size images cause a program panic
    if image.get_size() == 0 {
        panic!("Image has zero width!")
    }

    // Loop over the triangles
    for i in (0..vertices.len()).step_by(3) {
        // Extract vertices
        let a = &vertices[i];
        let b = &vertices[i + 1];
        let c = &vertices[i + 2];

        // Determine bounding box
        let min_x = f64::min(a.x, f64::min(b.x, c.x));
        let min_y = f64::min(a.y, f64::min(b.y, c.y));
        let max_x = f64::max(a.x, f64::max(b.x, c.x));
        let max_y = f64::max(a.y, f64::max(b.y, c.y));

        // Convert bounding box to integers of the image buffer
        let bbox_start_x = usize::clamp(min_x as usize, 0, image.get_width() - 1);
        let bbox_start_y = usize::clamp(min_y as usize, 0, image.get_height() - 1);
        let bbox_end_x = usize::clamp(max_x as usize + 1, 0, image.get_width() - 1);
        let bbox_end_y = usize::clamp(max_y as usize + 1, 0, image.get_height() - 1);

        // Loop over pixels in the bounding box
        for y in bbox_start_y..=bbox_end_y {
            for x in bbox_start_x..=bbox_end_x {
                let p = Float2::new(x as f64, y as f64);

                // Is the current pixel inside the current triangle?
                if point_in_triangle(a, b, c, &p) {
                    // Apply the triangle's color to the image buffer
                    image[[x, y]] = colors[i / 3];
                }
            }
        }
    }
}

/// Inintialize triangles with random positions, velocities and colors. Returns the flattened vertices vector, the velocities and the traingle colors.
fn setup_triangles(screen_width: usize, screen_height: usize) -> (Vec<Float2>, Vec<Float2>, Vec<Float3>) {
    const TRIANGLE_COUNT: usize = 250;

    // Initialize data arrays with 0-vectors
    let mut points = vec![Float2::zeros(); TRIANGLE_COUNT * 3];
    let mut velocities = vec![Float2::zeros(); points.len()];
    let mut triangle_colors = vec![Float3::zeros(); TRIANGLE_COUNT];

    // Compute the image center
    let center = Float2::new((screen_width / 2) as f64, (screen_height / 2) as f64);

    // Initialize a randomize
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
    let mut image = ImageBuffer::new(WIDTH, HEIGHT);
    
    let a = Float2::new(0.2 * WIDTH as f64, 0.2 * HEIGHT as f64);
    let b = Float2::new(0.7 * WIDTH as f64, 0.4 * HEIGHT as f64);
    let c = Float2::new(0.4 * WIDTH as f64, 0.8 * HEIGHT as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let p = Float2::new(x as f64, y as f64);

            if point_in_triangle(&a, &b, &c, &p) {
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

fn write_image_to_file(image: &ImageBuffer, name: String) -> Result<(), io::Error> {
    let Ok(bmp_buffer) = image_to_bmp_buffer(&image) else {
        panic!("Failed to convert image to bitmap!");
    };
    write(name, &bmp_buffer)?;
    Ok(())
}