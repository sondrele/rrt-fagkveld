use prelude::*;

extern crate wavefront_obj;

use self::wavefront_obj::obj::{self, Primitive};
use std::collections::HashMap;

pub struct Mesh {
    pub object: obj::Object,
    pub materials: HashMap<String, Material>,
}

impl Mesh {
    pub fn new(object: obj::Object, materials: HashMap<String, Material>) -> Mesh {
        Mesh {
            object: object,
            materials: materials,
        }
    }
}

impl Intersectable for Mesh {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut closest_so_far: f64 = t_max;

        let &Mesh { ref object, ref materials } = self;
        for geometry in &object.geometry {
            let name: &String = &geometry.material_name.clone().unwrap_or("default".to_string());
            let material = materials.get(name).unwrap();
            for s in &geometry.shapes {
                match s.primitive {
                    Primitive::Triangle(x, y, z) => {
                        let triangle = Triangle::new(object.vertices[x.0],
                                                     object.vertices[y.0],
                                                     object.vertices[z.0]);
                        match intersects(triangle, ray, material, t_min, closest_so_far) {
                            Some(other_intersection) => {
                                closest_so_far = other_intersection.distance;
                                intersection = Some(other_intersection);
                            }
                            None => (),
                        }
                    }
                    _ => panic!("Only Triangle is supported of obj primitives"),
                }
            }
        }
        intersection
    }
}

struct Triangle {
    vertices: [Vec3; 3],
}

impl Triangle {
    fn new(a: obj::Vertex, b: obj::Vertex, c: obj::Vertex) -> Triangle {
        Triangle {
            vertices: [Vec3::new(a.x, a.y, a.z),
                       Vec3::new(b.x, b.y, b.z),
                       Vec3::new(c.x, c.y, c.z)],
        }
    }
}

fn intersects<'a>(triangle: Triangle,
                  ray: &Ray,
                  material: &'a Material,
                  t_min: f64,
                  t_max: f64)
                  -> Option<Intersection<'a>> {
    let p: Vec3 = ray.origin;
    let d: Vec3 = ray.direction;
    let v0: Vec3 = triangle.vertices[0];
    let v1: Vec3 = triangle.vertices[1];
    let v2: Vec3 = triangle.vertices[2];

    let e1: Vec3 = v1 - v0;
    let e2: Vec3 = v2 - v0;

    let h: Vec3 = d.cross(e2);
    let a0: f64 = e1.dot(h);

    if a0 > -0.00000001 && a0 < 0.00000001 {
        return None;
    }

    let f: f64 = 1.0 / a0;
    let s: Vec3 = p - v0;
    let u: f64 = f * s.dot(h);

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q: Vec3 = s.cross(e1);
    let v: f64 = f * d.dot(q);

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // at this stage we can compute t to find out where
    // the intersection point is on the line
    let t: f64 = f * e2.dot(q);

    if t < t_max && t > t_min {
        let intersection_point = ray.point_along_direction(t);
        let v = triangle.vertices[1] - triangle.vertices[0];
        let w = triangle.vertices[2] - triangle.vertices[0];
        let normal = v.cross(w).normalize();
        Some(Intersection::new(t, intersection_point, normal, material))
    } else {
        // this means that there is
        // a line intersection but not a ray intersection
        None
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
}

fn create_intersection<'a>(sphere: &'a Sphere, delta: f64, ray: &Ray) -> Option<Intersection<'a>> {
    let intersection_point = ray.point_along_direction(delta);
    let surface_normal = (intersection_point - sphere.origin) / sphere.radius;
    Some(Intersection::new(delta, intersection_point, surface_normal, &sphere.material))
}
