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

    let mut dt = DrawTarget::new(width as i32, height as i32);
    dt.clear(SolidSource {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    });

    for (a, b) in &leaf.edges {
        let mut pb = PathBuilder::new();
        let a = leaf.vertices[*a];
        let b = leaf.vertices[*b];

        let p = a.position();
        pb.move_to(
            (p[0] - leaf_min_x) * scale + x_offset,
            (p[1] - leaf_min_y) * scale + y_offset,
        );
        let p = b.position();
        pb.line_to(
            (p[0] - leaf_min_x) * scale + x_offset,
            (p[1] - leaf_min_y) * scale + y_offset,
        );
        dt.stroke(&pb.finish(), &edge_color, &edge_stroke, &DrawOptions::new());
    }

    dt
}

fn extremes(leaf: &Leaf) -> (f32, f32, f32, f32) {
    let mut min_x = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    let mut min_y = std::f32::MAX;
    let mut max_y = std::f32::MIN;

    for vertex in &leaf.vertices {
        let p = vertex.position();
        if p[0] < min_x {
            min_x = p[0];
        } else if p[0] > max_x {
            max_x = p[0];
        }
        if p[1] < min_y {
            min_y = p[1];
        } else if p[1] > max_y {
            max_y = p[1];
        }
    }
    (min_x, max_x, min_y, max_y)
}
