use std::env::args;

use leaf_morphogenesis::{render::render, Leaf};

fn main() {
    let leaf = Leaf::new();
    let image = render(&leaf, 500, 500);
    image.write_png(args().nth(1).unwrap()).unwrap();
}
