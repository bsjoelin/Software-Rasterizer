use std::ops::{Index, IndexMut};

use crate::vector_math::vector::Float3;

#[derive(Debug)]
pub struct ImageBuffer{
    pub buffer: Vec<Float3>,
    width: usize,
    height: usize,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut buffer = Vec::new();
        buffer.reserve(size);
        for _ in 0..size {
            buffer.push(Float3::zeros());
        }
        Self{ buffer, width, height}
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
            self.buffer[i] = Float3::zeros();
        }
    }
}

impl Index<[usize; 2]> for ImageBuffer {
    type Output = Float3;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.buffer[index[0] + index[1] * self.width]
    }
}

impl IndexMut<[usize; 2]> for ImageBuffer {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.buffer[index[0] + index[1] * self.width]
    }
}