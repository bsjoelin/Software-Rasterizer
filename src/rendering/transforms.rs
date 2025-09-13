use crate::vector_math::vector::{Float2, Float3};

pub struct Transform {
    /// Rotation around the x-axis
    pub pitch: f64,
    /// Rotation around the z-axis (the up)
    pub yaw: f64,
}

impl Transform {
    pub fn new(rotation_around_x: f64, rotation_around_z: f64) -> Self {
        Self { pitch: rotation_around_x, yaw: rotation_around_z }
    }

    fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        // Trigonometry of rotation angles
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();

        // Combined pitch and yaw - worked out by hand
        let ihat = Float3::new(cy, 0.0, sy);
        let jhat = Float3::new(sp*sy, cp, -sp*cy);
        let khat = Float3::new(-cp*sy, sp, cp*cy);

        (ihat, jhat, khat)
    }

    pub fn vertex_to_world(&self, vertex: &Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        self.transform_vector(&ihat, &jhat, &khat, vertex)
    }

    fn transform_vector(&self, ih: &Float3, jh: &Float3, kh: &Float3, vertex: &Float3) -> Float3 {
        ih * vertex.x + jh * vertex.y + kh * vertex.z
    }
}

/// Transform vertex position into screen-space position [pixel coordinates]
pub fn vertex_to_screen(vertex: &Float3, tranform: &Transform, width: usize, height: usize) -> Float2 {
    let vertex_world = tranform.vertex_to_world(vertex);

    let world_screen_height = 5f64;
    let pixel_factor = height as f64 / world_screen_height;

    let mut pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixel_factor;
    pixel_offset += Float2::new(width as f64 / 2f64, height as f64 / 2f64);
    pixel_offset
}