pub mod render;

#[derive(Copy, Clone, Debug)]
pub struct Morphogens;
type Point = [f32; 2];

type VeinId = usize;
type MarginIdx = usize;

#[derive(Copy, Clone, Debug)]
pub struct MarginVertex {
    position: Point,
    morphogens: Morphogens,
}

#[derive(Copy, Clone, Debug)]
pub struct Vein {
    base: VeinVertex,
    child: Option<VeinVertex>,
    tip: VeinVertex,
}

pub struct Parameters {
    vein_growth_rate: f32,
    new_vein_proportion: f32,
}

pub struct Leaf {
    pub margin: Vec<MarginVertex>,
    pub veins: Vec<Vein>,
    pub parameters: Parameters,
}

#[derive(Copy, Clone, Debug)]
enum VeinVertex {
    Convergence(MarginIdx),
    Internal(Point),
}

impl Leaf {
    pub fn new() -> Self {
        Self {
            margin: vec![
                MarginVertex { position: [0.1, 0.0], morphogens: Morphogens },
                MarginVertex { position: [0.0, 1.0], morphogens: Morphogens },
                MarginVertex { position: [-0.1, 0.0], morphogens: Morphogens },
            ],
            veins: vec![Vein { base: VeinVertex::Internal([0.0, 0.0]), tip: VeinVertex::Convergence(1), child: None }],
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
        // Elongate segments
    }
}
