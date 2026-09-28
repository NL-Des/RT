use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

// Le vecteur d'entrée est transformé en couleur.
// out indique où écrire la couleur
// pixel_color c'est la couleur du pixel.
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}