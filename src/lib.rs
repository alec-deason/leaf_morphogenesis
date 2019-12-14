pub mod render;

#[derive(Copy, Clone, Debug)]
pub struct Morphogens;
type Point = [f32; 2];

#[derive(Copy, Clone, Debug)]
pub enum Vertex {
    Lamina(Point),
    Margin(Point, Morphogens),
    Vein(Point),
    Convergence(Point, Morphogens),
}

impl Vertex {
    pub fn position(&self) -> &Point {
        match self {
            Vertex::Lamina(p) => p,
            Vertex::Margin(p, _) => p,
            Vertex::Vein(p) => p,
            Vertex::Convergence(p, _) => p,
        }
    }

    pub fn set_position(&mut self, position: [f32; 2]) {
        match self {
            Vertex::Lamina(p) => *p = position,
            Vertex::Margin(p, _) => *p = position,
            Vertex::Vein(p) => *p = position,
            Vertex::Convergence(p, _) => *p = position,
        }
    }
}

pub struct Parameters {
    vein_growth_rate: f32,
    new_vein_proportion: f32,
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
                Vertex::Convergence([0.0, 1.0], Morphogens),
                Vertex::Margin([0.1, 0.1], Morphogens),
                Vertex::Margin([-0.1, 0.1], Morphogens),
            ],
            edges: vec![(0, 1), (0, 2), (0, 2), (0, 3), (2, 1), (3, 1)],
            parameters: Parameters{
                vein_growth_rate: 1.0,
                new_vein_proportion: 0.1,
            },
        }
    }

    pub fn step_simulation(&mut self, delta: f32) {
        self.step_veins(delta);
    }

    fn step_veins(&mut self, delta: f32) {
        let mut tips = vec![];
        // Elongate segments
        for e in &self.vein_edges() {
            let vertex_a = &self.vertices[self.edges[*e].0];
            let vertex_b = &self.vertices[self.edges[*e].1];
            let position_a = vertex_a.position();
            let position_b = vertex_b.position();
            let dx = position_b[0] - position_a[0];
            let dy = position_b[1] - position_a[1];
            let orientation = (dy).atan2(dx);
            if let Vertex::Convergence(_, _) = vertex_b {
                tips.push((*e, dx, dy, orientation, *position_a));
            }
            let growth = self.parameters.vein_growth_rate * delta;
            let new_b = [
                position_b[0] + orientation.cos() * growth,
                position_b[1] + orientation.sin() * growth,
            ];
            self.vertices[self.edges[*e].1].set_position(new_b);
        }

        // Add segments
        for (e, dx, dy, orientation, position_a) in &tips {
            let length = (dx*dx + dy*dy).sqrt();
            let new_edge_length = length * self.parameters.new_vein_proportion;
            let new_x = position_a[0] + orientation.cos()*(length - new_edge_length);
            let new_y = position_a[1] + orientation.sin()*(length - new_edge_length);
            let new_vertex = Vertex::Vein([new_x, new_y]);
            self.split_edge(*e, new_vertex);
        }
    }

    fn vein_edges(&self) -> Vec<usize> {
        let mut veins = vec![];
        for (e, (a, b)) in self.edges.iter().enumerate() {
            let vertex_a = self.vertices[*a];
            let vertex_b = self.vertices[*b];
            match (vertex_a, vertex_b) {
                (Vertex::Vein(_), Vertex::Vein(_)) => veins.push(e),
                (Vertex::Vein(_), Vertex::Convergence(_, _)) => veins.push(e),
                _ => (),
            }
        }

        veins
    }

    pub fn triangles_for_edge(&self, e: usize) -> Vec<[usize; 3]> {
        let (a, b) = self.edges[e];
        let mut potential_triangles = vec![(false, false); self.vertices.len()];
        for (aa, bb) in &self.edges {
            if *aa == a || *aa == b {
                potential_triangles[*bb].0 = true;
            } else if *bb == a || *bb == b {
                potential_triangles[*aa].1 = true;
            }
        }
        let mut triangles = vec![];
        for (c, (a_adjacent, b_adjacent)) in potential_triangles.into_iter().enumerate() {
            if c == a || c == b {
                continue
            }
            if a_adjacent && b_adjacent {
                triangles.push([a, b, c]);
            }
        }
        debug_assert!(triangles.len() == 1 || triangles.len() == 2, "Expected one or two triangles for edge but found {}", triangles.len());
        triangles
    }

    fn split_edge(&mut self, e: usize, new_vertex: Vertex) {
        let triangles = self.triangles_for_edge(e);

        let new_vertex_id = self.vertices.len();
        for triangle in triangles {
            self.edges.push((new_vertex_id, triangle[2]));
        }

        let (a, b) = self.edges[e];
        self.edges.push((a, new_vertex_id));
        self.edges.push((new_vertex_id, b));
        self.vertices.push(new_vertex);
        self.edges.remove(e);
    }
}
