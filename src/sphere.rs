use crate::ray;
use crate::vector;
use ray::Ray;
use vector::Vec3;

#[derive(Debug)]
pub struct HitRecord {
  pub p: Vec3,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new() -> Self {
    HitRecord {
      p: Vec3::new(),
      normal: Vec3::new(),
      t: 0.0,
      front_face: false,
    }
  }
  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = vector::dot(&ray.direction, outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    }
  }
}

#[derive(Debug)]
pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(ox: f32, oy: f32, oz: f32, radius: f32) -> Self {
    Sphere {
      center: Vec3(ox, oy, oz),
      radius: radius,
    }
  }
}

pub trait Hittable {
  fn hit(self, ray: &Ray, tmin: f32, tmax: f32, hit_record: &mut HitRecord) -> bool;
}

impl Hittable for Sphere {
  fn hit(self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
    let oc = vector::sub(&ray.origin, &self.center);
    let a = ray.direction.length_squared();
    let half_b = vector::dot(&oc, &ray.direction);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let temp = (-half_b - root) / a;
      if temp < t_max && temp > t_min {
        hit_record.t = temp;
        hit_record.p = ray.at(temp);
        let outward_normal = vector::sub(&hit_record.p, &self.center) / self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);
        true
      } else {
        let temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
          hit_record.t = temp;
          hit_record.p = ray.at(temp);
          let outward_normal = vector::sub(&hit_record.p, &self.center) / self.radius;
          hit_record.set_face_normal(&ray, &outward_normal);
          true
        } else {
          false
        }
      }
    } else {
      false
    }
  }
}

pub struct World<T: Hittable> {
  hittables: Vec<T>,
}

impl Hittable for World<Sphere> {
  fn hit(self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new();
    let mut hit_anything = false;
    let closest_so_far = t_max;

    for e in self.hittables {
      if e.hit(&ray, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        hit_record = &mut temp_rec;
      }
    }
    hit_anything
  }
}
