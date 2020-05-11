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
  Dielectric { ri: f32 },
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

pub fn new_dielectric(ri: f32) -> Material {
  Material::Dielectric { ri: ri }
}

pub fn new_lambertian(r: f32, g: f32, b: f32) -> Material {
  Material::Lambertian {
    albedo: Vec3(r, g, b),
  }
}

pub fn scatter(material: &Material, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  match material {
    Material::Metal { albedo } => scatter_metal(&albedo, &ray, &hit_record),
    Material::Lambertian { albedo } => scatter_lambertian(&albedo, &hit_record),
    Material::Dielectric { ri } => scatter_dielectric(*ri, &ray, &hit_record),
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

fn scatter_lambertian(albedo: &Vec3, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  let scattered_direction = vector::add(hit_record.normal, Vec3::new_random_unit());
  let scattered = Ray::new(hit_record.p, scattered_direction);
  let attenuation = *albedo;
  Some((attenuation, scattered))
}

fn scatter_dielectric(ri: f32, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
  let attenuation = Vec3(1.0, 1.0, 1.0);
  let etai_over_etat = if hit_record.front_face { 1.0 / ri } else { ri };
  let unit_direction = vector::unit_vector(ray.direction);
  let cos_theta = min(vector::dot(&-unit_direction, &hit_record.normal), 1.0);
  let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

  if (etai_over_etat * sin_theta) > 1.0 {
    let reflected = vector::reflect(&unit_direction, &hit_record.normal);
    let scattered = Ray::new(hit_record.p, reflected);
    Some((attenuation, scattered))
  } else {
    let refracted = vector::refract(&unit_direction, &hit_record.normal, etai_over_etat);
    let scattered = Ray::new(hit_record.p, refracted);
    Some((attenuation, scattered))
  }
}

fn min(a: f32, b: f32) -> f32 {
  let epsion = 0.00000000001;
  if a - b < epsion {
    a
  } else {
    b
  }
}
