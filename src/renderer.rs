use types::*;
use scene::Scene;
use ray::Ray;

pub struct Renderer<'a> {
    scene: &'a Scene
}

impl<'a> Renderer<'a> {
    pub fn new(scene: &'a Scene) -> Renderer<'a>
    {
        let renderer = Renderer {
            scene: scene
        };
        renderer
    }
    pub fn render(&self, ray: &Ray) -> Color
    {
        let mut c = Color::new(0.0, 0.0, 0.0);
        //  for p in &self.primitives {
        //      if (p.intersect(&ray)) {
        //          c.x = 1.0;
        //          c.y = 1.0;
        //          c.z = 1.0; 
        //      }
        //  }
        c
    }
}