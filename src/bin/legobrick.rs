extern crate raytracer;
extern crate bmp;

use raytracer::prelude::*;
use bmp::{Image, Pixel};

fn create_camera(options: &raytracer::Options) -> Camera {
    let origin = Vec3::new(-5.5, 5.5, -9.0);
    let view_point = Vec3::new(0.0, 0.0, 0.0);
    let orthogonal_up = Vec3::new(0.0, 1.0, 0.0);
    let vertical_field_of_view = 35.0;
    let aspect_ratio = options.width as f64 / options.height as f64;
    let aperture = 0.0;
    let distance_to_focus = (origin - view_point).length();
    Camera::new(origin,
                view_point,
                orthogonal_up,
                vertical_field_of_view,
                aspect_ratio,
                aperture,
                distance_to_focus)
}

fn main() {
    let options = raytracer::Options {
        width: 600,
        height: 300,
        num_samples: 1,
        environment: bmp::open("imgs/sky.bmp").ok(),
    };
    let camera = create_camera(&options);
    let scene = raytracer::Scene::parse("scenes/legoBrick.obj");

    let pixels = raytracer::trace_scene(&options, &camera, &scene);
    pixel_array_to_image(&options, pixels)
}

fn pixel_array_to_image(options: &raytracer::Options, pixels: Vec<Color>) {
    let &raytracer::Options { width, height, .. } = options;
    let mut image = Image::new(width, height);
    for y in 0..height {
        for x in 0..width {
            image.set_pixel(x, y, to_pixel(pixels[(y * width + x) as usize]));
        }
    }
    let _ = image.save("legobrick.bmp");
}

fn to_pixel(color: Color) -> Pixel {
    Pixel::new((255.99 * color.r) as u8,
               (255.99 * color.g) as u8,
               (255.99 * color.b) as u8)
}
