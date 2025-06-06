use std::ops::{Sub};

/// Float representation of 3D vector
#[derive(Clone, Debug)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Float3 {
    /// Creates new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{ x, y, z }
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

#[derive(Clone, Debug)]
pub struct Float2 {
    pub x: f64,
    pub y: f64,
}

impl Float2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zeros() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // Get vector rotated by 90 degrees clockwise
    pub fn perpendicular(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }
}

// impl<'a> Sub<&Float2> for &'a Float2 {
impl<'a, 'b> Sub<&'b Float2> for &'a Float2 {
    type Output = Float2;

    fn sub(self, other: &'b Float2) -> Float2 {
        Float2 { x: self.x - other.x, y: self.y - other.y }
    }
}