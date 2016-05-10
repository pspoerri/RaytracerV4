use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use ray::Ray;
use types::*;
use primitive::Primitive;

pub struct HitInfo<'a> {
    pub primitive: &'a Primitive,
    pub d: Float,     // Hit distance
    pub i: Vec3,    // Incident vector
    pub p: Pnt3,    // Hit point
    pub o: Pnt3,    // Hit origin
    pub n: Vec3     // Normal vector
}

impl<'a>  HitInfo<'a> {
    pub fn new(
        primitive: &'a Primitive,
        distance: Float, 
        incident: Vec3,
        hit_point: Pnt3,
        hit_origin: Pnt3) -> HitInfo
    {
        let hit = HitInfo {
            primitive: primitive,
            d: distance,
            i: incident,
            p: hit_point,
            o: hit_origin,
            n: Vec3::new(1.0, 1.0, 1.0)
        };
        hit
    }
}