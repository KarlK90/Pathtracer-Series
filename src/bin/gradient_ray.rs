extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use pathtracer::{Ray, Vec3};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let (widht, height, pixels) = trace();

    let window = video_subsystem
        .window("Ray Tracing - In One Weekend", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut picture: Texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, widht, height)
        .map_err(|e| e.to_string())?;

    picture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for (i, pixel) in pixels.iter().enumerate() {
            let offset = i * 3;
            buffer[offset] = (pixel.r() * 255.9) as u8;
            buffer[offset + 1] = (pixel.g() * 255.9) as u8;
            buffer[offset + 2] = (pixel.b() * 255.9) as u8;
        }
    })?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.copy(&picture, None, Some(Rect::new(100, 100, widht * 3, height * 3)))?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn trace() -> (u32, u32, Vec<Vec3>) {
    let width = 200.0;
    let height = 100.0;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut pixels: Vec<Vec3> = Vec::with_capacity(width as usize * height as usize);

    for y in (0..height as i32).rev() {
        for x in 0..width as i32 {
            let u = x as f32 / width;
            let v = y as f32 / height;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            pixels.push(color(&r));
        }
    }

    (width as u32, height as u32, pixels)
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction());
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
