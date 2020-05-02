#[derive(Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
  pub fn write(&self) {
    let i0 = (256.0 * self.0) as i32;
    let i1 = (256.0 * self.1) as i32;
    let i2 = (256.0 * self.2) as i32;
    println!("{} {} {}", i0, i1, i2);
  }
  pub fn add(a: &Vec3, other: &Vec3) -> Vec3 {
    Vec3(a.0 + other.0, a.1 + other.1, a.2 + other.2)
  }
  pub fn add_scalar(a: &Vec3, scalar: f32) -> Vec3 {
    Vec3(a.0 + scalar, a.1 + scalar, a.2 + scalar)
  }
}
