use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

// HittableList définit la scène, la liste des objets que les rayons peuvent toucher.
// Et la logique pour trouver lequel est touché en premier.
 
#[derive(Default)] // Génère de baseune liste avec des vec vide.

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    // Vec : pour héberger une liste dynamique, car les objets n'ont pas tous la même taille.
    // Box : pour héberger des objets de tailles différentes. Qui sont tous des pointeurs vers des objets de type Hittable.
    // dyn : permet de stocker des objets de types différents, avec le même trait (Hittable). C'est du polymorphisme, et autorise la compilation.
}

// Création de la liste aux types multiples (dyn), avec la possibilité d'ajouter des objets à la liste (mut).
impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }
 
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

// Implémentation du trait Hittable pour HittableList, pour pouvoir calculer les intersections avec les rayons.
// La méthode hit parcourt tous les objets de la liste, et retourne true si un objet est touché par le rayon.
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
 
        // Parcours de tous les objets de la liste, pour trouver le plus proche touché par le rayon.
        // Ainsi, on ne parcours pas toute la scène, mais on s'arrête dès qu'on trouve un objet touché par le rayon.
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
 
        hit_anything
    }
}