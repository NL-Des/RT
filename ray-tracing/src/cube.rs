use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// Boîte alignée sur les axes, définie par ses deux coins opposés.
pub struct Cube {
    min: Point3,
    max: Point3,
    mat: Rc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, m: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat: m }
    }

    // Cube de côté `size` centré sur `center`.
    pub fn from_center(center: Point3, size: f64, m: Rc<dyn Material>) -> Cube {
        let half = Vec3::new(size / 2.0, size / 2.0, size / 2.0);
        Cube::new(center - half, center + half, m)
    }
}

fn axis_normal(axis: usize, sign: f64) -> Vec3 {
    match axis {
        0 => Vec3::new(sign, 0.0, 0.0),
        1 => Vec3::new(0.0, sign, 0.0),
        _ => Vec3::new(0.0, 0.0, sign),
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let orig = [r.origin().x(), r.origin().y(), r.origin().z()];
        let dir = [r.direction().x(), r.direction().y(), r.direction().z()];
        let min = [self.min.x(), self.min.y(), self.min.z()];
        let max = [self.max.x(), self.max.y(), self.max.z()];

        // Méthode des "slabs" : on garde l'entrée la plus tardive et la sortie la plus précoce
        let mut t_near = -f64::INFINITY;
        let mut t_far = f64::INFINITY;
        let mut near_normal = Vec3::default();
        let mut far_normal = Vec3::default();

        for axis in 0..3 {
            if dir[axis].abs() < 1.0e-12 {
                // Rayon parallèle à cette paire de faces
                if orig[axis] < min[axis] || orig[axis] > max[axis] {
                    return false;
                }
                continue;
            }

            let mut t0 = (min[axis] - orig[axis]) / dir[axis];
            let mut t1 = (max[axis] - orig[axis]) / dir[axis];
            // Le rayon entre par la face opposée à sa direction
            let mut sign = -1.0;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
                sign = 1.0;
            }

            if t0 > t_near {
                t_near = t0;
                near_normal = axis_normal(axis, sign);
            }
            if t1 < t_far {
                t_far = t1;
                far_normal = axis_normal(axis, -sign);
            }
            if t_near > t_far {
                return false;
            }
        }

        // Find the nearest root that lies in the acceptable range
        let (root, outward_normal) = if t_min < t_near && t_near < t_max {
            (t_near, near_normal)
        } else if t_min < t_far && t_far < t_max {
            (t_far, far_normal)
        } else {
            return false;
        };

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
