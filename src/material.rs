use bmp;
use scatter;
use prelude::*;

#[derive(Clone)]
pub struct Material {
    albedo: Color,
    diffusiveness: Option<f64>,
    refraction_index: Option<f64>,
    texture: Option<bmp::Image>,
}

impl Material {
    pub fn diffusive(albedo: Color) -> Material {
        Material {
            albedo: albedo,
            diffusiveness: None,
            refraction_index: None,
            texture: None,
        }
    }

    pub fn reflective(albedo: Color, diffusiveness: f64) -> Material {
        Material {
            albedo: albedo,
            diffusiveness: Some(diffusiveness),
            refraction_index: None,
            texture: None,
        }
    }

    pub fn refractive(refraction_index: f64) -> Material {
        Material {
            albedo: Color::white(),
            diffusiveness: None,
            refraction_index: Some(refraction_index),
            texture: None,
        }
    }

    pub fn texture(texture: &str) -> Material {
        Material {
            albedo: Color::black(),
            diffusiveness: None,
            refraction_index: None,
            texture: Some(bmp::open(texture).unwrap()),
        }
    }

    pub fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        if let Some(diffusiveness) = self.diffusiveness {
            scatter::reflection(self.albedo, diffusiveness, ray, intersection)
        } else if let Some(refraction_index) = self.refraction_index {
            scatter::refraction(refraction_index, ray, intersection)
        } else if let Some(ref texture) = self.texture {
            scatter::texture(texture, intersection)
        } else {
            scatter::diffusive(self.albedo, intersection)
        }
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use prelude::*;

    #[test]
    fn should_reflect_ray() {
        let m = Material::reflective(Color::new(1.0, 0.0, 0.0), 0.0);
        let r = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, -1.0));
        let i = Intersection::new(1.0, Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0), &m);
        let (_, scattered) = m.scatter(&r, &i).unwrap();

        expect!(scattered.origin[0]).to(be_close_to(0.0));
        expect!(scattered.origin[1]).to(be_close_to(0.0));
        expect!(scattered.origin[2]).to(be_close_to(-1.0));

        expect!(scattered.direction[0]).to(be_close_to(0.0));
        expect!(scattered.direction[1]).to(be_close_to(-0.7071067811865475));
        expect!(scattered.direction[2]).to(be_close_to(0.7071067811865475));
    }

}
