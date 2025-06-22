use std::{fs::write, io};

mod rendering;
mod vector_math;

use crate::rendering::bitmap::image_to_bmp_buffer;
use crate::vector_math::vector::*;
use crate::vector_math::triangle::*;

fn main() {
    create_test_image();
}

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