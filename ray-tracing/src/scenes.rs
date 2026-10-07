use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::common;
use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

pub const SCENE_NAMES: [&str; 5] = ["sphere", "plane_cube", "all", "all_alt", "random"];

// Position et taille de la lampe, pour pouvoir la viser depuis les surfaces mates.
pub struct Light {
    pub center: Point3,
    pub radius: f64,
}

pub struct Scene {
    pub world: HittableList,
    pub camera: Camera,
    // Luminosité du ciel : 0.0 = nuit noire, 1.0 = plein jour
    pub ambient: f64,
    // Lampe de la scène, `None` si elle n'est éclairée que par le ciel
    pub light: Option<Light>,
}

pub fn by_name(name: &str, aspect_ratio: f64) -> Option<Scene> {
    match name {
        "sphere" => Some(sphere(aspect_ratio)),
        "plane_cube" => Some(plane_cube(aspect_ratio)),
        "all" => Some(all(aspect_ratio)),
        "all_alt" => Some(all_alt(aspect_ratio)),
        "random" => Some(random(aspect_ratio)),
        _ => None,
    }
}

// Caméra nette (ouverture nulle) placée en `lookfrom` et visant `lookat`.
fn camera(lookfrom: Point3, lookat: Point3, vfov: f64, aspect_ratio: f64) -> Camera {
    let vup = Vec3::new(0.0, 1.0, 0.0);
    Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, 0.0, 1.0)
}

// Sol : plan horizontal passant par l'origine.
fn add_ground(world: &mut HittableList) {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));
}

// Lampe : sphère lumineuse blanche, `intensity` règle sa luminosité.
fn add_light(world: &mut HittableList, position: Point3, intensity: f64) -> Light {
    let light = Light {
        center: position,
        radius: 6.0,
    };
    let light_material = Rc::new(DiffuseLight::new(Color::new(intensity, intensity, intensity)));
    world.add(Box::new(Sphere::new(light.center, light.radius, light_material)));
    light
}

// Scène 1 : une sphère.
fn sphere(aspect_ratio: f64) -> Scene {
    let mut world = HittableList::new();
    add_ground(&mut world);

    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        sphere_material,
    )));

    let light = add_light(&mut world, Point3::new(-6.0, 10.0, 5.0), 2.5);

    Scene {
        world,
        camera: camera(
            Point3::new(0.0, 2.5, 7.0),
            Point3::new(0.0, 0.8, 0.0),
            35.0,
            aspect_ratio,
        ),
        ambient: 0.3,
        light: Some(light),
    }
}

// Scène 2 : un plan et un cube, moins éclairés que la scène 1.
fn plane_cube(aspect_ratio: f64) -> Scene {
    let mut world = HittableList::new();
    add_ground(&mut world);

    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.4, 0.8)));
    world.add(Box::new(Cube::from_center(
        Point3::new(0.0, 0.75, 0.0),
        1.5,
        cube_material,
    )));

    let light = add_light(&mut world, Point3::new(-6.0, 10.0, 5.0), 0.75);

    Scene {
        world,
        camera: camera(
            Point3::new(3.5, 3.0, 6.0),
            Point3::new(0.0, 0.6, 0.0),
            35.0,
            aspect_ratio,
        ),
        ambient: 0.1,
        light: Some(light),
    }
}

// Objets des scènes 3 et 4 : un cube, une sphère, un cylindre et un plan.
fn all_objects() -> (HittableList, Light) {
    let mut world = HittableList::new();
    add_ground(&mut world);

    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.4, 0.8)));
    world.add(Box::new(Cube::from_center(
        Point3::new(-2.5, 0.75, 0.0),
        1.5,
        cube_material,
    )));

    let sphere_material = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        sphere_material,
    )));

    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.7, 0.3)));
    world.add(Box::new(Cylinder::new(
        Point3::new(2.5, 0.0, 0.0),
        0.7,
        1.8,
        cylinder_material,
    )));

    let light = add_light(&mut world, Point3::new(-6.0, 10.0, 5.0), 2.5);

    (world, light)
}

// Scène 3 : tous les objets, vus de face.
fn all(aspect_ratio: f64) -> Scene {
    let (world, light) = all_objects();
    Scene {
        world,
        camera: camera(
            Point3::new(0.0, 3.0, 9.0),
            Point3::new(0.0, 0.8, 0.0),
            35.0,
            aspect_ratio,
        ),
        ambient: 0.3,
        light: Some(light),
    }
}

// Scène 4 : la scène 3 vue depuis une autre position.
fn all_alt(aspect_ratio: f64) -> Scene {
    let (world, light) = all_objects();
    Scene {
        world,
        camera: camera(
            Point3::new(7.0, 5.0, 6.0),
            Point3::new(0.0, 0.8, 0.0),
            35.0,
            aspect_ratio,
        ),
        ambient: 0.3,
        light: Some(light),
    }
}

// Scène finale du guide : beaucoup de petites sphères aléatoires, éclairées par le ciel.
fn random(aspect_ratio: f64) -> Scene {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Scene {
        world,
        camera: Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
        ),
        ambient: 1.0,
        light: None,
    }
}
