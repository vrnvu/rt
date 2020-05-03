use crate::vector;
use vector::Vec3;

#[derive(Debug)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    return Ray { origin, direction };
  }

  pub fn at(&self, t: f32) -> Vec3 {
    return vector::add(self.origin, self.direction * t);
  }
}
