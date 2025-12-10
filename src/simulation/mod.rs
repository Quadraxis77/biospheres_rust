pub mod adhesion_inheritance;
pub mod cell_allocation;
pub mod clock;
pub mod cpu_physics;
pub mod cpu_sim;
pub mod double_buffer;
pub mod gpu_physics;
pub mod initial_state;
pub mod nutrient_system;
pub mod physics_config;
pub mod preview_sim;
pub mod synchronized_nutrients;

/// Current simulation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationMode {
    Cpu,
    #[default]
    Preview,
}

/// Global simulation state
pub struct SimulationState {
    pub mode: SimulationMode,
    pub paused: bool,
    pub target_time: Option<f32>,
    pub is_resimulating: bool,
    pub needs_respawn: bool,
    /// Simulation speed multiplier (1.0 = real-time, 10.0 = 10x speed)
    pub speed_multiplier: f32,
    /// Current simulation time
    pub current_time: f32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            mode: SimulationMode::default(),
            paused: false,
            target_time: None,
            is_resimulating: false,
            needs_respawn: false,
            speed_multiplier: 1.0,
            current_time: 0.0,
        }
    }
}
