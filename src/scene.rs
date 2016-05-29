use std::vec::Vec;
use std::boxed::Box;
use light::Light;
use shape::*;
use types::*;
use ray::Ray;
// use std::num::abs;
use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};


pub struct Scene {
    pub shapes: Vec<Box<Shape>>,
    pub lights: Vec<Box<Light>>
    // pub elements: Vec<Vector3>
}

impl Scene {
    pub fn new() -> Scene {
        let mut shapes: Vec<Box<Shape>> = Vec::new();
        shapes.push(Box::new(
            Sphere { 
                position: Pnt3::new(0.0,0.0,0.0),
                color: Color::new(1.0, 1.0, 1.0),
                radius: 1.0
            } 
        ));

        let mut lights: Vec<Box<Light>> = Vec::new();
        let mut scene = Scene {
            shapes: shapes,
            lights: lights
        };
        scene
    }
    // pub fn intersect(ray: &Ray) -> Option<&Shape>
    // {
        
    // }
}