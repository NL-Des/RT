fn main () {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Double boucles qui va écrire les pixels de l'image.
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j); // Affiche le nombre de lignes restantes à traiter avant de pouvoir afficher l'image.
        for i in 0..IMAGE_WIDTH {
            // 0 c'est noir, 1 c'est blanc. On va donc faire un dégradé de couleur.
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64; // Red pour rouge.
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64; // Green pour vert.
            let b = 0.25; // Blue pour bleu.

            // Conversion des valeurs de 0-1 à 0-255 pour l'affichage.
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprint!("\nDone.\n");// Indique quand le programme à fini de générer l'image.
}