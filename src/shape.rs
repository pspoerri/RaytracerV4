use std::rc::Rc;
use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use ray::Ray;
use types::*;
use hit::HitInfo;
use renderer::Renderer;
use shader::Shader;
use rand::Rng;
use warp::*;
use std::f64;


pub trait Shape: Sync {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color;
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
        if t < ray.tmin && t.abs() > f64::EPSILON {
            return None;
        }
        if t > ray.tmax {
            return None;
        }
        let p = o+t*d;
        let n = (p-self.position)/self.radius;
        return Some(HitInfo::new(&*self, t, -d, p, o, n));
    }
    fn shade(&self, hit: &HitInfo, renderer: &Renderer) -> Color {
        self.shader.shade(hit, renderer)
    }
}

unsafe impl Sync for Sphere {}

// pub struct Triangle {
//     pub v1: Pnt3,
//     pub v2: Pnt3,
//     pub v3: Pnt3,
//     pub n: Vec3
// }

// impl Triangle {
//     fn new(v1: Pnt3, v2: Pnt3, v3: Pnt3) -> Triangle {
//         let e1 = v2 - v1;
//         let e2 = v3 - v1;
//         let n = e1.cross(&e2).normalize();
//     }
// }

// impl Shape for Triangle {
//     fn intersect(&self, ray: Ray) -> Option<HitInfo> {

//     }
// }
