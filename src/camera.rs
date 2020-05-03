use crate::ray;
use crate::vector;
use ray::Ray;
use vector::Vec3;

pub struct Camera {
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  origin: Vec3,
}

impl Camera {
  pub fn new() -> Self {
    Camera {
      lower_left_corner: Vec3(-2.0, -1.0, -1.0),
      horizontal: Vec3(4.0, 0.0, 0.0),
      vertical: Vec3(0.0, 2.0, 0.0),
      origin: Vec3(0.0, 0.0, 0.0),
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    let du = self.horizontal * u;
    let dv = self.vertical * v;
    let xu = vector::add(self.lower_left_corner, du);
    let vxu = vector::add(xu, dv);
    Ray {
      origin: self.origin,
      direction: vector::sub(&vxu, &self.origin),
    }
  }
}
