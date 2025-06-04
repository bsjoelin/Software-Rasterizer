use std::{fs::File, io::{Error, Write}};

mod vector_math;

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
    if image.is_empty() {
        panic!("Image has zero width!")
    }
    let image_width: u32 = image.len().try_into().expect("Resolution width has to fit inside a u32!");
    let image_height: u32 = image[0].len().try_into().expect("Resolution width has to fit inside a u32!");

    let byte_counts = [14, 40, 4*image_width*image_height];

    let mut file = File::create(name).expect("File couldn't be created in write mode!");

    let mut buffer: Vec<u8> = Vec::new();
    // --- Headers ---
    // BMP header start
    buffer.extend(b"BM");
    // Total file size
    buffer.extend(byte_counts.iter().sum::<u32>().to_ne_bytes());
    // Mark image as manually created
    buffer.extend(0u32.to_ne_bytes());
    // Data offset from start byte
    buffer.extend(byte_counts[0..2].iter().sum::<u32>().to_ne_bytes());
    // DIP header size
    buffer.extend(byte_counts[1].to_ne_bytes());
    // Image width
    buffer.extend(image_width.to_ne_bytes());
    // Image height
    buffer.extend(image_height.to_ne_bytes());
    // Number of color planes, must be 1
    buffer.extend(1u16.to_ne_bytes());
    // Bits per pixel (1 byte per color channel and one for alignment)
    buffer.extend((8 * 4u16).to_ne_bytes());  // TODO: Change back to 4u32!!
    // Set to RGB format without compression
    buffer.extend(0u32.to_ne_bytes());
    // Size of data
    buffer.extend(byte_counts[2].to_ne_bytes());
    // Print resolution and palette - we ignore that
    buffer.extend([0u8; 16]);

    // --- Write image data ---
    for y in 0..image_height {
        for x in 0..image_width {
            buffer.extend(convert_to_color(&image[x as usize][y as usize]));
            buffer.push(0u8);
        }
    }
    file.write_all(&buffer)?;

    Ok(())
}

fn convert_to_color(v: &Float3) -> Vec<u8> {
    let v = vec![v.b(), v.g(), v.r()];
    v.iter().map(|f| (f * 255.0) as u8).collect()
}