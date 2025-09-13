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
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();

        // Compute yaw basis vectors
        let ihat_yaw = Float3::new(cy, 0.0, sy);
        let jhat_yaw = Float3::new(0.0, 1.0, 0.0);
        let khat_yaw = Float3::new(-sy, 0.0, cy);
        // Compute pitch basis vectors
        let ihat_pitch = Float3::new(1.0, 0.0, 0.0);
        let jhat_pitch = Float3::new(0.0, cp, -sp);
        let khat_pitch = Float3::new(0.0, sp, cp);
        // Apply yaw transformation to pitch basis vectors
        let ihat = self.transform_vector(&ihat_yaw, &jhat_yaw, &khat_yaw, &ihat_pitch);
        let jhat = self.transform_vector(&ihat_yaw, &jhat_yaw, &khat_yaw, &jhat_pitch);
        let khat = self.transform_vector(&ihat_yaw, &jhat_yaw, &khat_yaw, &khat_pitch);
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