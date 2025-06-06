use std::{fs::File, io::{Error, Write}};

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
        Err(why) => panic!("Failed to write to file {}: {}", file_name, why),
        Ok(_) => (),
    };
}

fn write_image_to_file(image: Vec<Vec<Float3>>, name: String) -> Result<(), Error> {
    let Ok(bmp_buffer) = image_to_bmp_buffer(image) else {
        panic!("Failed to convert image to bitmap!");
    };
    let mut file = File::create(name).expect("File couldn't be created in write mode!");
    file.write_all(&bmp_buffer)?;

    Ok(())
}