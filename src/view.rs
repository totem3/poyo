use std::sync::{Arc, Mutex};

pub trait Renderable {
    fn render(&self);
}

pub struct View {
    objects: Vec<Arc<Mutex<Renderable>>>,
}

impl View {
    pub fn new(objects: Vec<Arc<Mutex<Renderable>>>) -> Self {
        View { objects }
    }
}

impl Renderable for View {
    fn render(&self) {
        for o in &self.objects {
            let o = o.lock().unwrap();
            o.render();
        }
    }
}
