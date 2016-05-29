use camera::Camera;
use scene::Scene;
use renderer::Renderer;
use image::{ImageBuffer, Rgb};
use std::vec::Vec;
use types::*;

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
        let renderer = Renderer::new(scene);
        let mut imgbuf = ImageBuffer::new(self.width, self.height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let ray = self.camera.generate_ray(x, y);
            let color: Color = renderer.render(&ray)*255.0;
            // println!("{:?}", color);
            *pixel = Rgb([color.x as u8, color.y as u8, color.z as u8])
        }
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