extern crate nalgebra;
extern crate num_traits;
extern crate image;

// pub mod math;
mod types;
mod camera;
mod window;
mod scene;
mod ray;
mod light;
mod shape;
mod hit;
mod renderer;

use window::Window;
use scene::Scene;

fn draw_png(window: &Window, scene: &Scene) {
    use std::fs::File;
    use std::path::Path;

    let imgbuf = window.draw_as_image(&scene);


    let ref mut fout = File::create(&Path::new("rendering.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

fn main() {
    let mut window = Window::new(1024, 768);
    // let window = Window::new(3, 3);
    let mut scene = Scene::new();
    draw_png(&window, &scene)
}
