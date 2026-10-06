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

fn ray_color(r: &Ray, world: &dyn Hittable, ambient: f64, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mat = rec.mat.as_ref().unwrap();
        let emitted = mat.emitted();
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return emitted + attenuation * ray_color(&scattered, world, ambient, depth - 1);
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
                pixel_color += ray_color(&r, &world, scene.ambient, MAX_DEPTH);
            }
            color::write_color(&mut out, pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone.\n");// Indique quand le programme à fini de générer l'image.
}
