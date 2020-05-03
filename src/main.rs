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
    let image_width = 800;
    let image_height = 400;
    //basic_image(image_width, image_height);
    basic_gradient(image_width, image_height);
}

fn print_ppm_header(image_width: i32, image_height: i32) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
}

fn ray_color(ray: Ray) -> Vec3 {
    let unit_direction = vector::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.1 + 1.0);
    let start_value = Vec3(1.0, 1.0, 1.0);
    let end_value = Vec3(0.5, 0.7, 1.0);
    return vector::add(start_value * (1.0 - t), end_value * t);
}

fn basic_gradient(image_width: i32, image_height: i32) {
    print_ppm_header(image_width, image_height);
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(8.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 4.0, 0.0);
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
