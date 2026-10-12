pub use std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

// Donne le champs de vision de la caméra en degrés et le convertit en radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Génère un nombre aléatoire entre 0 et 1.
// Cela sert à l'anticrénelage et aux rebonds aléatoires des rayons dans la scène.
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

// Génère un nombre aléatoire entre min et max.
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

// Clamp la valeur x entre min et max (Donc entre 0 et 255 pour les couleurs).
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}