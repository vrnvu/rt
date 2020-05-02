use crate::vector::Vec3;

#[derive(Debug)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: f32,
}

impl Ray {
  pub fn new() -> Self {
    return Ray {
      origin: Vec3(0.0, 0.0, 0.0),
      direction: 0.0,
    };
  }

  pub fn at(&self, t: f32) -> Vec3 {
    return self.origin + self.direction * t;
  }
}
