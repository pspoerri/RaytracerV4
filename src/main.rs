extern crate nalgebra;
extern crate num_traits;
extern crate image;
extern crate rand;
// extern crate threadpool;
extern crate scoped_threadpool;

mod camera;
mod hit;
mod light;
mod ray;
mod renderer;
mod sampling;
mod scene;
mod shader;
mod shape;
mod types;
mod warp;
mod window;

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
