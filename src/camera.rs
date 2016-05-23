use vec::Vec3;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        panic!("Step 1a) Return a new Camera")
    }

    pub fn  create_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            panic!("Step 1c) calculate: lower_left_corner + u * horizontal + v * vertical - origin")
        )
    }
}
