use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

// Plan infini, défini par un point qui lui appartient et sa normale.
pub struct Plane {
    point: Point3,
    normal: Vec3,
    mat: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, m: Rc<dyn Material>) -> Plane {
        Plane {
            point,
            normal: vec3::unit_vector(normal),
            mat: m,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denom = vec3::dot(r.direction(), self.normal);
        // Rayon parallèle au plan : pas d'intersection
        if denom.abs() < 1.0e-8 {
            return false;
        }

        let t = vec3::dot(self.point - r.origin(), self.normal) / denom;
        if t <= t_min || t_max <= t {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
