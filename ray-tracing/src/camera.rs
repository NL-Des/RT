use crate::common;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
pub struct Camera {
    origin: Point3, // Position de la caméra dans la scène.
    lower_left_corner: Point3, // Coin inférieur gauche de l'écran virtuel de la caméra dans l'espace 3D. (Point de vue de la caméra, pour le rendu que nous aurons.)
    horizontal: Vec3, // Largeur de l'écran virtuel de la caméra dans l'espace 3D.
    vertical: Vec3, // Hauteur de l'écran virtuel de la caméra dans l'espace 3D.
    u: Vec3, // Direction droite de la caméra dans l'espace 3D.
    v: Vec3, // Direction haute de la caméra dans l'espace 3D.
    lens_radius: f64, //Rayon de la lentille de la caméra, pour simuler la profondeur de champ. Plus le rayon est grand, plus l'effet de flou est important.
}

// Construction de la caméra.
impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        // Calcul de la largeur de l'écran du rendu.
        let theta = common::degrees_to_radians(vfov); 
        let h = f64::tan(theta / 2.0); //
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
 
        // Repères de la caméra dans l'espace 3D.
        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);
        
        // Calcul de la position de la caméra et de l'écran virtuel dans l'espace 3D.
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
 
        let lens_radius = aperture / 2.0;
 
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    // Production d'un rayon aléatoire à partir de la caméra.
    // Le retour permet des rayons, permettent de simuler la profondeur de champ, en fonction de la position du pixel sur l'écran virtuel.
     pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
 
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}