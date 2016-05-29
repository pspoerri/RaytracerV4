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
    pub fn render(&self, ray: &mut Ray) -> Color
    {
        let mut c = Color::new(0.0, 0.0, 0.0);        
        match self.scene.intersect(ray) {
            None => {
                return c;
            },
            Some(hit) => {
                c = hit.shape.shade(&hit, &self.scene);
            }
        }
        // ToDo: Fog
        c
    }
}