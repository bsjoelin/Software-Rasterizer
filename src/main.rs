use std::{fs::File, io::{Error, Write}};

mod rendering;
mod vector_math;

use crate::rendering::bitmap::image_to_bmp_buffer;
use crate::vector_math::vector::*;

fn main() {
    create_test_image();
}

fn create_test_image() -> () {
    const WIDTH: usize = 128;
    const HEIGHT: usize = 128;

    let mut image = vec![vec![Float3::zeros(); HEIGHT]; WIDTH];
    
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (x as f64) / ((WIDTH - 1) as f64);
            let g = (y as f64) / ((HEIGHT - 1) as f64);
            image[x][y] = Float3::new(r, g, 0.0);
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