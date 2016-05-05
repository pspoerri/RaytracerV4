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
mod primitive;
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
    // let mut camera = Camera::new(1000.0, 1000.0);
    // camera.print();
    let window = Window::new(1024, 768);
    let mut scene = Scene::new();
    draw_png(&window, &scene)
}
