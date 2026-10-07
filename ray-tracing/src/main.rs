mod camera;
mod color;
mod common;
mod cube;
mod cylinder;
mod hittable;
mod hittable_list;
mod material;
mod plane;
mod ray;
mod scenes;
mod sphere;
mod vec3;

use std::io::{self, BufWriter};
use std::process;

use color::Color;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use scenes::Light;
use vec3::Vec3;

// Éclairage direct : au lieu d'attendre qu'un rebond tombe sur la lampe par hasard,
// on tire un rayon vers elle. Renvoie la lumière reçue au point `rec.p`.
fn direct_light(rec: &HitRecord, world: &dyn Hittable, light: &Light) -> Color {
    let black = Color::new(0.0, 0.0, 0.0);

    let to_center = light.center - rec.p;
    let distance_squared = to_center.length_squared();
    if distance_squared <= light.radius * light.radius {
        return black;
    }

    // Vue depuis le point, la lampe est un disque : le cône qui l'entoure
    let cos_max = f64::sqrt(1.0 - light.radius * light.radius / distance_squared);

    // Repère dont l'axe `w` pointe vers le centre de la lampe
    let w = vec3::unit_vector(to_center);
    let a = if w.x().abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let v = vec3::unit_vector(vec3::cross(w, a));
    let u = vec3::cross(w, v);

    // Direction au hasard dans le cône
    let z = 1.0 - common::random_double() * (1.0 - cos_max);
    let phi = 2.0 * common::PI * common::random_double();
    let s = f64::sqrt(1.0 - z * z);
    let direction = s * f64::cos(phi) * u + s * f64::sin(phi) * v + z * w;

    // Lampe derrière la surface
    let cos = vec3::dot(direction, rec.normal);
    if cos <= 0.0 {
        return black;
    }

    // Si un autre objet est touché avant la lampe, il n'émet rien : c'est l'ombre
    let mut light_rec = HitRecord::new();
    if !world.hit(&Ray::new(rec.p, direction), 0.001, common::INFINITY, &mut light_rec) {
        return black;
    }
    light_rec.mat.as_ref().unwrap().emitted() * (cos * 2.0 * (1.0 - cos_max))
}

// `count_emitted` : faux quand la lampe a déjà été comptée par `direct_light`
// au rebond précédent, pour ne pas l'ajouter deux fois.
fn ray_color(
    r: &Ray,
    world: &dyn Hittable,
    light: Option<&Light>,
    ambient: f64,
    depth: i32,
    count_emitted: bool,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mat = rec.mat.as_ref().unwrap();
        let emitted = if count_emitted {
            mat.emitted()
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            // Surface mate : on vise la lampe, le rebond ne doit plus la compter
            let direct = match light {
                Some(light) if mat.is_diffuse() => Some(direct_light(&rec, world, light)),
                _ => None,
            };
            let bounced = ray_color(&scattered, world, light, ambient, depth - 1, direct.is_none());
            return match direct {
                Some(direct) => emitted + attenuation * (direct + bounced),
                None => emitted + attenuation * bounced,
            };
        }
        return emitted;
    }
    // Ciel : sa luminosité dépend du réglage `ambient` de la scène
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    ambient * ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0))
}

fn usage() -> String {
    format!(
        "Usage: cargo run --release -- [--scene <nom>] [--width <pixels>] [--samples <n>] > image.ppm\n\
         \x20 --scene    scène à rendre : {} (défaut : all)\n\
         \x20 --width    largeur de l'image, la hauteur suit en 4/3 (défaut : 800, soit 800x600)\n\
         \x20 --samples  nombre de rayons par pixel (défaut : 200)",
        scenes::SCENE_NAMES.join(", ")
    )
}

fn fail(message: &str) -> ! {
    eprintln!("{}\n{}", message, usage());
    process::exit(1);
}

fn main () {
    // Image
    const ASPECT_RATIO: f64 = 4.0 / 3.0;
    const MAX_DEPTH: i32 = 50;
    let mut image_width: i32 = 800;
    let mut samples_per_pixel: i32 = 200;
    let mut scene_name = String::from("all");

    // Arguments de la ligne de commande
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scene" => match args.next() {
                Some(value) => scene_name = value,
                None => fail("--scene attend un nom de scène."),
            },
            "--width" => match args.next().and_then(|v| v.parse().ok()) {
                Some(value) if value > 1 => image_width = value,
                _ => fail("--width attend un entier supérieur à 1."),
            },
            "--samples" => match args.next().and_then(|v| v.parse().ok()) {
                Some(value) if value > 0 => samples_per_pixel = value,
                _ => fail("--samples attend un entier supérieur à 0."),
            },
            "--help" | "-h" => {
                eprintln!("{}", usage());
                return;
            }
            _ => fail(&format!("Argument inconnu : {}", arg)),
        }
    }
    let image_height = ((image_width as f64 / ASPECT_RATIO) as i32).max(2);

    // World, Camera
    let scene = match scenes::by_name(&scene_name, ASPECT_RATIO) {
        Some(scene) => scene,
        None => fail(&format!("Scène inconnue : {}", scene_name)),
    };
    let world = scene.world;
    let cam = scene.camera;
    let light = scene.light.as_ref();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    let mut out = BufWriter::new(io::stdout().lock());

    // Double boucles qui va écrire les pixels de l'image.
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + common::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + common::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, light, scene.ambient, MAX_DEPTH, true);
            }
            color::write_color(&mut out, pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone.\n");// Indique quand le programme à fini de générer l'image.
}
