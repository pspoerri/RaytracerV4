use camera::Camera;
use scene::Scene;
use renderer::Renderer;
use image::{ImageBuffer, Rgb};
use std::vec::Vec;
use types::*;

use std::sync::{Arc, Barrier};
// use threadpool::ThreadPool;
use scoped_threadpool::Pool;

#[derive(Debug)]
pub struct Window {
    pub camera: Camera,
    pub width: u32,
    pub height: u32
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let mut window = Window{
            camera: Camera::new(width, height),
            width: width,
            height: height
        };
        window
    }
    pub fn draw_as_image(&self, scene: &Scene) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);        
        let mut pool = Pool::new(4);
        let renderer: Renderer = Renderer::new(scene);
        let renderer_ref: &Renderer = &renderer;
        let camera_ref: &Camera = &self.camera;
        pool.scoped(|scope| {
            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                scope.execute(move || {
                    let mut ray = camera_ref.generate_ray(x, y);
                    let color: Color = renderer_ref.render(&mut ray)*255.0;
                    // println!("{:?}", color);
                    *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8]);
                });
            }
        });

        imgbuf
    }
    pub fn draw(&self, scene: &Scene) {
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.camera.generate_ray(x, y);
                println!("Ray at ({x}, {y})", x=x, y=y);
                println!("{:?}", ray);
            }
        }
    }
}