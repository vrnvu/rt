extern crate rand;
use rand::Rng;

mod camera;
mod material;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use material::Color;
use ray::Ray;
use sphere::Hittable;
use sphere::Sphere;
use sphere::World;
use vector::Vec3;

fn main() {
    let image_width = 400;
    let image_height = 200;
    let samples_per_pixel = 100;
    render(image_width, image_height, samples_per_pixel);
}

fn print_ppm_header(image_width: i32, image_height: i32) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
}

fn ray_color<T: Hittable>(ray: Ray, world: &World<T>, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3::new();
    }
    if let Some(rec) = world.hit(&ray, 0.001, std::f32::INFINITY) {
        if let Some((attenuation, scattered)) = material::scatter(&rec.material, &ray, &rec) {
            return vector::mul(attenuation, ray_color(scattered, &world, depth - 1));
        } else {
            return Vec3::new();
        }
    } else {
        let unit_direction = vector::unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.1 + 1.0);
        let start_value = Vec3(1.0, 1.0, 1.0);
        let end_value = Vec3(0.5, 0.7, 1.0);
        return vector::add(start_value * (1.0 - t), end_value * t);
    }
}

fn render(image_width: i32, image_height: i32, samples_per_pixel: i32) {
    // 0,0,0 is the origin / eye of the camera / pov
    // y axis goes up and x axis goes to the right, into the screen negative z
    // we traverse from the lower left and use two offset vectors (u, v)
    // to move the ray endpoint across the screen.
    print_ppm_header(image_width, image_height);

    let camera = Camera::new();
    let max_depth = 50;
    let world: World<Sphere> = World {
        spheres: vec![
            Sphere::new(0.0, 0.0, -1.0, 0.5, material::new_lambertian(0.7, 0.3, 0.3)),
            Sphere::new(
                0.0,
                -100.5,
                -1.0,
                100.0,
                material::new_lambertian(0.8, 0.8, 0.8),
            ),
            Sphere::new(1.0, 0.0, -1.0, 0.5, material::new_metal(0.8, 0.6, 0.2)),
            Sphere::new(-1.0, 0.0, -1.0, 0.5, material::new_metal(0.8, 0.8, 0.8)),
        ],
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vec3::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random()) / image_width as f32;
                let v = (j as f32 + random()) / image_height as f32;
                let ray = camera.get_ray(u, v);
                // += to base color 0,0,0
                color = vector::add(color, ray_color(ray, &world, max_depth));
            }
            color.write(samples_per_pixel);
        }
    }
}

fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 1.0)
}
