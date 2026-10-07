# RT
- Sujet github de l'exercice : https://github.com/01-edu/public/tree/master/subjects/rt
- Guide pour créer le moteur en rust : https://the-ray-tracing-road-to-rust.vercel.app/
- Extension VSCode pour lire les images directements dans VSCode : https://marketplace.visualstudio.com/items?itemName=ngtystr.ppm-pgm-viewer-for-vscode

## Fonctionnalités

- Génération de quatre objets : sphère, cube, plan et cylindre, chacun placé où l'on veut dans la scène.
- Une caméra libre : position, point visé et angle de champ réglables. Depuis le code.
- Une lumière réglable : une lampe (sphère lumineuse) et la luminosité du ciel. Depuis le code.
- Des ombres, calculées par les rebonds des rayons (path tracing).
- Trois matières : mate (`Lambertian`), métal réfléchissant (`Metal`) et verre réfractant (`Dielectric`).
- Sortie au format PPM (`P3`), résolution choisie sur la ligne de commande.

## Lancer le programme

Prérequis : [Rust et Cargo](https://www.rust-lang.org/tools/install).

Le programme écrit l'image au format PPM sur la sortie standard ; il faut donc la rediriger vers un fichier :

```bash
cd ray-tracing
cargo run --release > ../images/image.ppm
```

Sans option, il rend la scène `all` en 800x600. La progression (`Scanlines remaining`) s'affiche dans le terminal pendant le calcul. Le mode `--release` est indispensable : sans lui, le rendu est beaucoup plus lent.

Les options se placent après `--` :

| Option | Rôle | Défaut |
|---|---|---|
| `--scene <nom>` | Scène à rendre : `sphere`, `plane_cube`, `all`, `all_alt` ou `random` | `all` |
| `--width <pixels>` | Largeur de l'image ; la hauteur suit au format 4/3 | `800` (800x600) |
| `--samples <n>` | Nombre de rayons par pixel : plus il y en a, moins l'image a de grain | `200` |

```bash
# Un exemple pour un aperçu rapide.
cargo run --release -- --scene all --width 240 --samples 100 > ../images/image.ppm

# Une scène plus complexe et longue.
cargo run --release -- --scene random --width 800 --samples 200 > ../images/image.ppm 
```

## Créer une scène

Les scènes sont décrites dans [ray-tracing/src/scenes.rs](ray-tracing/src/scenes.rs). Une scène regroupe un monde (la liste des objets), une caméra et la luminosité du ciel :

```rust
Scene {
    world,              // HittableList : les objets et la lampe
    camera,             // point de vue
    ambient: 0.3,       // luminosité du ciel
    light: Some(light), // la lampe renvoyée par add_light, ou None sans lampe
}
```

Pour ajouter une scène, écrire une fonction qui renvoie un `Scene`, puis l'ajouter dans `by_name` et dans `SCENE_NAMES`. Elle devient alors disponible avec `--scene`.

Le repère : `x` vers la droite, `y` vers le haut, `z` vers la caméra. Le sol des scènes fournies est en `y = 0`.

### Créer les objets

Chaque objet reçoit une matière, par exemple une couleur mate (rouge, vert, bleu entre 0 et 1) :

```rust
let mut world = HittableList::new();
let red = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2)));
```

Sphère : centre, rayon.

```rust
world.add(Box::new(Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, red.clone())));
```

Cube : centre et longueur du côté. `Cube::new(min, max, matière)` donne une boîte quelconque à partir de deux coins opposés. Le cube reste aligné sur les axes.

```rust
world.add(Box::new(Cube::from_center(Point3::new(-2.5, 0.75, 0.0), 1.5, red.clone())));
```

Plan : un point du plan et sa normale (ici un sol horizontal).

```rust
world.add(Box::new(Plane::new(
    Point3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    red.clone(),
)));
```

Cylindre : centre de la base, rayon, hauteur. Son axe est vertical.

```rust
world.add(Box::new(Cylinder::new(Point3::new(2.5, 0.0, 0.0), 0.7, 1.8, red.clone())));
```

Pour déplacer un objet, changer son `Point3`.

Autres matières :

```rust
let metal = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05)); // couleur, flou du reflet (0 = miroir)
let glass = Rc::new(Dielectric::new(1.5));                        // indice de réfraction
```

### Changer la luminosité

Deux réglages, tous deux dans la scène :

1. L'intensité de la lampe, dernier argument de `add_light`. La lampe est une sphère lumineuse ; c'est elle qui projette les ombres.

```rust
let light = add_light(&mut world, Point3::new(-6.0, 10.0, 5.0), 2.5); // position, intensité
```

2. La luminosité du ciel, champ `ambient` du `Scene` : `0.0` pour une nuit noire, `1.0` pour le plein jour. Plus elle est haute, plus les ombres sont claires.

La scène `sphere` utilise une lampe à `2.5` et un ciel à `0.3` ; la scène `plane_cube`, plus sombre, une lampe à `0.75` et un ciel à `0.1`.

### Déplacer la caméra

```rust
camera(
    Point3::new(0.0, 3.0, 9.0), // lookfrom : position de la caméra
    Point3::new(0.0, 0.8, 0.0), // lookat : point visé
    35.0,                       // angle de champ vertical en degrés (plus petit = zoom)
    aspect_ratio,
)
```

Changer `lookfrom` déplace la caméra, changer `lookat` change la direction du regard. Les scènes `all` et `all_alt` ont les mêmes objets et ne diffèrent que par `lookfrom` : `(0, 3, 9)` pour l'une, `(7, 5, 6)` pour l'autre.

Pour un flou de profondeur de champ, appeler directement `Camera::new`, qui prend en plus l'ouverture et la distance de mise au point (voir la scène `random`).
