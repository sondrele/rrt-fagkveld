use rand::{self, Rng};
use std::f64;
use bmp;

use ::vec::Vec3;
use ::ray::Ray;
use ::color::Color;
use ::scene::*;

const INTERSECTION_ORIGIN_OFFSET: f64 = 0.00000001;

pub fn diffusive(attenuation: Color, intersection: &Intersection) -> Option<(Color, Ray)> {
    Some((attenuation, scatter_ray(intersection)))
}

pub fn reflection(attenuation: Color,
                  diffusiveness: f64,
                  ray: &Ray,
                  intersection: &Intersection)
                  -> Option<(Color, Ray)> {
    let reflected = reflect(ray.direction, intersection.normal) +
                    diffusiveness * random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    if reflected.dot(intersection.normal) > 0.0 {
        Some((attenuation, Ray::new(origin, reflected)))
    } else {
        None
    }
}

pub fn refraction(refraction_index: f64,
                  ray: &Ray,
                  intersection: &Intersection)
                  -> Option<(Color, Ray)> {
    let attenuation = Color::white();

    let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(intersection.normal) > 0.0 {
        (intersection.normal.invert(),
         refraction_index,
         refraction_index * ray.direction.dot(intersection.normal) / ray.direction.length())
    } else {
        (intersection.normal,
         1.0 / refraction_index,
         -ray.direction.dot(intersection.normal) / ray.direction.length())
    };

    let refracted = refract(ray.direction, outward_normal, ni_over_nt);
    let should_refract = refracted.is_some() &&
                         shlick_approximation(cosine, refraction_index) <
                         rand::thread_rng().next_f64();

    match refracted {
        Some(refracted) if should_refract => {
            let origin = refraction_origin(intersection.intersection_point, outward_normal);
            Some((attenuation, Ray::new(origin, refracted)))
        }
        _ => {
            let origin = reflection_origin(intersection);
            let reflected = reflect(ray.direction, intersection.normal);
            Some((attenuation, Ray::new(origin, reflected)))
        }
    }

}

pub fn texture(texture: &bmp::Image, intersection: &Intersection) -> Option<(Color, Ray)> {
    let d = intersection.normal;
    let u = 0.5 + d.z.atan2(d.x) / (2.0 * f64::consts::PI);
    let v = 0.5 - d.y.asin() / f64::consts::PI;
    let x = ((1.0 - u) * texture.get_width() as f64) as u32;
    let y = (v * texture.get_height() as f64) as u32;
    let bmp::Pixel { r, g, b } = texture.get_pixel(x, y);
    let color = Color::new(r as f64 / 255.99, g as f64 / 255.99, b as f64 / 255.99);
    Some((color, scatter_ray(intersection)))
}

fn scatter_ray(intersection: &Intersection) -> Ray {
    let target = intersection.intersection_point + intersection.normal +
                 random_point_in_unit_sphere();
    let origin = reflection_origin(intersection);
    let direction = (target - origin).normalize();
    Ray::new(origin, direction)
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.next_f64(), rng.next_f64(), rng.next_f64()) -
                Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn reflection_origin(intersection: &Intersection) -> Vec3 {
    intersection.intersection_point + intersection.normal * INTERSECTION_ORIGIN_OFFSET
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    (v - 2.0 * v.dot(n) * n).normalize()
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((ni_over_nt * (uv - n * dt) - n * discriminant.sqrt()).normalize())
    } else {
        None
    }
}

fn refraction_origin(origin: Vec3, normal: Vec3) -> Vec3 {
    origin - normal * INTERSECTION_ORIGIN_OFFSET
}

fn shlick_approximation(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use prelude::*;
    use super::*;

    #[test]
    fn should_reflect_point() {
        let v = Vec3::new(1.0, -2.0, 0.0);

        let r = reflect(v, Vec3::new(0.0, 1.0, 0.0));

        expect!(r[0]).to(be_close_to(0.4472135954999579));
        expect!(r[1]).to(be_close_to(0.8944271909999159));
        expect!(r[2]).to(be_close_to(0.0));
    }

    #[test]
    fn should_refract_point() {
        let v = Vec3::new(0.0, -1.0, -1.0);

        let r = refract(v, Vec3::new(0.0, 1.0, 0.0), 1.0 / 1.5).unwrap();

        expect!(r[0]).to(be_close_to(0.0));
        expect!(r[1]).to(be_close_to(-0.881917103688197));
        expect!(r[2]).to(be_close_to(-0.4714045207910316));
    }

    #[test]
    fn should_calculate_random_point_in_unit_hemisphere() {
        let p = random_point_in_unit_sphere();

        expect!(p.x).to(be_less_than(1.0));
        expect!(p.y).to(be_less_than(1.0));
        expect!(p.z).to(be_less_than(1.0));
    }
}
