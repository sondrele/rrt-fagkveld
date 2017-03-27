use prelude::*;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
            material: material,
        }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let translated_origin = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = translated_origin.dot(ray.direction);
        let c: f64 = translated_origin.dot(translated_origin) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;
        if discriminant > 0.0 {
            let delta = (-b - (b * b - a * c).sqrt()) / a;
            if delta < t_max && delta > t_min {
                return create_intersection(self, delta, ray);
            }

            let delta = (-b + (b * b - a * c).sqrt()) / a;
            if delta < t_max && delta > t_min {
                return create_intersection(self, delta, ray);
            }
            None
        } else {
            None
        }
    }
}

fn create_intersection<'a>(sphere: &'a Sphere, delta: f64, ray: &Ray) -> Option<Intersection<'a>> {
    let intersection_point = ray.point_along_direction(delta);
    let surface_normal = (intersection_point - sphere.origin) / sphere.radius;
    Some(Intersection::new(delta, intersection_point, surface_normal, &sphere.material))
}
