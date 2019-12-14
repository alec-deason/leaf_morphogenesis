use std::env::args;

use leaf_morphogenesis::{render::render, Leaf};

fn main() {
    let mut leaf = Leaf::new();
    for _ in 0..10 {
        leaf.step_simulation(0.1);
    }
    let image = render(&leaf, 500, 500);
    image.write_png(args().nth(1).unwrap()).unwrap();
}
