use scatter;
use prelude::*;
use bmp;
use std::rc::Rc;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn move_to(&self, vec: Vec3) -> Box<Intersectable>;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub intersection_point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64,
               intersection_point: Vec3,
               normal: Vec3,
               material: &'a Material)
               -> Intersection<'a> {
        Intersection {
            distance: distance,
            intersection_point: intersection_point,
            normal: normal,
            material: material,
        }
    }
}

pub struct Scene {
    pub shapes: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new(shapes: Vec<Box<Intersectable>>) -> Scene {
        Scene { shapes: shapes }
    }
}

impl Intersectable for Scene {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut closest_so_far: f64 = t_max;

        for shape in self.shapes.iter() {
            match shape.intersects(ray, t_min, closest_so_far) {
                Some(other_intersection) => {
                    closest_so_far = other_intersection.distance;
                    intersection = Some(other_intersection);
                }
                None => (),
            }
        }
        intersection
    }

    fn move_to(&self, _: Vec3) -> Box<Intersectable> {
        Box::new(Scene::new(vec![]))
    }
}

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

    fn move_to(&self, vec: Vec3) -> Box<Intersectable> {
        Box::new(Sphere {
            origin: vec,
            radius: self.radius,
            material: self.material.clone()
        })
    }
}

fn create_intersection<'a>(sphere: &'a Sphere, delta: f64, ray: &Ray) -> Option<Intersection<'a>> {
    let intersection_point = ray.point_along_direction(delta);
    let surface_normal = (intersection_point - sphere.origin) / sphere.radius;
    Some(Intersection::new(delta, intersection_point, surface_normal, &sphere.material))
}
