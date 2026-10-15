use std::fmt::{Display, Formatter, Result};
// Display comment un objet va être affiché dans println!
    // Formatter c'est le contexte dans lequel Rust va formater l'objet.
        // Rsult c'est le résultat utilisé lors du formatage.
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
// Ce sont les traits qui correspondent aux opérateurs mathématiques.
    // Add (+), AddAssign (+=), Div(/), DivAssign(/=), Mul(*), MulAssign(*=), Neg(-), Sub(-)

use crate::common;

#[derive(Copy, Clone, Default)]
// Le Copy permet de copier les valeurs, plutôt que de les déplacer entre les variables.
// Et éviter de devoir gérer l'emprunt.

// Encapsulation des coordonnées.
pub struct Vec3 {
    e: [f64;3],
}

// Constructeur de vecteur 3D avec des coordonnées x, y, z.
impl Vec3 {
    // Construction d'un nouveau vecteur avec les coordonnées x, y, z.
    pub fn new(x: f64, y:f64, z:f64) -> Vec3 {
        Vec3 { e: [x, y, z]} 
    }
    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}

// Renommage pour faciliter la lisibilité durant les étapes de développements.
pub type Point3 = Vec3;

 
// Affiche  "x y z", le format utilisé pour écrire les pixels dans le fichier PPM.
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
 
// -Vec3, permet d'inverser les composants.
impl Neg for Vec3 {
    type Output = Vec3;
 
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
 
// Vec3 += Vec3, permet d'additionner les composants.
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
// Vec3 *= f64, permet de multiplier les couleurs (atténuation d'un matériau * la couleur reçue)
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
 
// Vec3 /= f64, permet de réduire la couleur.
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}
 
// Vec3 + Vec3, addition simple.
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}
 
// Vec3 - Vec3, soustraction simple.
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 
// Vec3 * Vec3, multiplication simple.
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

// C'est la fonction la plus utilisée du ray tracer.
// Si deux vecteurs pointent dans le même sens, elle sert à calculer les intersections et les angles.
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

// Le produit vectoriel, il renvoie un vecteur perpendiculaire.
// Cela permet à la caméra de construire son repère de vision.
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

// Renvoi un vecteur de longueur 1 qui garde par contre la direction.
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// C'est de l'échantillonnage par rejet. Il y a du tir de rayon aléatoire.
// Si sa rencontre un objet, on garde. Sinon, on recommence.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

// Normalisation d'un point dnas la sphère, ce qui permet d'occasionner le rebond,
// Puis de le faire dans une direction aléatoire.
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

// Comme en haut, mais en 2D, pour créer un effet de flou de profondeur.
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            common::random_double_range(-1.0, 1.0),
            common::random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

// L'effet miroir.
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

// La réfraction (loi de Snell-Descartes). https://fr.wikipedia.org/wiki/Lois_de_Snell-Descartes
// Le rayon réfracté est découpé en deux rebonds :
// Un perpendiculaire à la normale.
// Un le long de la normale.
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}
