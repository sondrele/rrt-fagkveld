use prelude::*;

pub use self::shapes::*;

mod shapes;

pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
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
}
