use std::env::args;

use leaf_morphogenesis::{
    Leaf,
    render::render,
};

fn main() -> anyhow::Result<()>{
    let leaf = Leaf::new();
    let image = render(&leaf, 500, 500);
    image.write_png(args().nth(1).unwrap())?;
    Ok(())
}
