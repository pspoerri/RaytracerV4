use std::vec::Vec;
use std::boxed::Box;
use light::Light;
use primitive::*;
use types::*;
use ray::Ray;
// use std::num::abs;
use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};


pub struct Scene {
    pub primitives: Vec<Box<Primitive>>,
    pub lights: Vec<Box<Light>>
    // pub elements: Vec<Vector3>
}

impl Scene {
    pub fn new() -> Scene {
        let mut primitives: Vec<Box<Primitive>> = Vec::new();
        primitives.push(Box::new(
            Sphere { 
                position: Vec3::new(0.0,0.0,0.0),
                color: Vec3::new(1.0, 1.0, 1.0),
                radius: 0.25
            } 
        ));

        let mut lights: Vec<Box<Light>> = Vec::new();
        let mut scene = Scene {
            primitives: primitives,
            lights: lights
        };
        scene
    }
    pub fn compute(&self, ray: &Ray) -> Color {
        Color::new(ray.dir.x.abs(), ray.dir.y.abs(), ray.dir.z.abs())*100.0
        // Color::new(ray.dir.x, ray.dir.y, 0.0)+Color::new(0.5, 0.5, 0.0)
        // Color::new(abs(ray.dir.x))
    }
}