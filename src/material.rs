use bmp;
use scatter;
use prelude::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)>;
}

#[derive(Clone)]
pub struct Mat {
    albedo: Color,
    diffusiveness: Option<f64>,
    refraction_index: Option<f64>,
    texture: Option<bmp::Image>,
}

impl Mat {
    pub fn diffusive(albedo: Color) -> Mat {
        Mat {
            albedo: albedo,
            diffusiveness: None,
            refraction_index: None,
            texture: None,
        }
    }

    pub fn reflective(albedo: Color, diffusiveness: f64) -> Mat {
        Mat {
            albedo: albedo,
            diffusiveness: Some(diffusiveness),
            refraction_index: None,
            texture: None,
        }
    }

    pub fn refractive(refraction_index: f64) -> Mat {
        Mat {
            albedo: Color::white(),
            diffusiveness: None,
            refraction_index: Some(refraction_index),
            texture: None,
        }
    }

    pub fn texture(texture: &str) -> Mat {
        Mat {
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

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        scatter::diffusive(self.albedo, intersection)
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        assert!(f <= 1.0 && f >= 0.0);
        Metal {
            albedo: albedo,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        scatter::reflection(self.albedo, self.fuzz, ray, intersection)
    }
}

#[derive(Clone)]
pub struct Dialectric {
    ref_index: f64,
}

impl Dialectric {
    pub fn new(ref_index: f64) -> Dialectric {
        Dialectric { ref_index: ref_index }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        scatter::refraction(self.ref_index, ray, intersection)
    }
}

#[derive(Clone)]
pub struct Texture {
    texture: bmp::Image,
}

impl Texture {
    pub fn new(texture_path: &str) -> Texture {
        Texture { texture: bmp::open(texture_path).unwrap() }
    }
}

impl Material for Texture {
    fn scatter(&self, _: &Ray, intersection: &Intersection) -> Option<(Color, Ray)> {
        scatter::texture(&self.texture, intersection)
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use prelude::*;
    use scatter::*;


    #[test]
    fn should_reflect_ray() {
        let m = Metal::new(Color::new(1.0, 0.0, 0.0), 0.0);

        let (_, scattered) =
            m.scatter(&Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, -1.0, -1.0)),
                         &Intersection::new(1.0,
                                            Vec3::new(0.0, 0.0, -1.0),
                                            Vec3::new(0.0, 0.0, 1.0),
                                            &m))
                .unwrap();

        expect!(scattered.origin[0]).to(be_close_to(0.0));
        expect!(scattered.origin[1]).to(be_close_to(0.0));
        expect!(scattered.origin[2]).to(be_close_to(-1.0));

        expect!(scattered.direction[0]).to(be_close_to(0.0));
        expect!(scattered.direction[1]).to(be_close_to(-0.7071067811865475));
        expect!(scattered.direction[2]).to(be_close_to(0.7071067811865475));
    }

}
