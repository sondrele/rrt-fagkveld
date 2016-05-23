use std::fmt::Debug;

use vec::Vec3;
use ray::Ray;
use color::Color;

pub trait Intersectable : Debug {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        None
    }

    fn scatter(&self, _: &Ray, _: &Intersection) -> Option<(Color, Ray)> {
        None
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub point: Vec3,

    // Step 3a) Add a surface normal to the intersection, remember to update the 'new'-function
    // pub normal: Vec3,

    // Step 4a) Add a reference to the shape that has been intersected
    // pub shape: &'a Intersectable,
}

impl Intersection {
    pub fn new(t: f64, point: Vec3) -> Intersection {
        Intersection {
            t: t,
            point: point,
        }
    }

    // pub fn new(t: f64, point: Vec3, normal: Vec3, shape: &'a Intersectable) -> Intersection<'a> {
    //     Intersection {
    //         t: t,
    //         point: point,
    //         normal: normal,
    //         shape: shape,
    //     }
    // }
}
