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
    pub fn position(&self) -> &Point {
        match self {
            Vertex::Lamina(p) => p,
            Vertex::Margin(p, _) => p,
            Vertex::Vein(p) => p,
        }
    }

    pub fn set_position(&mut self, position: [f32; 2]) {
        match self {
            Vertex::Lamina(p) => *p = position,
            Vertex::Margin(p, _) => *p = position,
            Vertex::Vein(p) => *p = position,
        }
    }
}

pub struct Parameters {
    vein_growth_rate: f32,
}

pub struct Leaf {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<(usize, usize)>,
    pub parameters: Parameters,
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
            parameters: Parameters {
                vein_growth_rate: 1.0,
            },
        }
    }

    pub fn step_simulation(&mut self, delta: f32) {
        self.step_veins(delta);
    }

    fn step_veins(&mut self, delta: f32) {
        // Elongate segments
        for (a, b) in &self.vein_edges() {
            let vertex_a = &self.vertices[*a];
            let vertex_b = &self.vertices[*b];
            let position_a = vertex_a.position();
            let position_b = vertex_b.position();
            let dx = position_b[0] - position_a[0];
            let dy = position_b[1] - position_a[1];
            let orientation = (dy).atan2(dx);
            let growth = self.parameters.vein_growth_rate * delta;
            let new_b = [
                position_b[0] + orientation.cos() * growth,
                position_b[1] + orientation.sin() * growth,
            ];
            self.vertices[*b].set_position(new_b);
        }
    }

    fn vein_edges(&self) -> Vec<(usize, usize)> {
        let mut veins = vec![];
        for (a, b) in &self.edges {
            let vertex_a = self.vertices[*a];
            let vertex_b = self.vertices[*b];
            if let (Vertex::Vein(_), Vertex::Vein(_)) = (vertex_a, vertex_b) {
                veins.push((*a, *b));
            }
        }

        veins
    }
}
