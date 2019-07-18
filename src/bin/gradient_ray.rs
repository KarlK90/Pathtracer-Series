extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Texture;

use pathtracer::{Ray, Vec3};

const WIDTH : u32 = 200;
const HEIGHT  : u32 = 100;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let pixels = trace(WIDTH, HEIGHT);

    let window = video_subsystem
        .window("Ray Tracing - In One Weekend", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut picture: Texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .map_err(|e| e.to_string())?;

    picture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
        for (i, pixel) in pixels.iter().enumerate() {
            let offset = i * 3;
            buffer[offset] = (pixel.r() * 255.9) as u8;
            buffer[offset + 1] = (pixel.g() * 255.9) as u8;
            buffer[offset + 2] = (pixel.b() * 255.9) as u8;
        }
    })?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.copy(&picture, None, Some(Rect::new(0, 0, WIDTH, HEIGHT)))?;
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
    }

    Ok(())
}

fn trace(width: u32, height: u32) -> Vec<Vec3> {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    (0..(height * width))
        .map(|p| {
            let y = height - (p / width);
            let x = p - (p / width) * width;
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            color(&r)
        })
        .collect()
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = Vec3::unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
