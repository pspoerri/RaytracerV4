use nalgebra::{Vector3, Vector4, Matrix4, Norm, Cross, Dot, Inverse, Eye};
use num_traits::identities::{Zero};
use std::f64;

use types::*;
use ray::Ray;
// use std::ops::Sub;
// use core::marker::Copy;
// 
#[derive(Debug)]
pub struct Camera {
    pub width: f64,
    pub height: f64,
    pub aspect: f64,
    pub angle: f64,
    pub near: f64,
    pub far: f64,
    pub pos: Vec3,
    pub pos4: Vec4,
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
            width: width as f64, 
            height: height as f64, 
            aspect: (width as f64)/(height as f64),
            angle: 45.0, 
            near: 0.001, 
            far: 10000.0,
            pos: -Vec3::new(1.0, 1.0, 1.0),
            pos4: Vec4::new(0.0, 0.0, 0.0, 1.0),
            front: Vec3::new(1.0, 1.0, 1.0),
            up: -Vec3::new(0.0, 0.0, 1.0),
            world_to_camera: Matrix4::zero(),
            camera_to_world: Matrix4::zero(),
            perspective: Matrix4::zero(),
            screen_to_world: Matrix4::zero(),
            world_to_screen: Matrix4::zero(),
            world_to_ndc: Matrix4::zero(),
            ndc_to_window: Matrix4::zero()
        };
        camera.pos4 = Vec4::new(camera.pos.x, camera.pos.y, camera.pos.z, 1.0);
        camera.update_camera();
        camera
    }

    pub fn generate_ray(&self, x: u32, y: u32) -> Ray {
        let xd = x as f64;
        let yd = y as f64;
        let pixel_pos = self.screen_to_world*Vec4::new(xd, yd, 0.0, 1.0 );
        let pixel_pos = pixel_pos/pixel_pos.w;
        let pixel_pos_max = self.screen_to_world*Vec4::new(xd, yd, 1.0, 1.0);
        let pixel_pos_max = pixel_pos_max/pixel_pos_max.w;

        let dir = pixel_pos-pixel_pos_max;
        let tmin = (pixel_pos-self.pos4).norm();
        let tmax = (pixel_pos_max-self.pos4).norm();

        let mut dir = Vector3::new(dir.x, dir.y, dir.z);
        Ray::new(&self.pos, dir, tmin, tmax)
    }

    pub fn print(&self) {
        println!("width: {width} height: {height}", width=self.width, height=self.height);
        println!("apsect: {aspect}", aspect=self.aspect);
        println!("pos: {pos}\nfront: {front}\nup: {up}", pos=self.pos, front=self.front, up=self.up);
        println!("world_to_camera:\n{world_to_camera}", world_to_camera=self.world_to_camera);
        println!("camera_to_world:\n{camera_to_world}", camera_to_world=self.camera_to_world);
        println!("perspective:\n{perspective}", perspective=self.perspective);
        println!("screen_to_world:\n{screen_to_world}", screen_to_world=self.screen_to_world);
        println!("world_to_screen:\n{world_to_screen}", world_to_screen=self.world_to_screen);
        println!("ndc_to_window\n{ndc_to_window}", ndc_to_window=self.ndc_to_window);
    }

    pub fn update_camera(&mut self) {
        self.world_to_camera = Camera::create_lookat(&self.pos, &(self.pos+self.front), &self.up);
        match self.world_to_camera.inverse() {
            Some(x) => self.camera_to_world = x,
            None    => println!("Warning: Could not inverse matrix!")
        }
        self.perspective = Camera::create_perspective(self.angle, self.aspect, self.near, self.far);
        self.world_to_ndc = self.perspective * self.world_to_camera;
        self.ndc_to_window = Camera::create_viewport(self.width, self.height);
        match self.ndc_to_window.inverse() {
            Some(x) => match self.world_to_ndc.inverse() {
                Some(y) => self.screen_to_world = x*y,
                None    => println!("Warning: Could not inverse screen_to_world matrix!")
            },
            None => println!("Warning: Could not inverse ndc_to_window")
        }
        self.world_to_screen = self.ndc_to_window*self.world_to_ndc;
    }
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width as f64;
        self.height = height as f64;
        self.aspect = self.width/self.height;
        self.update_camera();
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.pos4.x = pos.x;
        self.pos4.y = pos.y;
        self.pos4.z = pos.z;

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
    fn create_viewport(width: f64, height: f64) -> Matrix4<f64> {
        let ws = width  * 0.5;
        let hs = height * 0.5;
        Matrix4::new(
             ws, 0.0, 0.0,  ws,
            0.0,  hs, 0.0,  hs,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0
        )
    }
    fn create_inv_viewport(width: f64, height: f64) -> Matrix4<f64> {
        let ws = 2.0 / width;
        let hs = 2.0 / height;
        Matrix4::new(
             ws, 0.0, 0.0,-1.0,
            0.0,  hs, 0.0,-1.0,
            0.0, 0.0, 2.0,-1.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    fn create_lookat(pos: &Vec3, look_at: &Vec3, up: &Vec3) -> Matrix4<f64> {
        let z = (pos.clone()-look_at.clone()).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x).normalize();
        let tr = -pos.clone();
        Matrix4::new(
            x.x, x.y, x.z, x.dot(&tr),
            y.x, y.y, y.z, y.dot(&tr),
            z.x, z.y, z.z, z.dot(&tr),
            0.0, 0.0, 0.0, 1.0
        )
    }

    fn create_lookat_inv(pos: &Vec3, look_at: &Vec3, up: &Vec3) -> Matrix4<f64> {
        let z = (pos.clone()-look_at.clone()).normalize();
        let x = up.cross(&z).normalize();
        let y = z.cross(&x).normalize();
        Matrix4::new(
            x.x, y.x, z.x, pos.x,
            x.y, y.y, z.y, pos.y,
            x.z, y.z, z.z, pos.z,
            0.0, 0.0, 0.0, 1.0
        )
    }

}