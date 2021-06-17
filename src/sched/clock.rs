use std::sync::{Arc, RwLock};

pub struct Clock(Arc<RwLock<u128>>);

impl Clock {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(0)))
    }

    pub fn tick(&mut self) {
        let mut w = self.0.write().unwrap();
        *w += 1;
    }

    pub fn time(&self) -> u128 {
        *self.0.read().unwrap()
    }
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
