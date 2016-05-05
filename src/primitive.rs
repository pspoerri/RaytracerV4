use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use ray::Ray;
use types::*;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> bool;
    fn compute_distance(&self, ray: &Ray) -> Float;
    fn compute_hit(&self, ray: &Ray) -> Color;
}

pub struct Sphere {
    pub position: Pnt3,
    pub radius: Float,
    pub color: Color
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let c = self.position;
        let o = ray.origin;
        let d = ray.dir;
        let r2 = self.radius*self.radius;
        let dk = (c-o).dot(&d);
        let D2 = (c-o).norm_squared()-dk*dk;
        if D2 > r2 {
            return false;
        }
        let f = Float::sqrt(r2-D2);
        let t = dk;
        let t1 = t+f;
        let t2 = t-f;

        let t = t1.min(t2);
        if t < ray.tmin {
            let t = t1.max(t2);
            if (t < ray.tmax) {
                // Hit distance is t
            }
            return false;
        }
        if t > ray.tmax {
            return false;
        }
        return true;
    }
    fn compute_hit(&self, ray: &Ray) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
    fn compute_distance(&self, ray: &Ray) -> Float {
        1.0
    }
}