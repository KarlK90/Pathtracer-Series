extern crate pathtracer;

use pathtracer::Vec3;

fn main() {
    let width = 200.0;
    let height = 100.0;

    // Final image is encoded in the PPM plaintext format.
    println!("P3 {} {} 255", width, height);

    for y in (0..height as i32).rev() {
        for x in 0..width as i32 {
            let col = Vec3::new((x as f32) / width, (y as f32) / height, 0.2);
            print!(
                "{} {} {} ",
                (col.r() * 255.9) as i32,
                (col.g() * 255.9) as i32,
                (col.b() * 255.9) as i32
            );
        }
    }
}
