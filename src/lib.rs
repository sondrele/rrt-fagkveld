#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

extern crate rand;
extern crate bmp;
extern crate rayon;

use std::f64;
use rand::Rng;
use prelude::*;

mod scatter;
mod vec;
mod ray;
mod color;
mod camera;
mod scene;
mod matrix;
mod animate;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use Options;
    pub use ray::Ray;
    pub use vec::Vec3;
    pub use matrix::Matrix4;
    pub use color::Color;
    pub use camera::Camera;
    pub use scene::{Scene, Sphere, Intersectable};
    pub use animate::{animate, Keyframes, Keyframe};
}

pub struct Options {
    pub width: u32,
    pub height: u32,
    pub num_samples: u32,
    pub environment: Option<bmp::Image>,
}

pub fn trace_scene(options: &Options, camera: &Camera, scene: &Scene) -> Vec<Color> {
    let &Options { width, height, num_samples, .. } = options;
    let mut rng = rand::thread_rng();
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let (x_trans, y_trans) = (x as f64, y as f64);
            let mut color = Color::black();
            for _ in 0..num_samples {
                let u = (x_trans + rng.next_f64()) / width as f64;
                let v = ((height as f64 - y_trans - 1.0) + rng.next_f64()) / height as f64;

                let ray = camera.create_ray(u, v);
                color = color + trace_ray_in_scene(&ray, scene, 0, &options.environment);
                // color = panic!("Step 2b) Call the 'trace_ray_in_scene' function with the \
                //                 appropriate parameters");
            }
            color = color / num_samples as f64;
            pixels.push(color.gamma2());
        }
    }
    pixels
}

fn trace_ray_in_scene(ray: &Ray, scene: &Scene, depth: u32, env: &Option<bmp::Image>) -> Color {
    if depth == 50 {
        return Color::black(); // Return black to avoid being stuck with an unlimited recursion
    }
    match scene.intersects(ray, 0.0, f64::MAX) {
        Some(intersection) => {
            match intersection.shape.scatter(ray, &intersection) {
                Some((attenuation, scattered)) => {
                    attenuation * trace_ray_in_scene(&scattered, scene, depth + 1, env)
                }
                None => Color::black(),
            }
        }
        None => gradient(ray, env),
    }
    // panic!("Step 2b) Return a gradient by calling the 'gradient' function, passing the ray as \
    //         parameter")
}

fn gradient(ray: &Ray, env: &Option<bmp::Image>) -> Color {
    if let &Some(ref texture) = env {
        let d = ray.direction;
        let u = 0.5 + d.z.atan2(d.x) / (2.0 * f64::consts::PI);
        let v = 0.5 - d.y.asin() / f64::consts::PI;
        let x = ((1.0 - u) * texture.get_width() as f64) as u32;
        let y = (v * texture.get_height() as f64) as u32;
        let bmp::Pixel { r, g, b } = texture.get_pixel(x, y);
        let color = Color::new(r as f64 / 255.99, g as f64 / 255.99, b as f64 / 255.99);
        color
    } else {
        let t = 0.5 * (ray.direction.normalize().y + 1.0);
        (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
    }
}
