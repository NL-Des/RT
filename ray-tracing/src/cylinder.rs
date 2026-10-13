use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// Cylindre fermé d'axe vertical (Y), posé sur le centre de sa base.
pub struct Cylinder {
    base_center: Point3, // Centre du disque de base du cylindre, le cylindre est posé sur ce point.
    radius: f64, // Rayon du cylindre.
    height: f64, // Hauteur du cylindre.
    mat: Rc<dyn Material>, // Le matériaux composant le cylindre.
}

// Implémentation des données dans la struct Cylinder.
impl Cylinder {
    pub fn new(base_center: Point3, radius: f64, height: f64, m: Rc<dyn Material>) -> Cylinder {
        Cylinder {
            base_center,
            radius,
            height,
            mat: m,
        }
    }
}

// Le trait Hittable sert à puorvoir calculer quand un rayon touche l'objet.
// Donc, quand il le fait, pouvoir après récupérer les informations :
// Matière, couleur, position, luminosité, etc...
impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let y_bottom = self.base_center.y();
        let y_top = y_bottom + self.height;

        // Coordonnées du rayon par rapport au cylindre. (Par rapport au centre de la base du cylindre)
        let ox = r.origin().x() - self.base_center.x();
        let oz = r.origin().z() - self.base_center.z();
        // Coordonnées de la direction du rayon.
        let dx = r.direction().x();
        let dy = r.direction().y();
        let dz = r.direction().z();

        // On va chercher le point d'intersection le plus proche du rayon avec le cylindre.
        let mut closest = t_max;
        let mut outward_normal = Vec3::default();
        let mut hit_anything = false;

        // Calculs pour l'intersection avec la surface latérale du cylindre.
        let a = dx * dx + dz * dz;
        if a > 1.0e-12 { // Si le rayon est parallèle à l'axe du cylindre, on ne fait pas le calcul. Pour éviter une division par Zéro.
            let half_b = ox * dx + oz * dz;
            let c = ox * ox + oz * oz - self.radius * self.radius;
            let discriminant = half_b * half_b - a * c;
            if discriminant >= 0.0 {
                let sqrt_d = f64::sqrt(discriminant);
                for root in [(-half_b - sqrt_d) / a, (-half_b + sqrt_d) / a] {
                    if root <= t_min || closest <= root {
                        continue;
                    }
                    let y = r.origin().y() + root * dy;
                    if y < y_bottom || y > y_top {
                        continue;
                    }
                    closest = root;
                    outward_normal =
                        Vec3::new(ox + root * dx, 0.0, oz + root * dz) / self.radius;
                    hit_anything = true;
                }
            }
        }

        // Disques du bas et du haut du cylindre.
        if dy.abs() > 1.0e-12 {
            for (y_cap, normal_y) in [(y_bottom, -1.0), (y_top, 1.0)] {
                let root = (y_cap - r.origin().y()) / dy;
                if root <= t_min || closest <= root {
                    continue;
                }
                let px = ox + root * dx;
                let pz = oz + root * dz;
                if px * px + pz * pz > self.radius * self.radius {
                    continue;
                }
                closest = root;
                outward_normal = Vec3::new(0.0, normal_y, 0.0);
                hit_anything = true;
            }
        }

        // Si on a rien touché, on retourne false.
        if !hit_anything {
            return false;
        }

        rec.t = closest;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
