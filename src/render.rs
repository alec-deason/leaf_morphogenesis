use raqote::*;

use crate::Leaf;

pub fn render(leaf: &Leaf, width: u32, height: u32) -> DrawTarget {
    let (leaf_min_x, leaf_max_x, leaf_min_y, leaf_max_y) = extremes(leaf);
    let leaf_width = leaf_max_x - leaf_min_x;
    let leaf_height = leaf_max_y - leaf_min_y;
    let scale = (width as f32 / leaf_width).min(height as f32 / leaf_height);
    let x_offset = (width as f32 - leaf_width * scale) / 2.0;
    let y_offset = (height as f32 - leaf_height * scale) / 2.0;

    let edge_color = Source::Solid(SolidSource {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    });
    let edge_stroke = StrokeStyle {
        width: 1.0,
        ..Default::default()
    };
    let lamina_color = Source::Solid(SolidSource {
        r: 12,
        g: 200,
        b: 80,
        a: 255,
    });

    let mut dt = DrawTarget::new(width as i32, height as i32);
    dt.clear(SolidSource {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });

    let mut pb = PathBuilder::new();
    pb.move_to(
        (leaf.margin[0].position[0] - leaf_min_x) * scale + x_offset,
        (leaf.margin[0].position[1] - leaf_min_y) * scale + y_offset,
    );
    eprintln!("{:?}", leaf.margin[0].position);
    for vertex in &leaf.margin[1..] {
        pb.line_to(
            (vertex.position[0] - leaf_min_x) * scale + x_offset,
            (vertex.position[1] - leaf_min_y) * scale + y_offset,
        );
        eprintln!("{:?}", vertex.position);
    }
    pb.line_to(
        (leaf.margin[0].position[0] - leaf_min_x) * scale + x_offset,
        (leaf.margin[0].position[1] - leaf_min_y) * scale + y_offset,
    );
    dt.fill(&pb.finish(), &lamina_color, &DrawOptions::new());

    dt
}

fn extremes(leaf: &Leaf) -> (f32, f32, f32, f32) {
    let mut min_x = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    let mut min_y = std::f32::MAX;
    let mut max_y = std::f32::MIN;

    for vertex in &leaf.margin {
        let p = vertex.position;
        if p[0] < min_x {
            min_x = p[0];
        }
        if p[0] > max_x {
            max_x = p[0];
        }
        if p[1] < min_y {
            min_y = p[1];
        }
        if p[1] > max_y {
            max_y = p[1];
        }
    }
    eprintln!("{:?}", (min_x, max_x, min_y, max_y));
    (min_x, max_x, min_y, max_y)
}
