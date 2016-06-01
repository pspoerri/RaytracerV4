use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye, clamp, Rotate, Rotation3, new_identity};
use ray::Ray;
use types::*;
use hit::HitInfo;
use std::option::Option;
use std::f64;
use rand::{thread_rng, ThreadRng, Rng};
use warp::*;
use std::f64::consts::*;

use renderer::Renderer;

pub trait Shader {
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color;
}

pub struct GouraudShader {
    pub color: Color
}

impl Shader for GouraudShader {
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color {
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
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color {
        self.color
    }
}

pub struct AmbientOcculusionShader {
    pub samples: i32,
    pub color: Color
}

impl Shader for AmbientOcculusionShader {
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut rng = thread_rng();
        let fsamples = self.samples as f64;

        let axis = Vec3::new(0.0, 0.0, 1.0);
        let rot = rotate_to(&axis, &hit.n);
        for i in 0..self.samples {
            let (dir, pdf) = sample_hit(hit, &mut rng);

            let mut ray = Ray::new(
                &hit.p, 
                rot*dir, 
                f64::EPSILON,
                f64::INFINITY);
            match renderer.intersect(&mut ray) {
                None => {color += self.color;}
                Some(hit) => {}
            }
        }
        color / (fsamples)
    }
}

fn sample_hit(hit: &HitInfo, rng: &mut ThreadRng) -> (Vec3, Float) {
    let s: Float = rng.gen_range(0.0, 1.0);
    let t: Float = rng.gen_range(0.0, 1.0);
    let dir = warp_point(s.clone(), t.clone(), WarpFunction::CosineHemisphere);
    let pdf = get_pdf(s, t, WarpFunction::CosineHemisphere);
    let sample_dir = Vec3::new(0.0, 0.0, 1.0);
    (dir, pdf)
}

fn rotate_to(from: &Vec3, to: &Vec3) -> Rotation3<Float> {
    if from.dot(to) > 0.9999 {
        return new_identity(3);
    }
    let angle: Float;
    let mut axis = from.cross(&to);
    if (axis.norm_squared() < 1e-10) {
        if (from.x > -0.9) && (from.x < 0.9) {
            axis = Vec3::new(1.0, 0.0, 0.0);
        } else if (from.y > -0.9) && (from.y < 0.9) {
            axis = Vec3::new(0.0, 1.0, 0.0);
        } else {
            axis = Vec3::new(0.0, 0.0, 1.0);
        }
        angle = PI;
    } else {
        angle = from.dot(to).acos();
        axis.normalize_mut();
    }
    return Rotation3::new(axis*angle);
}