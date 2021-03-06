use std::rc::Rc;
use std::vec::Vec;
use std::boxed::Box;
use light::Light;
use shape::*;
use types::*;
use shader::*;
use hit::HitInfo;
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
        let gouraud_shader = Rc::new(
                GouraudShader { color: Color::new(1.0, 0.0, 0.0) }
            );
        let gouraud_shader_green = Rc::new(
                GouraudShader { color: Color::new(0.0, 1.0, 0.0) }
            );
        let ambient_occlusion_shader = Rc::new(
                AmbientOcculusionShader { samples: 256, color: Color::new(0.7411, 0.7411, 0.7411) }
            );

        let mut shapes: Vec<Box<Shape>> = Vec::new();
        shapes.push(Box::new(
            Sphere { 
                position: Pnt3::new(-2.0,0.0,0.0),
                // color: Color::new(1.0, 1.0, 1.0),
                radius: 1.0,
                shader: ambient_occlusion_shader.clone()
            } 
        ));
        shapes.push(Box::new(
            Sphere { 
                position: Pnt3::new(2.0,0.0,0.0),
                // color: Color::new(1.0, 1.0, 1.0),
                radius: 1.0,
                shader: ambient_occlusion_shader.clone()
            } 
        ));
        shapes.push(Box::new(
            Sphere { 
                position: Pnt3::new(0.0,1.0,0.0),
                // color: Color::new(1.0, 1.0, 1.0),
                radius: 1.0,
                shader: ambient_occlusion_shader.clone()
            } 
        ));

        shapes.push(Box::new(
            Sphere { 
                position: Pnt3::new(0.0,0.0,-10000000.0-1.0),
                // color: Color::new(1.0, 1.0, 1.0),
                radius: 10000000.0,
                shader: ambient_occlusion_shader.clone()
            } 
        ));

        let mut lights: Vec<Box<Light>> = Vec::new();
        let mut scene = Scene {
            shapes: shapes,
            lights: lights
        };
        scene
    }
}