macro_rules! debug {
    ($e:expr) => {
        eprintln!("{:?}", $e);
    };
}

#[derive(Debug)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn write(&self) {
        let i0 = (256.0 * self.0) as i32;
        let i1 = (256.0 * self.1) as i32;
        let i2 = (256.0 * self.2) as i32;
        println!("{} {} {}", i0, i1, i2);
    }
    fn add(a: &Vec3, other: &Vec3) -> Vec3 {
        Vec3(a.0 + other.0, a.1 + other.1, a.2 + other.2)
    }
    fn add_scalar(a: &Vec3, scalar: f32) -> Vec3 {
        Vec3(a.0 + scalar, a.1 + scalar, a.2 + scalar)
    }
}

#[derive(Debug)]
struct Ray {
    origin: Vec3,
    direction: f32,
}

impl Ray {
    fn new() -> Self {
        return Ray {
            origin: Vec3(0.0, 0.0, 0.0),
            direction: 0.0,
        };
    }

    fn at(&self, t: f32) -> Vec3 {
        return Vec3::add_scalar(&self.origin, self.direction * t);
    }
}

fn main() {
    let ray = Ray::new();
    debug!(ray);
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
            let v = Vec3(r, g, b);
            v.write();
        }
    }
}
