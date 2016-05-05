use ray::Ray;
use types::*;

pub trait Primitive {
    fn compute_distance(&self, ray: &Ray) -> Float;
    fn compute_hit(&self, ray: &Ray) -> Color;
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: Float,
    pub color: Vec3
}

impl Primitive for Sphere {
    fn compute_hit(&self, ray: &Ray) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
    fn compute_distance(&self, ray: &Ray) -> Float {
        1.0
    }
}