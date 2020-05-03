struct HitRecord {
  p: Vec3,
  normal: Vec3,
  t: Vec3,
  front_face: bool,
}

impl HitRecord {
  fn set_face_normal(&mut self, ray: &ray, outward_normal: &Vec3) {
    self.front_face = vector::dot(ray.direction, outward_normal) < 0.0;
    self.normal = if self.front_face {
      outward_normal
    } else {
      -outward_normal
    }
  }
}

trait Hittable {
  fn hit(ray: &Ray, tmin: f32, tmax: f32, hit_record: &HitRecord) -> bool
}

struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Hittable for Sphere {
  fn hit(self, ray: &Ray, tmin: f32, tmax: f32, hit_record: &mut HitRecord) -> bool {
    let oc = ray.origin - self.center;
    let a = ray.direction.length_squared();
    let half_b = vector::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let temp - (-half_b - root) / a;
      if temp < t_max && temp > t_min {
        hit_record.t = temp;
        hit_record.p = ray.at(temp);
        let outward_normal = (hit_record.p - center) / radius;
        hit_record.set_face_normal(&ray, &outward_normal)
        true
      } else {
        let temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
          hit_record.t = temp;
          hit_record.p = ray.at(temp);
          let outward_normal = (hit_record.p - center) / radius;
          hit_record.set_face_normal(&ray, &outward_normal)
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