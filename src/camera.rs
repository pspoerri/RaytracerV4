use nalgebra::{Vector2, Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye, Transpose};
use num_traits::identities::{Zero};
use std::f64;

use types::*;
use ray::Ray;
// use std::ops::Sub;
// use core::marker::Copy;
// 
#[derive(Debug)]
pub struct Camera {
    pub resolution: Vector2<u32>,
    pub aspect: f64,
    pub angle: f64,
    pub near: f64,
    pub far: f64,
    pub position: Pnt3,
    pub position4: Pnt4,
    pub up: Vec3,
    pub front: Vec3,
    world_to_camera: Matrix4<f64>,
    camera_to_world: Matrix4<f64>,
    perspective: Matrix4<f64>,
    screen_to_world: Matrix4<f64>,
    world_to_screen: Matrix4<f64>,
    world_to_ndc: Matrix4<f64>,
    ndc_to_window: Matrix4<f64>
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let mut camera = Camera { 
            resolution: Vector2::new(width, height),
            aspect: (width as f64)/(height as f64),
            angle: 45.0, 
            near: 0.01, 
            far: 10000.0,
            position: Pnt3::new(0.0, -10.0, 0.5),
            position4: Pnt4::new(0.0, 0.0, 0.0, 1.0),
            front: Vec3::new(0.0, 1.0, -0.0),
            up: Vec3::new(0.0, 0.0, -1.0),
            world_to_camera: Matrix4::zero(),
            camera_to_world: Matrix4::zero(),
            perspective: Matrix4::zero(),
            screen_to_world: Matrix4::zero(),
            world_to_screen: Matrix4::zero(),
            world_to_ndc: Matrix4::zero(),
            ndc_to_window: Matrix4::zero()
        };
        // camera.height = 2.0*f64::tan(f64::to_radians(camera.angle));
        camera.position4 = Pnt4::new(camera.position.x, camera.position.y, camera.position.z, 1.0);
        camera.update_camera();
        camera
    }

    pub fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let xd = x as f64;
        let yd = y as f64;
        let pixel_pos = self.screen_to_world*Pnt4::new(xd, yd, 0.0, 1.0 );
        let pixel_pos = pixel_pos/pixel_pos.w;
        let pixel_pos = Pnt3::new(pixel_pos.x, pixel_pos.y, pixel_pos.z);
        let pixel_pos_max = self.screen_to_world*Pnt4::new(xd, yd, 1.0, 1.0);
        let pixel_pos_max = pixel_pos_max/pixel_pos_max.w;
        let pixel_pos_max = Pnt3::new(pixel_pos_max.x, pixel_pos_max.y, pixel_pos_max.z);

        let dir = pixel_pos-self.position;
        let tmin = (pixel_pos-self.position).norm();
        let tmax = (pixel_pos_max-self.position).norm();

        let dir = Vec3::new(dir.x, dir.y, dir.z);
        Ray::new(&self.position, dir, tmin, tmax)
    }

    pub fn update_camera(&mut self) {
        let lookat = self.position+self.front;
        self.world_to_camera = Camera::create_lookat(&self.position, &lookat, &self.up);
        self.camera_to_world = Camera::create_lookat_inv(&self.position, &lookat, &self.up);
        self.perspective = Camera::create_perspective(self.angle, self.aspect, self.near, self.far);
        self.world_to_ndc = self.perspective * self.world_to_camera;
        self.ndc_to_window = Camera::create_viewport(self.resolution.x, self.resolution.y);
        match self.ndc_to_window.inverse() {
            Some(x) => match self.world_to_ndc.inverse() {
                Some(y) => self.screen_to_world = y*x,
                None    => println!("Warning: Could not inverse screen_to_world matrix!")
            },
            None => println!("Warning: Could not inverse ndc_to_window")
        }
        self.world_to_screen = self.ndc_to_window*self.world_to_ndc;
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.resolution.x = width;
        self.resolution.y = height;
        let x = width as f64;
        let y = height as f64;
        self.aspect = x / y;
        self.update_camera();
    }

    pub fn set_position(&mut self, pos: &Pnt3) {
        self.position = pos.clone();
        self.position4.x = pos.x;
        self.position4.y = pos.y;
        self.position4.z = pos.z;

        self.update_camera();
    }
    pub fn set_front(&mut self, front: Vec3) {
        self.front = front;
        self.update_camera();
    }
    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
        self.update_camera();
    }

    // Port of glfrustum
    fn create_frustum(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) -> Matrix4<f64> {
        let inv1 = 1.0/(r-l);
        let inv2 = 1.0/(t-b);
        let inv3 = 1.0/(f-n);
        Matrix4::new(
            (2.0*n)*inv1,          0.0,  (r+l)*inv1,             0.0,
                     0.0, (2.0*n)*inv2,  (t+b)*inv2,             0.0,
                     0.0,          0.0, -(f+n)*inv3, (-2.0*f*n)*inv3,
                     0.0,          0.0,        -1.0,             0.0
        )
    }
    fn create_frustum_inv(l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) -> Matrix4<f64> {
        let inv1 = 1.0/(2.0*n);
        let inv2 = 1.0/(2.0*f*n);
        Matrix4::new(
            (r-l)*inv1,        0.0,         0.0, (r+l)*inv1,
                   0.0, (t-b)*inv1,         0.0, (t+b)*inv1,
                   0.0,        0.0,         0.0,       -1.0,
                   0.0,        0.0, -(f-n)*inv2, (f+n)*inv2
        )
    }
    pub fn create_perspective(angle: f64, aspect: f64, nDist: f64, fDist: f64) -> Matrix4<f64> {
        let wT = f64::tan(f64::to_radians(angle)*0.5)*nDist;
        let wB = -wT;
        let wR = wT*aspect;
        let wL = -wR;
        Camera::create_frustum(wL, wR, wB, wT, nDist, fDist)
    }
    fn create_perspective_inv(angle: f64, aspect: f64, nDist: f64, fDist: f64) -> Matrix4<f64> {
        let wT = f64::tan(f64::to_radians(angle)*0.5)*nDist;
        let wB = -wT;
        let wR = wT*aspect;
        let wL = -wR;
        Camera::create_frustum_inv(wL, wR, wB, wT, nDist, fDist)    
    }

    // Create viewport
    fn create_viewport(width: u32, height: u32) -> Matrix4<f64> {
        let ws = (width as f64)  * 0.5;
        let hs = (height as f64) * 0.5;
        Matrix4::new(
             ws, 0.0, 0.0,  ws,
            0.0,  hs, 0.0,  hs,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0
        )
    }
    fn create_viewport_inv(width: u32, height: u32) -> Matrix4<f64> {
        let ws = 2.0 / (width as f64);
        let hs = 2.0 / (height as f64);
        Matrix4::new(
             ws, 0.0, 0.0,-1.0,
            0.0,  hs, 0.0,-1.0,
            0.0, 0.0, 2.0,-1.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    fn create_lookat(eye: &Pnt3, center: &Pnt3, up: &Vec3) -> Matrix4<f64> {
        let z = (eye.to_vector()-center.to_vector()).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x).normalize();
        let tr = -eye.to_vector();
        Matrix4::new(
            x.x, x.y, x.z, x.dot(&tr),
            y.x, y.y, y.z, y.dot(&tr),
            z.x, z.y, z.z, z.dot(&tr),
            0.0, 0.0, 0.0, 1.0
        )
    }

    fn create_lookat_inv(eye: &Pnt3, center: &Pnt3, up: &Vec3) -> Matrix4<f64> {
        let z = (eye.to_vector()-center.to_vector()).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x).normalize();
        Matrix4::new(
            x.x, y.x, z.x, eye.x,
            x.y, y.y, z.y, eye.y,
            x.z, y.z, z.z, eye.z,
            0.0, 0.0, 0.0, 1.0
        )
    }

}