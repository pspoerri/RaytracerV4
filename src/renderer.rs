use types::*;
use scene::Scene;
use ray::Ray;
use hit::HitInfo;

pub struct Renderer<'a> {
    scene: &'a Scene
}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene) -> Renderer
    {
        let renderer = Renderer {
            scene: scene
        };
        renderer
    }
    pub fn render(&self, ray: &mut Ray) -> Color
    {
        let mut c = Color::new(0.0, 0.0, 0.0);        
        match self.intersect(ray) {
            None => {
                return c;
            },
            Some(hit) => {
                c = hit.shape.shade(&hit, &self);
            }
        }
        // ToDo: Fog
        c
    }
    
    pub fn intersect(&self, ray: &mut Ray) -> Option<HitInfo>
    {
        let mut value = None;
        for s in &self.scene.shapes {
            match s.intersect(&ray) {
                None => {},
                Some(hit) => {
                    ray.tmax = hit.d;
                    value = Some(hit);               
                }
            }
        }
        value
    }
}
unsafe impl<'a> Send for Renderer<'a> {}
