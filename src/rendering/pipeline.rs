use crate::rendering::image::ImageBuffer;
use crate::rendering::transforms::{vertex_to_screen, Transform};
use crate::vector_math::{vector::*, triangle::*};


/// Render triangles to an image buffer using rasterization
pub fn render2d(vertices: &Vec<Float2>, colors: &Vec<Float3>, image: &mut ImageBuffer) -> () {
    // For now, zero-size images cause a program panic
    if image.get_size() == 0 {
        panic!("Image has no size!")
    }

    // Loop over the triangles
    for i in (0..vertices.len()).step_by(3) {
        // Extract vertices
        let a = &vertices[i];
        let b = &vertices[i + 1];
        let c = &vertices[i + 2];

        let bbox = determine_bounding_box(a, b, c, image.get_width(), image.get_height());
        paint_in_triangle(a, b, c, bbox, colors[i / 3], image);
        
    }
}

/// Render 3D triangles to an image buffer using rasterization
pub fn render3d(vertices: &Vec<Float3>, colors: &Vec<Float3>, transform: &Transform, image: &mut ImageBuffer) -> () {
    if image.get_size() == 0 {
        panic!("Image has no size!")
    }

    // Loop over the triangles
    for i in (0..vertices.len()).step_by(3) {
        let a = vertex_to_screen(&vertices[i + 0], &transform, image.get_width(), image.get_height());
        let b = vertex_to_screen(&vertices[i + 1], &transform, image.get_width(), image.get_height());
        let c = vertex_to_screen(&vertices[i + 2], &transform, image.get_width(), image.get_height());

        let bbox = determine_bounding_box(&a, &b, &c, image.get_width(), image.get_height());
        paint_in_triangle(&a, &b, &c, bbox, colors[i / 3], image);
    }
}

fn paint_in_triangle(a: &Float2, b: &Float2, c: &Float2, bbox: BBox, color: Float3, image: &mut ImageBuffer) -> () {
    // Loop over pixels in the bounding box
    for y in bbox.min_y..=bbox.max_y {
        for x in bbox.min_x..=bbox.max_x {
            let p = Float2::new(x as f64, y as f64);

            // Is the current pixel inside the current triangle?
            if point_in_triangle(a, b, c, &p) {
                // Apply the triangle's color to the image buffer
                image[[x, y]] = color;
            }
        }
    }
}

struct BBox {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize
}

fn determine_bounding_box(a: &Float2, b: &Float2, c: &Float2, width: usize, height: usize) -> BBox {
    // Determine bounding box
    let min_x = f64::min(a.x, f64::min(b.x, c.x));
    let min_y = f64::min(a.y, f64::min(b.y, c.y));
    let max_x = f64::max(a.x, f64::max(b.x, c.x));
    let max_y = f64::max(a.y, f64::max(b.y, c.y));

    // Convert bounding box to integers of the image buffer
    let bbox_start_x = usize::clamp(min_x as usize, 0, width - 1);
    let bbox_start_y = usize::clamp(min_y as usize, 0, height - 1);
    let bbox_end_x = usize::clamp(max_x as usize + 1, 0, width - 1);
    let bbox_end_y = usize::clamp(max_y as usize + 1, 0, height - 1);

    BBox { min_x: bbox_start_x, min_y: bbox_start_y, max_x: bbox_end_x, max_y: bbox_end_y }
}