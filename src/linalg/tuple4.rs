
#[derive(Debug, Default)]
pub struct Tuple4(pub f64, pub f64, pub f64, pub f64);

impl Tuple4 {
    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
}