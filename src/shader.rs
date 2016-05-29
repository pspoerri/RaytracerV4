use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye, clamp};
use ray::Ray;
use types::*;
use hit::HitInfo;
use std::option::Option;
use scene::Scene;

pub trait Shader {
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color;
}

pub struct GouraudShader {
    pub color: Color
}

impl Shader for GouraudShader {
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color {
        let n = Color::new(0.5, 0.5, 0.5)+hit.n*0.5;
        let c = &self.color;
        let r = n.x*c.x;
        let g = n.y*c.y;
        let b = n.z*c.z;
        Color::new(r,g,b)
    }
}

pub struct PhongShader {
    color: Color
}

impl Shader for PhongShader {
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color {
        self.color
    }
}

pub struct LambertShader {
    color: Color
}

impl Shader for LambertShader {
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color {
        let hN = &hit.n;
        let hO = &hit.p;

        for s in &scene.lights {
            // match s.intersect(&ray) {
            //     None => {},
            //     Some(hit) => {
            //         ray.tmin = hit.d;
            //         value = Some(hit);               
            //     }
            // }
        }
        self.color
    }   
}