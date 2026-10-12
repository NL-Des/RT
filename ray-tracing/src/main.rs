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
// rec.p est la surface touchée par le rayon au moment du rebond.
fn direct_light(rec: &HitRecord, world: &dyn Hittable, light: &Light) -> Color {
    let black = Color::new(0.0, 0.0, 0.0);

    let to_center = light.center - rec.p;
    let distance_squared = to_center.length_squared();
    if distance_squared <= light.radius * light.radius {
        return black;
    }

    // Vue depuis le point, la lampe est une sphère.
    // Du point de vue du point éclairé, c'est un cône.
    let cos_max = f64::sqrt(1.0 - light.radius * light.radius / distance_squared);

    // Repère dont l'axe `w` pointe vers le centre de la lampe
    // Il y a un rayon par appel par pixel.
    // Chaque rayon termine dans le cône de lumière de la lampe, mais pas forcément sur le centre.
    let w = vec3::unit_vector(to_center);
    let a = if w.x().abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let v = vec3::unit_vector(vec3::cross(w, a));
    let u = vec3::cross(w, v);

    // Direction au hasard dans le cône.
    let z = 1.0 - common::random_double() * (1.0 - cos_max); // L'écart par rapport à l'axe.
    let phi = 2.0 * common::PI * common::random_double(); // Angle autour de l'axe, de quel côté on s'écarte.
    let s = f64::sqrt(1.0 - z * z); //la part perpendiculaire à l'axe.
    let direction = s * f64::cos(phi) * u + s * f64::sin(phi) * v + z * w; // Assemblage des trois composantes. Et prise en compte de l'éloignement de la cible.

    // Vérification que la Lampe, la Lumière, soit du bon côté de la surface de l'exercice.
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


fn ray_color(
    r: &Ray, // Le rayon à suivre, de son origine à sa destination. Il peut rebondir sur des surfaces, et chaque rebond peut générer un nouveau rayon.
    world: &dyn Hittable, // La scène, tout ce que le rayon peut toucher.
    light: Option<&Light>, // La lamp de la scène.
    ambient: f64, // La luminosité du ciel.
    depth: i32, // Le nombre de rebonds autorisés.
    count_emitted: bool, // Faut-il compter la lumière émise par les surfaces touchées par le rayon, ou seulement la lumière réfléchie ?
) -> Color {
    // Si on a dépassé le nombre de rebonds autorisés, on ne renvoie rien.
    // Sa permet d'initier ombres et reflexions, mais pas de manière infinie.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // C'est le coeur de ray_color : on regarde si le rayon touche un objet de la scène.
    // Si oui, quelle est la couleur qui revient de ce point de contact, en fonction de la lumière reçue et de la matière.
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mat = rec.mat.as_ref().unwrap(); // L'objet est-il brillant ou mat ?
        let emitted = if count_emitted {
            mat.emitted()
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
        // Le rayon va-t-il rebondir sur la surface touchée, ou s'arrêter là ?
        let mut attenuation = Color::default(); // La couleur de la lumière réfléchie par la surface touchée. Combien de lumière est absorbée par la surface, et combien est renvoyée ?
        let mut scattered = Ray::default(); // Le nouveau rayon après rebond.
        // Si le rayon rebondit, on calcule la couleur de la lumière reçue par le nouveau rayon.
        if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            // Surface mate : on vise la lampe, le rebond ne doit plus la compter
            let direct = match light { 
                Some(light) if mat.is_diffuse() => Some(direct_light(&rec, world, light)),
                _ => None,
            };
            // On calcule la couleur de la lumière reçue par le rayon après rebond.
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

// Affiche les instructions d'utilisation du programme, en cas d'erreur dans le terminal.
fn usage() -> String {
    format!(
        "Usage: cargo run --release -- [--scene <nom>] [--width <pixels>] [--samples <n>] > image.ppm\n\
         \x20 --scene    scène à rendre : {} (défaut : all)\n\
         \x20 --width    largeur de l'image, la hauteur suit en 4/3 (défaut : 800, soit 800x600)\n\
         \x20 --samples  nombre de rayons par pixel (défaut : 200)",
        scenes::SCENE_NAMES.join(", ")
    )
}

// Si échec, affiche un message d'erreur et les instructions d'utilisation, puis quitte le programme.
fn fail(message: &str) -> ! {
    eprintln!("{}\n{}", message, usage());
    process::exit(1);
}

// Initialisation de la scène, de la caméra, et génération de l'image.
fn main () {
    // Image
    const ASPECT_RATIO: f64 = 4.0 / 3.0; // Ratio largeur/hauteur de l'image.
    const MAX_DEPTH: i32 = 50; // Nombre de rebonds maximum autorisés pour un rayon.
    let mut image_width: i32 = 800; // Largeur de l'image en pixels.
    let mut samples_per_pixel: i32 = 200; // Nombre de rayons par pixel, pour lisser l'image et réduire le bruit.
    let mut scene_name = String::from("all"); // Nom de la scène à produire, par défaut "all".

    // Arguments de la ligne de commande entrée dans le terminal.
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
    // Calcul de la hauteur de l'image en fonction de la largeur et du ratio.
    let image_height = ((image_width as f64 / ASPECT_RATIO) as i32).max(2);

    // World, Camera
    let scene = match scenes::by_name(&scene_name, ASPECT_RATIO) {
        Some(scene) => scene,
        None => fail(&format!("Scène inconnue : {}", scene_name)),
    };
    let world = scene.world;
    let cam = scene.camera;
    let light = scene.light.as_ref();

    // Écriture de l'en-tête du fichier PPM dans le terminal, pour que le programme puisse être redirigé vers un fichier image.
    print!("P3\n{} {}\n255\n", image_width, image_height); // P3 : format texte, 255 : valeur max d'une couleur.
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
