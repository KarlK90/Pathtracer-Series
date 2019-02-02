fn main() {
    let width = 200;
    let height = 100;

    // Final image is encoded in the PPM plaintext format.
    println!("P3 {} {} 255", width, height);

    for y in (0..height).rev() {
        for x in 0..width {
            let red = (x as f32) / (width as f32);
            let green = (y as f32) / (height as f32);
            let blue = 0.2_f32;
            print!(
                "{} {} {} ",
                (red * 255.9) as i32,
                (green * 255.9) as i32,
                (blue * 255.9) as i32
            );
        }
    }
}
