use bmp;
use scatter;
use prelude::*;

extern crate wavefront_obj;
use wavefront_obj::mtl;

#[derive(Clone, Debug)]
pub struct Material {
    albedo: Color,
    diffusiveness: Option<f64>,
    refraction_index: Option<f64>,
    texture: Option<bmp::Image>,
}

impl Material {
    pub fn from_mtl(material: &mtl::Material) -> Material {
        // pub specular_coefficient: f64,
        // pub color_ambient: Color,
        // pub color_diffuse: Color,
        // pub color_specular: Color,
        // pub color_emissive: Option<Color>,
        // pub optical_density: Option<f64>,
        // pub alpha: f64,
        // pub illumination: Illumination,
        // pub uv_map: Option<String>,
        Material {
            albedo: Color::new(material.color_diffuse.r, material.color_diffuse.g, material.color_diffuse.b),
            diffusiveness: None,
            refraction_index: None,
            texture: None,
        }
    }

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
            // println!("{:?}", self.albedo);
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
