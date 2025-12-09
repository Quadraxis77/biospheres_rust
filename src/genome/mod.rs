pub mod node_graph;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub name: String,
    pub initial_mode: usize,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub color: [f32; 3],
    pub cell_type: u32,
    pub split_mass: f32,
    pub split_interval: f32,
}
