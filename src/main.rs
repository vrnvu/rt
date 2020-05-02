#[derive(Debug)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn debug(&self) {
        eprintln!("{:?}", self);
    }
    fn write(&self) {
        let i0 = (256.0 * self.0) as i32;
        let i1 = (256.0 * self.1) as i32;
        let i2 = (256.0 * self.2) as i32;
        println!("{} {} {}", i0, i1, i2);
    }
}

fn main() {
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
