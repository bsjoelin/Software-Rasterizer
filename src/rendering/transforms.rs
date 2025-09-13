use crate::vector_math::vector::{Float2, Float3};

pub struct Transform {
    /// Rotation around the x-axis
    pub pitch: f64,
    /// Rotation around the z-axis (the up)
    pub yaw: f64,
    /// World position
    pub position: Float3,
}

impl Transform {
    pub fn new(rotation_around_x: f64, rotation_around_z: f64, position: Float3) -> Self {
        Self { pitch: rotation_around_x, yaw: rotation_around_z, position }
    }

    pub fn empty() -> Self {
        Self { pitch: 0.0, yaw: 0.0, position: Float3::zeros() }
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
        self.transform_vector(&ihat, &jhat, &khat, vertex) + &self.position
    }

    fn transform_vector(&self, ih: &Float3, jh: &Float3, kh: &Float3, vertex: &Float3) -> Float3 {
        ih * vertex.x + jh * vertex.y + kh * vertex.z
    }
}

/// Transform vertex position into screen-space position [pixel coordinates]
/// 
/// The fov must be in radians
pub fn vertex_to_screen(vertex: &Float3, transform: &Transform, screen_size: &Float2, fov: f64) -> Float2 {
    let vertex_world = transform.vertex_to_world(vertex);

    let world_screen_height = f64::tan(fov/2.0) * 2.0;
    let pixel_factor = screen_size.y as f64 / world_screen_height / vertex_world.z;

    let mut pixel_offset = Float2::new(vertex_world.x, vertex_world.y) * pixel_factor;
    pixel_offset += screen_size / 2.0;
    pixel_offset
}