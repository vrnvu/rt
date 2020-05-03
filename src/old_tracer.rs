fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
  let oc = vector::sub(&ray.origin, &center);
  let a = ray.direction.length_squared();
  let half_b = vector::dot(&oc, &ray.direction);
  let c = oc.length_squared() - radius * radius;
  let descriminant = half_b * half_b - a * c;
  if descriminant < 0.0 {
    -1.0
  } else {
    (-half_b - descriminant.sqrt()) / a
  }
}

fn full_hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
  // If we solve the quadratic equation
  // 0 no roots, 1 root or 2 roots
  // so we check if the discriminant is bigger than 0
  let oc = vector::sub(&ray.origin, &center);
  let a = vector::dot(&ray.direction, &ray.direction);
  let b = vector::dot(&oc, &ray.direction) * 2.0;
  let c = vector::dot(&oc, &oc) - radius * radius;
  let discriminant = (b * b) - (4.0 * a * c);
  if discriminant < 0.0 {
    -1.0
  } else {
    (-b - discriminant.sqrt()) / (2.0 * a)
  }
}

fn _ray_color(ray: Ray) -> Vec3 {
  let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &ray);
  if t > 0.0 {
    let normal = vector::sub(&ray.at(t), &Vec3(0.0, 0.0, -1.0));
    let normalized = vector::unit_vector(normal);
    return (normalized + 1.0) * 0.5;
  } else {
    // Linearly blend white and blue depending on the height of the y coord
    // after scaling the ray direction to unit length (-1 < y < 1)
    // 0.0 <= t <= 1.0, when t = 1 we want blue, when 0 we want white.
    // In between we want a blend, this forms a linear blend / interpolation
    // or lerp for short.
    // blended_value = (1-t) * start_value + t * end_value
    let unit_direction = vector::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.1 + 1.0);
    let start_value = Vec3(1.0, 1.0, 1.0);
    let end_value = Vec3(0.5, 0.7, 1.0);
    return vector::add(start_value * (1.0 - t), end_value * t);
  }
}
