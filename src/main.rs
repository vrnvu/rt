macro_rules! debug {
    ($e:expr) => {
        eprintln!("{:?}", $e);
    };
}

mod ray;
mod vector;
use ray::Ray;
use vector::Vec3;

fn main() {
    let image_width = 200;
    let image_height = 100;
    //basic_image(image_width, image_height);
    basic_gradient(image_width, image_height);
}

fn print_ppm_header(image_width: i32, image_height: i32) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    // If we solve the quadratic equation
    // 0 no roots, 1 root or 2 roots
    // so we check if the discriminant is bigger than 0
    let oc = vector::sub(&ray.origin, &center);
    let a = vector::dot(&ray.direction, &ray.direction);
    let b = vector::dot(&oc, &ray.direction) * 2.0;
    let c = vector::dot(&oc, &oc) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0;
}

fn ray_color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3(1.0, 0.0, 0.0);
    }
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

fn basic_gradient(image_width: i32, image_height: i32) {
    // 0,0,0 is the origin / eye of the camera / pov
    // y axis goes up and x axis goes to the right, into the screen negative z
    // we traverse from the lower left and use two offset vectors (u, v)
    // to move the ray endpoint across the screen.
    print_ppm_header(image_width, image_height);
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / image_width as f32;
            let v = j as f32 / image_height as f32;
            let direction = vector::add(lower_left_corner, horizontal * u);
            let ray = Ray::new(origin, vector::add(direction, vertical * v));
            let color = ray_color(ray);
            color.write();
        }
    }
}

fn basic_image(image_width: i32, image_height: i32) {
    print_ppm_header(image_width, image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r: f32 = i as f32 / image_width as f32;
            let g: f32 = j as f32 / image_height as f32;
            let b: f32 = 0.2;
            let v = vector::Vec3(r, g, b);
            v.write();
        }
    }
}
