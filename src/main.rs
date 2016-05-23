#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(test)]
extern crate hamcrest;

extern crate rand;
extern crate bmp;

use std::f64;

use bmp::Image;
use rand::{ Rng };

use vec::Vec3;
use ray::Ray;
use camera::Camera;
use color::Color;
use scene::{ Scene, Sphere, Intersectable };

mod vec;
mod ray;
mod camera;
mod color;
mod scene;

// This returns a gradiant background for the image
fn gradient(point: Vec3) -> Color {
    let t = 0.5 * (point.y + 1.0);
    (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
}

fn color(ray: &Ray) -> Color {
    gradient(ray.direction.normalize())

    // Step 2c) Update the signature to accept a 'scene: &Scene' paramter and implement according to the pseudo code below
    // if scene.intersects(ray, 0, f64::MAX)
    // then
    //     return black
    // else
    //     return gradient from step 1)
    // hint: you can use a match-expression

    // Step 3c) Use the 'colorize_normal' function to determine the color of the intersection

    // Step 4c) Use the color provided by calling the 'scatter' function on the Intersectable shape
    // referenced by the intersection

    // Step 5) Call the 'color' function recusrively, but use the ray returned from the 'scatter'
    // function this time. Multiply the recursive color with the one returned from the 'scatter' function
}

fn create_camera() -> Camera {
    Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0))
}

fn create_scene() -> Scene {
    panic!("Step 2a) Complete the Scene with the following two Spheres");
    // Sphere { origin: (0.0, 0.0, -1.0), radius: 0.5, color: ( 0.1, 0.2, 0.5) }
    // Sphere { origin: (0.0, -100.5, -1.0), radius: 100.0, color: ( 0.5, 0.8, 0.0) }

    Scene::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Color::new(0.1, 0.2, 0.5)
        )),
        // ...
    ])
}

fn main() {
    let (width, height) = (300, 150);

    let camera = create_camera();

    // Step 2b) Assign a Scene to this variable
    // let scene = ...

    let mut image = Image::new(width, height);
    for (x, y) in image.coordinates() {
        let u = x as f64 / width as f64;
        let v = (height - y - 1) as f64 / height as f64;

        let ray = camera.create_ray(u, v);
        let c: Color = panic!("Step 1d) Use the 'color' function to convert the Ray to a Color");

        // Step 6) Increase the sampling, instead of firing a single Ray from point (u, v), sum the
        // result of firing 100 samples at point (u, v) and normalize the result afterwards by dividing
        // the three color channels of 100

        // Step 7) Add anti aliasing to the picture, this can be done by offsetting the (u, v) point
        // with a random value between [0, 1)

        // use gamma 2 to achieve more natural ligthning
        // achieved by raising the color to the power 1/gamma
        image.set_pixel(x, y, c.gamma2().into());
    }
    let _ = image.save("scene.bmp");
}

fn colorize_normal(dir: Vec3, norm: Vec3, color: Color) -> Color {
    color * (dir.dot(norm) / (dir.length() * norm.length())).abs()
}
