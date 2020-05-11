use crate::ray;
use crate::sphere;
use crate::vector;
use ray::Ray;
use sphere::HitRecord;
use vector::Vec3;

pub type Color = Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
  Metal { albedo: Vec3 },
  Lambertian { albedo: Vec3 },
}

pub fn default() -> Material {
  Material::Lambertian {
    albedo: Vec3(0.0, 0.0, 0.0),
  }
}

pub fn new_metal(r: f32, g: f32, b: f32) -> Material {
  Material::Metal {
    albedo: Vec3(r, g, b),
  }
}

pub fn new_lambertian(r: f32, g: f32, b: f32) -> Material {
  Material::Lambertian {
    albedo: Vec3(r, g, b),
  }
}

pub fn scatter(material: &Material, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  match material {
    Material::Metal { albedo } => scatter_metal(&albedo, &ray, &hit_record),
    Material::Lambertian { albedo } => scatter_lambertian(&albedo, &ray, &hit_record),
  }
}

fn scatter_metal(albedo: &Vec3, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  let reflected = vector::reflect(&vector::unit_vector(ray.direction), &hit_record.normal);
  let scattered = Ray::new(hit_record.p, reflected);
  let attenuation = *albedo;
  let actual = vector::dot(&scattered.direction, &hit_record.normal);
  if actual > 0.0 {
    Some((attenuation, scattered))
  } else {
    None
  }
}

fn scatter_lambertian(albedo: &Vec3, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  let scattered_direction = vector::add(hit_record.normal, Vec3::new_random_unit());
  let scattered = Ray::new(hit_record.p, scattered_direction);
  let attenuation = *albedo;
  Some((attenuation, scattered))
}
