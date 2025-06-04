/// Float representation of 3D vector
#[derive(Clone, Debug)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Float3 {
    /// Creates new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Float3 {
        Float3{ x, y, z }
    }

    /// Creates a Zero-vector
    pub fn zeros() -> Float3 {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Alias for the x-component. Useful for working with colors.
    pub fn r(&self) -> f64 {
        self.x
    }

    /// Alias for the y-component. Useful for working with colors.
    pub fn g(&self) -> f64 {
        self.y
    }

    /// Alias for the z-component. Useful for working with colors.
    pub fn b(&self) -> f64 {
        self.z
    }
}