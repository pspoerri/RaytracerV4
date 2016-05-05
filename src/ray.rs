use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use types::*;

#[derive(Debug)]
pub struct Ray {
    pub tmin: Float,
    pub tmax: Float,
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: &Vec3, dir: Vec3, tmin: Float, tmax: Float) -> Ray {
        let mut ray = Ray {
            tmin: tmin,
            tmax: tmax,
            origin: origin.clone(),
            dir: dir.normalize()
        };
        ray
    }
}