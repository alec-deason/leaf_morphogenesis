pub mod render;

#[derive(Copy, Clone, Debug)]
pub struct Morphogens;
type Point = [f32; 2];

#[derive(Copy, Clone, Debug)]
pub enum Vertex {
    Lamina(Point),
    Margin(Point, Morphogens),
    Vein(Point),
}

impl Vertex {
    pub fn location(&self) -> Point {
        match self {
            Vertex::Lamina(p) => *p,
            Vertex::Margin(p, _) => *p,
            Vertex::Vein(p) => *p,
        }
    }
}

pub struct Leaf {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<(usize, usize)>,
}

impl Leaf {
    pub fn new() -> Self {
        Self {
            vertices: vec![
                Vertex::Vein([0.0, 0.0]),
                Vertex::Vein([0.0, 1.0]),
                Vertex::Margin([0.1, 0.1], Morphogens),
                Vertex::Margin([0.0, 1.0], Morphogens),
                Vertex::Margin([-0.1, 0.1], Morphogens),
                Vertex::Margin([0.0, 1.0], Morphogens),
            ],
            edges: vec![(0, 1), (0, 2), (0, 4), (1, 3), (1, 5), (2, 3), (4, 5)],
        }
    }
}
