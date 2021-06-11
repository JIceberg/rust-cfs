pub struct Clock(u128);

impl Clock {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn tick(&mut self) {
        self.0 += 1
    }

    pub fn time(&self) -> u128 {
        self.0
    }
}
