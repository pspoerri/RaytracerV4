use types::*;
use std::f64::*;
use std::f64::consts::*;

// Source: Physically based Rendering, Chapter Monte Carlo Integration

pub enum WarpFunction {
    UniformHemisphere,
    UniformSphere,
    UniformDisk,
    ConcentricDisk,
    CosineHemisphere,
    UniformTriangle,

    Uniform_Square,
    Uniform_Cylinder,
    Uniform_Sphere_Cap,
    Phong_Hemisphere,
}

pub fn get_pdf(x: Float, y: Float, warp_function: WarpFunction) -> Float {
    match warp_function {
         WarpFunction::UniformHemisphere => 1.0/(2.0*PI),
         WarpFunction::UniformSphere => 1.0/(4.0*PI),
         WarpFunction::UniformDisk | WarpFunction::ConcentricDisk => 1.0/PI,
         WarpFunction::CosineHemisphere => {
            let mut rho = 0.0;
            let mut theta = 0.0;
            compute_concentric_map_rho_theta(x, y, &mut rho, &mut theta);
            1.0/PI*Float::cos(theta)*Float::sin(theta)
        },
        WarpFunction::UniformTriangle => 2.0,
        _ => 1.0
    }
}

pub fn warp_point(s: Float, t: Float, warp_function: WarpFunction) -> Vec3 {
    let mut v = Vec3::new(0.0, 0.0, 0.0);
    match warp_function {
        WarpFunction::UniformSphere => {
            let z = 1.0 - 2.0*s;
            let r = Float::max(0.0, 1.0-z*z).sqrt();
            let phi = 2.0 * PI * t;
            let x = r * Float::cos(phi);
            let y = r * Float::sin(phi);
            v = Vec3::new(x, y, z);
        }
        WarpFunction::UniformHemisphere => {
            let z = s;
            let r = Float::max(0.0, 1.0-z*z).sqrt();
            let phi = 2.0 * PI * t;
            let x = r * Float::cos(phi);
            let y = r * Float::sin(phi);
            v = Vec3::new(x, y, z);
        },
        WarpFunction::UniformDisk => {
            let r = s.sqrt();
            let theta = 2.0*PI*t;
            let x = r * Float::cos(theta);
            let y = r * Float::sin(theta);
            v = Vec3::new(x, y, 0.0);
        }
        WarpFunction::ConcentricDisk => {
            let mut rho = 0.0;
            let mut theta = 0.0;
            compute_concentric_map_rho_theta(s, t, &mut rho, &mut theta);
            let x = rho * Float::cos(theta);
            let y = rho * Float::sin(theta);
            v = Vec3::new(x, y, 0.0);
        }
        WarpFunction::CosineHemisphere => {
            v = warp_point(s, t, WarpFunction::ConcentricDisk);
            let x = v.x;
            let y = v.y;
            v.z = Float::max(0.0, 1.0 - x*x - y*y).sqrt();
        },
        WarpFunction::UniformTriangle => {
            let su1 = s.sqrt();
            v = Vec3::new(
                1.0 - su1,
                t * su1,
                0.0)
        }
        _ => {
            v.x = 2.0*s - 1.0;
            v.y = 2.0*t - 1.0;
            v.z = 0.0;
        }
    }
    v
}

fn compute_concentric_map_rho_theta(u1: Float, u2: Float, rho: &mut Float, theta: &mut Float) {
    let sx = 2.0 * u1 - 1.0;
    let sy = 2.0 * u2 - 1.0;
    let r: Float;
    let t: Float;
    if (sx == 0.0 && sy == 0.0) {
        *rho = 0.0;
        *theta = 0.0;
        return;
    }
    if sx >= -sy {
        if sx > sy {
            // Handle first region of the disk
            r = sx;
            if sy > 0.0 {
                t = sy/r;
            } else {
                t = 8.0 + sy/r;
            }
        } else {
            // Handle second region of the disk
            r = sy;
            t = 2.0 - sx/r;
        }
    }
    else {
        if sx <= sy {
            // Handle third region of the disk
            r = -sx;
            t = 4.0 - sy/r;
        } else {
            // Handle fourth region of the disk
            r = -sy;
            t = 6.0 + sx/r;
        }
    }
    *rho = r;
    *theta = t * (PI/4.0);
}