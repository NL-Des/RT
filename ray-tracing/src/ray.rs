use crate::vec3::{Point3, Vec3};

// ray.rs est ici pour définir le rayon.

#[derive(Default)]

pub struct Ray {
    orig: Point3, // Point d'origine du rayon (0, 0, 0).
    dir: Vec3, // Direction du rayon (0, 0, 0).
}

// Construction du rayon, qui part de point d'origine "orig", à destination de "dir".
impl Ray {
    // Création du rayon, copie les informations d'origine.
    pub fn new(origin : Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    // Récupère les coordonnées de l'origine du rayon.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    // Récupère les coordonnées de la direction du rayon.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Permet le déplacement du point de départ (l'origine du rayon), vers où il va (sa direction).
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}