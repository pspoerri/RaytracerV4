use std::rc::Rc;
use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use ray::Ray;
use types::*;
use hit::HitInfo;
use scene::Scene;
use shader::Shader;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color;
}

pub struct Sphere {
    pub position: Pnt3,
    pub radius: Float,
    pub shader: Rc<Shader>
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
            // let t = t1.max(t2);
            // if (t < ray.tmax) {
            //     // Hit distance is t
            // }
            return None;
        }
        if t > ray.tmax {
            return None;
        }
        let p = o+t*d;
        let n = (p-self.position)/self.radius;
        return Some(HitInfo::new(&*self, t, -d, p, o, n));
    }
    fn shade(&self, hit: &HitInfo, scene: &Scene) -> Color {
        self.shader.shade(hit, scene)
    }
}