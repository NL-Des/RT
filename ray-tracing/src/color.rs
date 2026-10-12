use std::io::Write;

use crate::common;
use crate::vec3::Vec3;

pub type Color = Vec3; // La couleur étant composée de trois composantes, on peut la représenter par un vecteur 3D.

// Écriture de la couleur dans le flux de sortie, l'image finale.
// out : la destination de l'écriture.
// pixel_color : la somme des couleurs de tous les rayons lancés à travers le pixel. Attention ! Ce n'est pas la couleur finale du pixel, il faut encore diviser par le nombre de rayons.
// samples_per_pixel : le nombre de rayons par pixel.
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    // On divise la couleur par le nombre de rayons pour obtenir la couleur moyenne du pixel.
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);
 
    // Conversion de la couleur en entier entre 0 et 255, pour l'écriture dans le flux de sortie.
    // On fait avec 0.999 pour éviter d'avoir 256, ce qui n'est pas une couleur valide.
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32, // Clamping pour éviter d'avoir des valeurs en dehors de l'intervalle [0, 255].
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}