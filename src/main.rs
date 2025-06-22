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

fn create_test_images() -> () {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;

    let mut image = vec![vec![Float3::zeros(); HEIGHT]; WIDTH];

    let (points, _velocities, triangle_colors) = setup_triangles(WIDTH, HEIGHT);

    render(&points, &triangle_colors, &mut image);

    let file_name = format!("test_frame_{:03}.bmp", 0);
    match write_image_to_file(image, file_name.to_string()) {
        Err(why) => panic!("Failed to write frame {} to file {}: {}", 0, file_name, why),
        Ok(_) => (),
    };

}

fn render(vertices: &Vec<Float2>, colors: &Vec<Float3>, image: &mut Vec<Vec<Float3>>) -> () {
    if image.is_empty() {
        panic!("Image has zero width!")
    }

    for y in 0..image.len() {
        for x in 0..image[0].len() {
            for i in (0..vertices.len()).step_by(3) {
                let p = Float2::new(x as f64, y as f64);

                if point_in_triangle(
                    &vertices[i], &vertices[i + 1], &vertices[i + 2], &p
                ) {
                    image[x][y] = colors[i / 3];
                }
            }
        }
    }
}

fn setup_triangles(screen_width: usize, screen_height: usize) -> (Vec<Float2>, Vec<Float2>, Vec<Float3>) {
    const TRIANGLE_COUNT: usize = 250;

    let mut points = vec![Float2::zeros(); TRIANGLE_COUNT * 3];
    let mut velocities = vec![Float2::zeros(); points.len()];
    let mut triangle_colors = vec![Float3::zeros(); TRIANGLE_COUNT];

    let center = Float2::new((screen_width / 2) as f64, (screen_height / 2) as f64);
    let mut g = rng();

    for p in points.iter_mut() {
        let random_point = random_float2(&mut g, screen_width, screen_height);
        let offset = (&random_point - &center) * 0.3;
        *p = &center + &offset;
    }

    for i in (0..velocities.len()).step_by(3) {
        let random_velocity = random_float2(&mut g, screen_width, screen_height);
        let velocity = (&random_velocity - &center) * 0.5;
        velocities[i + 0] = velocity.clone();
        velocities[i + 1] = velocity.clone();
        velocities[i + 2] = velocity;
        triangle_colors[i / 3] = random_color(&mut g);
    }
    (points, velocities, triangle_colors)
}

fn random_float2(rng: &mut ThreadRng, width: usize, height: usize) -> Float2 {
    Float2::new(rng.random_range(0..width) as f64, rng.random_range(0..height) as f64)
}

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