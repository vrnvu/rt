#![allow(dead_code)]

extern crate rand;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
  pub fn new() -> Vec3 {
    Vec3(0.0, 0.0, 0.0)
  }
  pub fn new_random_unit() -> Vec3 {
    let a = random_f32(0.0, 2.0 * std::f32::consts::PI);
    let z = random_f32(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
  }
  pub fn write(&self, samples_per_pixel: i32) {
    let scale = 1.0 as f32 / samples_per_pixel as f32;
    let r = (scale * self.0).sqrt();
    let g = (scale * self.1).sqrt();
    let b = (scale * self.2).sqrt();

    let i0 = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let i1 = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let i2 = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    println!("{} {} {}", i0, i1, i2);
  }
  pub fn length(&self) -> f32 {
    (square(self.0) + square(self.1) + square(self.2)).sqrt()
  }
  pub fn length_squared(&self) -> f32 {
    square(self.0) + square(self.1) + square(self.2)
  }
  pub fn abs(&self) -> Self {
    Vec3(self.0.abs(), self.1.abs(), self.2.abs())
  }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
  if x < min {
    return min;
  }
  if x > max {
    return max;
  }
  x
}

fn square(x: f32) -> f32 {
  x * x
}

pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    let p = random(-1.0, 1.0);
    if p.length_squared() < 1.0 {
      return p;
    }
  }
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
  let in_unit_sphere = random_in_unit_sphere();
  if dot(&in_unit_sphere, normal) > 0.0 {
    in_unit_sphere
  } else {
    -in_unit_sphere
  }
}

fn random(min: f32, max: f32) -> Vec3 {
  let mut rng = rand::thread_rng();
  Vec3(
    rng.gen_range(min, max),
    rng.gen_range(min, max),
    rng.gen_range(min, max),
  )
}

fn random_f32(min: f32, max: f32) -> f32 {
  let mut rng = rand::thread_rng();
  rng.gen_range(min, max)
}

impl PartialEq for Vec3 {
  fn eq(&self, other: &Vec3) -> bool {
    let delta = 0.0005;
    if (self.0 - other.0).abs() <= delta
      && (self.1 - other.1).abs() <= delta
      && (self.2 - other.2).abs() <= delta
    {
      true
    } else {
      false
    }
  }
}
impl Eq for Vec3 {}

impl std::ops::Add<f32> for Vec3 {
  type Output = Self;
  fn add(self, scalar: f32) -> Self {
    Vec3(self.0 + scalar, self.1 + scalar, self.2 + scalar)
  }
}

impl std::ops::Sub<f32> for Vec3 {
  type Output = Self;
  fn sub(self, scalar: f32) -> Self {
    Vec3(self.0 - scalar, self.1 - scalar, self.2 - scalar)
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
    Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
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

impl std::ops::Div<f32> for Vec3 {
  type Output = Self;
  fn div(self, scalar: f32) -> Self {
    self * (1.0 / scalar)
  }
}

pub fn add(a: Vec3, b: Vec3) -> Vec3 {
  Vec3(a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

pub fn sub(a: &Vec3, b: &Vec3) -> Vec3 {
  Vec3(a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub fn mul(a: Vec3, b: Vec3) -> Vec3 {
  Vec3(a.0 * b.0, a.1 * b.1, a.2 * b.2)
}

pub fn greater_than(a: Vec3, b: Vec3) -> bool {
  a.0 > b.0 && a.1 > b.1 && a.2 > b.2
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
  vec / vec.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
  u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3(
    u.1 * v.2 - u.2 * v.1,
    u.2 * v.0 - u.0 * v.2,
    u.0 * v.1 - u.1 * v.0,
  )
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
  let d: f32 = dot(v, n);
  let dn = (*n) * d;
  let dn2 = dn * 2.0;
  sub(v, &dn2)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
  let cos_theta = dot(&(-*uv), &n);
  let r_out_parallel = add(*uv, *n * cos_theta) * etai_over_etat;
  let r_out_perp = *n * -(1.0 - r_out_parallel.length_squared()).sqrt();
  add(r_out_parallel, r_out_perp)
}

#[test]
pub fn add_vec() {
  let v0 = Vec3(2.0, 3.0, 1.0);
  let v1 = Vec3(1.0, -1.0, 2.0);
  let actual = add(v0, v1);
  let expected = Vec3(3.0, 2.0, 3.0);
  assert_eq!(actual, expected);
}

#[test]
pub fn add_vec2() {
  let v0 = Vec3(-2.0, -3.5, -1.0);
  let v1 = Vec3(-1.0, -1.5, 2.0);
  let actual = add(v0, v1);
  let expected = Vec3(-3.0, -5.0, 1.0);
  assert_eq!(actual, expected);
}

#[test]
pub fn ops_add() {
  let v0 = Vec3(-2.0, -3.5, -1.0);
  let actual = v0 + 2.0;
  let expected = Vec3(0.0, -1.5, 1.0);
  assert_eq!(actual, expected);
}

#[test]
pub fn ops_mul() {
  let v0 = Vec3(-2.0, -3.5, -1.0);
  let actual = v0 * 2.0;
  let expected = Vec3(-4.0, -7.0, -2.0);
  assert_eq!(actual, expected);
}

#[test]
pub fn ops_add_assign() {
  let mut v0 = Vec3(-2.0, -3.5, -1.0);
  v0 += 2.0;
  let expected = Vec3(0.0, -1.5, 1.0);
  assert_eq!(v0, expected);
}

#[test]
pub fn ops_mul_assign() {
  let mut v0 = Vec3(-2.0, -3.5, -1.0);
  v0 *= 2.0;
  let expected = Vec3(-4.0, -7.0, -2.0);
  assert_eq!(v0, expected);
}

#[test]
pub fn test_square() {
  let actual = square(2.0);
  assert_eq!(4.0, actual);
}

#[test]
pub fn vec_length() {
  let vector = Vec3(1.0, 2.0, 3.0);
  let actual = vector.length();
  assert_eq!(3.7416575, actual);
}

#[test]
pub fn unit_vec() {
  let vector = Vec3(2.0, 3.0, 5.0);
  let actual = unit_vector(vector);
  assert_eq!(Vec3::new(), actual);
}
