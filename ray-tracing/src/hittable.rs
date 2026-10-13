use std::rc::Rc;
 
use crate::material::Material;

use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

// Hittable est un trait qui permet de définir les objets qui peuvent êtres touchés par les rayons.
// Il pose les conditions pour savoir si un rayon touche l'objet, et si oui, récupérer les informations sur le point d'intersection.
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3, // Le point d'impact entre le rayon et l'objet.
    pub normal: Vec3, // La normale à la surface de l'objet au point d'impact.
    pub mat: Option<Rc<dyn Material>>, // Le matériau de l'objet touché.
    pub t:f64, // La distance entre le point d'origine du rayon et le point d'impact.
    pub front_face: bool, // Si le rayon touche l'objet par l'extérieur (true) ou par l'intérieur (false).)
}

impl HitRecord {
    // Crée un nouveau HitRecord avec des valeurs par défaut.
    pub fn new() -> HitRecord {
        Default::default()
    }
    // Cette fonction parantit que la normale(outward_normal) stockée pointe toujours contre le rayon, et retiens le côté frappé.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0; // Fait un produit scalaire.
        self.normal = if self.front_face {
            outward_normal // Si positif, la normale pointe vers l'extérieur de l'objet.
        } else {
            -outward_normal // Si négatif, la normale pointe vers l'intérieur de l'objet.
        };
    }
}

pub trait Hittable {
    // ray : le rayon à tester.
    // t_min et t_max : intervalle des distances acceptés, pour éviter qu'une erreur de calcul
    // ne piège un rayon dans un rebond infini sur une surface.
    // rec : indique si une intersection d'un objet est trouvée. En gros, si un objet est touché.
    // bool : true si l'objet touche une intersection.
    fn hit (&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}