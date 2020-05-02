macro_rules! debug {
    ($e:expr) => {
        eprintln!("{:?}", $e);
    };
}

mod ray;
mod vector;
use ray::Ray;

fn main() {
    //let ray = Ray::new();
    //debug!(ray);
    basic_image();
}

fn basic_image() {
    let image_width = 400;
    let image_height = 200;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

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
