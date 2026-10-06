# Plan — combler l'écart entre le projet RT et le sujet 01-edu

## Contexte

Le dépôt contient le ray tracer du guide « Ray Tracing Road to Rust » terminé jusqu'à l'étape 13 (`ray-tracing/src/`). Le sujet 01-edu ([sujet](https://github.com/01-edu/public/tree/master/subjects/rt), [audit](https://github.com/01-edu/public/tree/master/subjects/rt/audit)) demande plus que ce que le guide produit. Ce plan liste ce qui manque et l'ordre dans lequel le combler.

Le sujet et l'audit ont été lus en texte brut complet. L'audit comporte 10 questions obligatoires et 4 bonus ; aujourd'hui le projet ne répond « oui » qu'à une question bonus (réflexion / réfraction).

## Réponses actuelles aux questions de l'audit

| # | Question de l'audit | Aujourd'hui | Étape qui corrige |
|---|---|---|---|
| 1 | Scène libre avec au moins un de chaque objet : l'image correspond-elle ? | Non (sphères seulement) | 1, 3 |
| 2 | Peut-on réduire la résolution de l'image ? | Oui, mais en modifiant une constante et en recompilant | 3 |
| 3 | Caméra déplacée : même scène, autre perspective ? | Possible dans le code, impossible avec `random_scene` (scène aléatoire différente à chaque rendu) | 3 |
| 4 | 4 images .ppm fournies ? | Non | 4 |
| 5 | Une image avec une sphère ? | Non | 3, 4 |
| 6 | Une image plan + cube, moins lumineuse que celle de la sphère ? | Non | 1, 2, 4 |
| 7 | Une image avec un cube, une sphère, un cylindre et un plan ? | Non | 1, 4 |
| 8 | La même scène depuis une autre position de caméra ? | Non | 3, 4 |
| 9 | Ombres visibles sur toutes les images ? | Non vérifiable (pas d'images) | 2, 4 |
| 10 | Documentation claire (créer des éléments, changer la luminosité, déplacer la caméra) ? | Non | 5 |
| B1 | Textures ? | Non | 6 |
| B2 | Objets réfléchissants / réfractants ? | Oui (`Metal`, `Dielectric`) | — |
| B3 | Particules ? | Non | hors périmètre |
| B4 | Fluides ? | Non | hors périmètre |

Points du sujet absents de l'audit mais à respecter : toutes les images en 800x600 ; position de chaque objet réglable avant le rendu ; bonus de préférence derrière un drapeau de ligne de commande (ex. `-t` pour les textures) ; l'auditeur doit pouvoir utiliser le programme lui-même, donc rendu raisonnablement rapide.

## État des lieux

| Exigence du sujet / de l'audit | État | Constat |
|---|---|---|
| Sphère | Fait | `sphere.rs` |
| Plan | Manquant | le sol est une sphère de rayon 1000 |
| Cube | Manquant | |
| Cylindre | Manquant | |
| Caméra déplaçable (position, angle) | Fait | `Camera::new(lookfrom, lookat, vup, vfov, …)` dans `camera.rs` |
| Lumière à luminosité réglable | Manquant | seule lumière : le dégradé de ciel codé en dur dans `ray_color` (`main.rs`) |
| Ombres visibles sur toutes les images | Partiel | ombres diffuses dues au ciel uniquement, pas de source de lumière qui projette une ombre nette |
| Sortie PPM P3, max 255 | Fait | `main.rs` + `color::write_color` |
| Résolution facile à changer | Partiel | constantes `IMAGE_WIDTH` / `ASPECT_RATIO` à recompiler |
| Rendu final en 800x600 | Manquant | actuellement 1200x800 (ratio 3/2) |
| 4 images PPM livrées | Manquant | une seule scène (`random_scene`), et `image.ppm` est ignoré par git |
| Documentation : créer chaque objet, changer la luminosité, déplacer la caméra, avec exemples de code | Manquant | le README n'explique que le lancement |
| Bonus réflexion / réfraction | Fait | `Metal`, `Dielectric` dans `material.rs` |
| Bonus textures, particules, fluides | Manquant | optionnels |

## Étapes

### 1. Nouvelles formes (`ray-tracing/src/`)
Chaque forme implémente le trait `Hittable` de `hittable.rs` et suit le modèle de `sphere.rs` (constructeur `new(..., Rc<dyn Material>)`, `rec.set_face_normal`).
- `plane.rs` : `Plane { point, normal, mat }`, intersection `t = dot(point - origin, n) / dot(dir, n)`.
- `cube.rs` : `Cube { min, max, mat }` (boîte alignée sur les axes, méthode des « slabs »), plus un constructeur `from_center(center, size, mat)`. La normale est celle de la face touchée.
- `cylinder.rs` : `Cylinder { base_center, radius, height, mat }` d'axe Y, corps (équation du second degré en x/z, bornée en y) et deux disques de fermeture.
- Déclarer les modules dans `main.rs`.

### 2. Lumière et luminosité
- `material.rs` : ajouter `fn emitted(&self) -> Color` au trait `Material` (noir par défaut) et un matériau `DiffuseLight { emit: Color }` dont l'intensité est le réglage de luminosité.
- `main.rs` : `ray_color` reçoit une couleur de fond `background: Color` (ciel multiplié par un facteur de luminosité ambiante) et renvoie `emitted + attenuation * ray_color(...)`.
- Les scènes placent une sphère lumineuse en hauteur avec un fond sombre : c'est elle qui produit des ombres nettes, et la luminosité se règle par son intensité et par le facteur ambiant.

### 3. Scènes et ligne de commande
- Nouveau `scenes.rs` : une fonction par image demandée, qui renvoie le monde, la caméra et la luminosité.
  1. `sphere` : une sphère sur un plan.
  2. `plane_cube` : plan + cube, luminosité plus faible que la scène 1.
  3. `all` : cube, sphère, cylindre, plan.
  4. `all_alt` : même monde que 3, autre `lookfrom` / `lookat`.
- `main.rs` : lire `std::env::args` (sans dépendance supplémentaire) : `--scene <nom>`, `--width <px>` (défaut 800), `--samples <n>`. Ratio 4/3 pour obtenir 800x600. `random_scene` reste disponible sous `--scene random`.
- Baisser les valeurs par défaut (échantillons, profondeur) pour qu'un rendu 800x600 reste raisonnable.

### 4. Images livrées
- Générer `images/sphere.ppm`, `images/plane_cube.ppm`, `images/all.ppm`, `images/all_alt.ppm` en 800x600.
- `.gitignore` n'ignore que `ray-tracing/image.ppm` et `ray-tracing/src/image.ppm` : le dossier `images/` sera bien suivi par git.

### 5. Documentation (`README.md`)
Compléter le README existant avec :
- les fonctionnalités du ray tracer ;
- un exemple de code pour créer chaque objet (sphère, cube, plan, cylindre) ;
- comment changer la luminosité (intensité de `DiffuseLight`, facteur ambiant) ;
- comment déplacer la caméra (`lookfrom`, `lookat`, `vfov`) ;
- les options de ligne de commande, dont le changement de résolution ;
- corriger le paragraphe actuel qui mentionne 1200x800 et 500 échantillons.

### 6. Bonus (facultatif, après le reste)
Réflexion et réfraction sont déjà là. Une texture damier sur le plan est le bonus restant le moins coûteux, à activer par un drapeau `-t` comme le suggère le sujet. Particules et fluides : hors périmètre sauf demande.

## Vérification
- `cargo build --release` sans avertissement dans `ray-tracing/`.
- Aperçu rapide de chaque scène : `cargo run --release -- --scene <nom> --width 200 --samples 20 > /tmp/x.ppm`, puis contrôle visuel avec l'extension PPM de VSCode.
- Pour chaque image : forme correcte des objets, ombre visible, scène 2 plus sombre que scène 1, scènes 3 et 4 identiques hormis le point de vue.
- En-tête des 4 fichiers finaux : `P3`, `800 600`, `255`.
- Rejouer l'audit : rendre la scène `all` à basse résolution, puis `all_alt`, et répondre aux 10 questions du tableau ci-dessus ; toutes doivent passer à « oui ».
