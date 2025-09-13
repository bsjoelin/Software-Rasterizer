pub mod bitmap;
pub mod image;
pub mod pipeline;
pub mod transforms;

use crate::rendering::image::{ImageBuffer, DepthBuffer};

pub struct RenderTarget {
    pub image_buffer: ImageBuffer,
    pub depth_buffer: DepthBuffer,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image_buffer: ImageBuffer::new(width, height),
            depth_buffer: DepthBuffer::new(width, height),
        }
    }

    pub fn get_size(&self) -> usize {
        self.image_buffer.get_size()
    }

    pub fn get_width(&self) -> usize {
        self.image_buffer.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.image_buffer.get_height()
    }

    pub fn clear(&mut self) -> () {
        self.image_buffer.clear();
        self.depth_buffer.clear();
    }
}