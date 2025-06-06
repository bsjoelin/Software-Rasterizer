use crate::vector_math::vector::Float2;

/// Determine whether `point` is inside the triangle spanned by `v1`->`v2`->`v3`->`v1`.
pub fn point_in_triangle(v1: &Float2, v2: &Float2, v3: &Float2, point: &Float2) -> bool {
    let right_of_12 = point_right_of_line(v1, v2, point);
    let right_of_23 = point_right_of_line(v2, v3, point);
    let right_of_31 = point_right_of_line(v3, v1, point);
    // If the point is on the same side, we are inside it
    right_of_12 == right_of_23 && right_of_23 == right_of_31
}

/// Determine if `point` is to the _right_ of the line from `a` to `b`.
fn point_right_of_line(a: &Float2, b: &Float2, point: &Float2) -> bool {
    // The dot product between a->point and a->b is positive if they point in 
    // the same direction. By rotating a->b 90 degrees clockwise, we can thus
    // determine if a->point is to the right of a->b (>0) or left (<0)
    let line_normal = (b - a).perpendicular();
    let ap = point - a;
    ap.dot(&line_normal) >= 0.0
}