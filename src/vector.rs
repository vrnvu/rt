#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
  pub fn new() -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
  }
  pub fn write(&self) {
    let i0 = (256.0 * self.0) as i32;
    let i1 = (256.0 * self.1) as i32;
    let i2 = (256.0 * self.2) as i32;
    println!("{} {} {}", i0, i1, i2);
  }
}

impl std::ops::Add<f32> for Vec3 {
  type Output = Self;
  fn add(self, scalar: f32) -> Self {
    Vec3(self.0 + scalar, self.1 + scalar, self.2 + scalar)
  }
}

impl std::ops::AddAssign<f32> for Vec3 {
  fn add_assign(&mut self, scalar: f32) {
    self.0 += scalar;
    self.1 += scalar;
    self.2 += scalar;
  }
}

impl std::ops::Mul<f32> for Vec3 {
  type Output = Self;
  fn mul(self, scalar: f32) -> Self {
    Vec3(self.0 + scalar, self.1 + scalar, self.2 + scalar)
  }
}

impl std::ops::MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, scalar: f32) {
    self.0 *= scalar;
    self.1 *= scalar;
    self.2 *= scalar;
  }
}

impl std::ops::Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self {
    Vec3(-self.0, -self.1, -self.2)
  }
}

pub fn add(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub fn sub(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3(a.0 - b.0, a.1 - b.1, a.2 - b.2)
}
