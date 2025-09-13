use std::{f64, ops::{Index, IndexMut}};

use crate::vector_math::vector::Float3;

pub trait Default<T> {
    fn get_default() -> T;
}

impl Default<Float3> for Float3 {
    fn get_default() -> Float3 { Float3::zeros() }
}

impl Default<f64> for f64 {
    fn get_default() -> f64 { f64::INFINITY }
}

pub type ImageBuffer = Buffer2D<Float3>;
pub type DepthBuffer = Buffer2D<f64>;

pub struct Buffer2D<T: Default<T>> {
    buffer: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default<T>> Buffer2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut buffer = Vec::new();
        buffer.reserve(size);
        for _ in 0..size {
            buffer.push(T::get_default());
        }
        Self{ buffer, width, height }
    }

    pub fn get_size(&self) -> usize {
        self.width * self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn clear(&mut self) -> () {
        for i in 0..self.get_size() {
            self.buffer[i] = T::get_default();
        }
    }
}

impl<T: Default<T>> Index<[usize; 2]> for Buffer2D<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.buffer[index[0] + index[1] * self.width]
    }
}

impl<T: Default<T>> IndexMut<[usize; 2]> for Buffer2D<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.buffer[index[0] + index[1] * self.width]
    }
}
