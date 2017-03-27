use wavefront_obj::obj::{self, Primitive};
use prelude::*;

pub struct Mesh {
    triangles: Vec<Triangle>,
    material: Material,
}

impl Mesh {
    pub fn from(object: &obj::Object, geometry: &obj::Geometry, material: Material) -> Mesh {
        let triangles = geometry.shapes
            .iter()
            .map(|shape| match shape.primitive {
                Primitive::Triangle(x, y, z) => {
                    Triangle::new([Vertex::new(x, object),
                                   Vertex::new(y, object),
                                   Vertex::new(z, object)])
                }
                _ => panic!("Only Triangle is supported of obj primitives"),

            })
            .collect();
        Mesh {
            triangles: triangles,
            material: material,
        }
    }
}

impl Intersectable for Mesh {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut closest_so_far: f64 = t_max;
        for triangle in &self.triangles {
            match intersects(&triangle, ray, &self.material, t_min, closest_so_far) {
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

struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle { vertices: vertices }
    }
}

fn intersects<'a>(triangle: &Triangle,
                  ray: &Ray,
                  material: &'a Material,
                  t_min: f64,
                  t_max: f64)
                  -> Option<Intersection<'a>> {
    let p: Vec3 = ray.origin;
    let d: Vec3 = ray.direction;
    let v0: Vec3 = triangle.vertices[0].coordinate;
    let v1: Vec3 = triangle.vertices[1].coordinate;
    let v2: Vec3 = triangle.vertices[2].coordinate;

    let e1: Vec3 = v1 - v0;
    let e2: Vec3 = v2 - v0;

    let h: Vec3 = d.cross(e2);
    let a0: f64 = e1.dot(h);

    if a0 == 0.0 {
        // > -0.00000001 && a0 < 0.00000001 {
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
        let normal = e1.cross(e2).normalize();
        Some(Intersection::new(t, intersection_point, normal, material))
    } else {
        // this means that there is
        // a line intersection but not a ray intersection
        None
    }
}

struct Vertex {
    coordinate: Vec3,
    normal: Option<Vec3>,
    texture_coordinate: Option<Vec3>,
}

impl Vertex {
    fn new(vtn: obj::VTNIndex, object: &obj::Object) -> Vertex {
        let (vertex_index, texture_index, normal_index) = vtn;
        Vertex {
            coordinate: object.vertices[vertex_index].into(),
            normal: texture_index.map(|index| object.normals[index].into()),
            texture_coordinate: normal_index.map(|index| object.tex_vertices[index].into()),
        }
    }
}

impl Into<Vec3> for obj::Vertex {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<Vec3> for obj::TVertex {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.u,
            y: self.v,
            z: self.w,
        }
    }
}
