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
                position: Pnt3::new(0.0,0.0,0.0),
                color: Color::new(1.0, 1.0, 1.0),
                radius: 1.0
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
        let mut c = Color::new(0.0, 0.0, 0.0);
        for p in &self.primitives {
            if (p.intersect(&ray)) {
                c.x = 1.0;
                c.y = 1.0;
                c.z = 1.0; 
            }
        }
        c
    }
}