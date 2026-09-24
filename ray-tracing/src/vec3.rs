#[derive(Copy, Clone, Default)]
// Le Copy est présent pour faire du copier-coler
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
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

pub type Point3 = Vec3;