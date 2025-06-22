use rand::{rng, rngs::ThreadRng, Rng};
use std::{fs::write, io};

mod rendering;
mod vector_math;

use crate::rendering::bitmap::image_to_bmp_buffer;
use crate::vector_math::vector::*;
use crate::vector_math::triangle::*;

fn main() {
    create_test_images();
}

/// Generate a bitmap image of randomly initialized triangles
fn create_test_images() -> () {
    // Image dimensions
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;

    // Initialize image buffer
    let mut image = vec![vec![Float3::zeros(); HEIGHT]; WIDTH];

    // Get the random vertices, triangle velocities and colors
    let (points, _velocities, triangle_colors) = setup_triangles(WIDTH, HEIGHT);

    // Render the triangles to the image buffer
    render(&points, &triangle_colors, &mut image);

    // Save the current stage of the image buffer to a bitmap
    let file_name = format!("test_frame_{:03}.bmp", 0);
    match write_image_to_file(image, file_name.to_string()) {
        Err(why) => panic!("Failed to write frame {} to file {}: {}", 0, file_name, why),
        Ok(_) => (),
    };

}

/// Render triangles to an image buffer using (un-optimized) rasterization
fn render(vertices: &Vec<Float2>, colors: &Vec<Float3>, image: &mut Vec<Vec<Float3>>) -> () {
    // For now, zero-size images cause a program panic
    if image.is_empty() {
        panic!("Image has zero width!")
    }

    // Loop over the image pixels
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            // Loop over the triangles
            for i in (0..vertices.len()).step_by(3) {
                let p = Float2::new(x as f64, y as f64);

                // Is the current pixel inside the current triangle?
                if point_in_triangle(
                    &vertices[i], &vertices[i + 1], &vertices[i + 2], &p
                ) {
                    // If yes, then we apply the triangles color to the image buffer
                    image[x][y] = colors[i / 3];
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
    const WIDTH: usize = 128;
    const HEIGHT: usize = 128;

    let mut image = vec![vec![Float3::zeros(); HEIGHT]; WIDTH];
    
    let a = Float2::new(0.2 * WIDTH as f64, 0.2 * HEIGHT as f64);
    let b = Float2::new(0.7 * WIDTH as f64, 0.4 * HEIGHT as f64);
    let c = Float2::new(0.4 * WIDTH as f64, 0.8 * HEIGHT as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let p = Float2::new(x as f64, y as f64);

            if point_in_triangle(&a, &b, &c, &p) {
                image[x][y] = Float3::new(0.0, 0.0, 1.0);
            }
        }
    }

    let file_name = "test_image.bmp";
    match write_image_to_file(image, file_name.to_string()) {
        Ok(_) => (),
        Err(why) => match why.kind() {
            io::ErrorKind::NotFound => panic!("The path {} is non-existent! Make sure the folder structure exists.", file_name.to_string()),
            io::ErrorKind::PermissionDenied => panic!("You don't have permissions to write to file \"{}\"", file_name.to_string()),
            _ => panic!("Failed to write to file {}: {}", file_name, why),
        }
    };
}

fn write_image_to_file(image: Vec<Vec<Float3>>, name: String) -> Result<(), io::Error> {
    let Ok(bmp_buffer) = image_to_bmp_buffer(image) else {
        panic!("Failed to convert image to bitmap!");
    };
    write(name, &bmp_buffer)?;
    Ok(())
}