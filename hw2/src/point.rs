use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn new_gen(rng: &mut ThreadRng) -> Self {
        Self::new(rng.r#gen(), rng.r#gen())
    }

    #[inline]
    pub fn in_circle(&self) -> bool {
        self.x.powf(2f64) + self.y.powf(2f64) <= 1f64
    }
}
