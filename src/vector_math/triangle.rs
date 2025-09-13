use crate::vector_math::vector::{Float2, Float3};

/// Determine whether `point` is inside the triangle spanned by `v1`->`v2`->`v3`->`v1`.
pub fn point_in_triangle(v1: &Float2, v2: &Float2, v3: &Float2, point: &Float2) -> (bool, Float3) {
    let area_12p = signed_triangle_area(v1, v2, point);
    let area_23p = signed_triangle_area(v2, v3, point);
    let area_31p = signed_triangle_area(v3, v1, point);
    // If the points are all right of, we are inside. Assumes clockwise winding of vertices
    let inside = area_12p >= 0.0 && area_23p >= 0.0 && area_31p >= 0.0;

    // Calculate normalized weights for trilinear interpolation
    let total_area = area_12p + area_23p + area_31p;
    if total_area < 1e-16 {  // Escape early if the triangle has no area
        return (false, Float3::zeros())
    }
    let inverse_area = 1.0 / total_area;
    let weights = Float3::new(area_12p * inverse_area, area_23p * inverse_area, area_31p * inverse_area);

    (inside, weights)
}

/// Calculate the area of the triangle abc.
/// 
/// A positive area means clockwise winding of the triangle and counter-clockwise for negative area
pub fn signed_triangle_area(a: &Float2, b: &Float2, c: &Float2) -> f64 {
    let ac = c - a;
    let ab_normal = (b - a).perpendicular();
    ac.dot(&ab_normal) / 2.0
}