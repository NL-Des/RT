use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

// plane.rs construit un plan infini et la manière de calculer où un rayon le touche.

// Ce qui constitue un plan.
pub struct Plane {
    point: Point3, // Contient n'importe quel point qui appartenant au plan.
    normal: Vec3, // Contient le vecteur perpendiculaire. En gros permet de donner son orientation, si nous sommes en haut ou en bas par rapport au plan.
    mat: Rc<dyn Material>, // Le matériau qui compose le plan.
}

// Le constructeur du plan.
impl Plane {
    pub fn new(point: Point3, normal: Vec3, m: Rc<dyn Material>) -> Plane {
        Plane {
            point,
            normal: vec3::unit_vector(normal),
            mat: m,
        }
    }
}

// Applique le trait Hittable pour Plane. Afin que qu'il y ait une surface plate infinie,
// sur laquelle repose les objets dans l'image.
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
