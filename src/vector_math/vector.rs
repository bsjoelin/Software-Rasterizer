use std::ops::{Add, AddAssign, Mul, Sub};

/// Float representation of 3D vector
#[derive(Clone, Copy, Debug)]
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

impl Add<f64> for Float3 {
    type Output = Float3;
    fn add(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl Add<Float3> for Float3 {
    type Output = Float3;
    fn add(self, other: Float3) -> Self::Output {
        Self::Output { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Mul<f64> for Float3 {
    type Output = Float3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<f64> for &Float3 {
    type Output = Float3;
    fn mul(self, rhs: f64) -> Self::Output {
        *self * rhs
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

/// Implement addition between Float2 references
impl<'a, 'b> Add<&'b Float2> for &'a Float2 {
    type Output = Float2;

    fn add(self, other: &'b Float2) -> Self::Output {
        Float2 { x: self.x + other.x, y: self.y + other.y }
    }
}

/// Implement in-place addition using Float2 reference
impl<'b> AddAssign<&'b Float2> for Float2 {    
    fn add_assign(&mut self, other: &'b Float2) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Implement in-place addition using Float2 (consumes)
impl AddAssign<Float2> for Float2 {
    fn add_assign(&mut self, other: Float2) {
        self.x += other.x;
        self.y += other.y;
    }
}


/// Implement subtraction between Float2 references
impl<'a, 'b> Sub<&'b Float2> for &'a Float2 {
    type Output = Float2;

    fn sub(self, other: &'b Float2) -> Float2 {
        Float2 { x: self.x - other.x, y: self.y - other.y }
    }
}

/// Implement multiplication between Float2 references
impl<'a> Mul<f64> for &'a Float2 {
    type Output = Float2;

    fn mul(self, rhs: f64) -> Self::Output {
        Float2 {x: self.x * rhs, y: self.y * rhs}
    }
}

/// Implement multiplication between a Float2 (consumes) and an f64
impl Mul<f64> for Float2 {
    type Output = Float2;

    fn mul(self, rhs: f64) -> Self::Output {
        Float2 {x: self.x * rhs, y: self.y * rhs}
    }
}