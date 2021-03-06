use std::sync::Mutex;

use super::{Ray, World};
use crate::{
    interface::Image,
    math::{Matrix, Tuple},
    properties::COLOR_BLACK,
};

use rayon::prelude::*;

const MAX_REFLECTIONS: u8 = 5;

pub struct Camera {
    pub hsize: u16,
    pub vsize: u16,
    pub half_width: f64,
    pub half_height: f64,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: u16, vsize: u16, field_of_view: f64) -> Self {
        let view_units = (field_of_view / 2.0).tan() * 2.0;
        let max_dimension = hsize.max(vsize) as f64;

        let pixel_size = view_units / max_dimension;

        let half_width = hsize as f64 * pixel_size / 2.0;
        let half_height = vsize as f64 * pixel_size / 2.0;

        // Original formula
        //
        // let half_view = (field_of_view / 2.0).tan();
        // let aspect = hsize as f64 / vsize as f64;
        // let (half_width, half_height) = if aspect >= 1.0 {
        //     (half_view, half_view / aspect)
        // } else {
        //     (half_view * aspect, half_view)
        // };
        // let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            field_of_view,
            transform: Matrix::identity(4),
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: u16, py: u16) -> Ray {
        // Offset from the canvas edge to the pixel's center
        //
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let camera_inverse_transform = self.transform.inverse();

        // The canvas's z is -1!!
        //
        let pixel = &camera_inverse_transform * &Tuple::point(world_x, world_y, -1);
        let origin = &camera_inverse_transform * &Tuple::point(0, 0, 0);

        let direction = (pixel - &origin).normalize();

        Ray { origin, direction }
    }

    pub fn render<T: Image>(&self, world: &World) -> T {
        let mut pixels_buffer = vec![vec![COLOR_BLACK; self.hsize as usize]; self.vsize as usize];
        let pixels_buffer_mtx = Mutex::new(&mut pixels_buffer);

        (0..self.vsize).into_par_iter().for_each(|y| {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray, MAX_REFLECTIONS);

                let mut pixels_buffer = pixels_buffer_mtx.lock().unwrap();
                pixels_buffer[y as usize][x as usize] = color;
            }
        });

        T::from_pixels(pixels_buffer, self.hsize, self.vsize)
    }
}
