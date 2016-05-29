use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use ray::Ray;
use types::*;
use hit::HitInfo;
use std::option::Option;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
    fn compute_distance(&self, ray: &Ray) -> Float;
}

pub struct Sphere {
    pub position: Pnt3,
    pub radius: Float,
    pub color: Color
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let c = self.position;
        let o = ray.origin;
        let d = ray.dir;
        let r2 = self.radius*self.radius;
        let dk = (c-o).dot(&d);
        let D2 = (c-o).norm_squared()-dk*dk;
        if D2 > r2 {
            return None;
        }
        let f = Float::sqrt(r2-D2);
        let t = dk;
        let t1 = t+f;
        let t2 = t-f;

        let t = t1.min(t2);
        if t < ray.tmin {
            let t = t1.max(t2);
            // if (t < ray.tmax) {
            //     // Hit distance is t
            // }
            return None;
        }
        if t > ray.tmax {
            return None;
        }
        return Some(HitInfo::new(&*self, t, -d, o+t*d, o));
    }
    fn compute_distance(&self, ray: &Ray) -> Float {
        1.0
    }
}