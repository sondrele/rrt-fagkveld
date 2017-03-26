use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use wavefront_obj::{obj, mtl};
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

    pub fn parse(path: &str) -> Scene {
        parse(Path::new(path))
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

fn parse(obj_path: &Path) -> Scene {
    let obj_set = obj::parse(read_file(obj_path)).unwrap();

    let materials = obj_set.material_library
        .as_ref()
        .map(|file_name| {
            obj_path.parent()
                .unwrap()
                .join(file_name)
        })
        .map(|file_path| mtl::parse(read_file(&file_path)).unwrap())
        .map(|mtl_set| {
            mtl_set.materials
                .iter()
                .map(|ref material| (material.name.clone(), Material::from_mtl(material)))
                .collect()
        })
        .unwrap_or(default_material());

    let shapes = obj_set.objects
        .iter()
        .map(|obj| Box::new(Mesh::new(obj.clone(), materials.clone())) as Box<Intersectable>)
        .collect();

    Scene::new(shapes)
}

fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}

fn default_material() -> HashMap<String, Material> {
    let mut default_material = HashMap::new();
    default_material.insert(String::from("default"),
                            Material::reflective(Color::new(0.64, 0.64, 0.64), 0.0));
    default_material
}
