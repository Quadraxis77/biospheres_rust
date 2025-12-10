use serde::{Deserialize, Serialize};

/// Represents a 3D vector for colors and positions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Represents a quaternion for rotations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
}

/// Settings for adhesion connections between cells
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdhesionSettings {
    pub can_break: bool,
    pub break_force: f32,
    pub rest_length: f32,
    pub linear_spring_stiffness: f32,
    pub linear_spring_damping: f32,
    pub orientation_spring_stiffness: f32,
    pub orientation_spring_damping: f32,
    pub max_angular_deviation: f32,
    pub enable_twist_constraint: bool,
    pub twist_constraint_stiffness: f32,
    pub twist_constraint_damping: f32,
}

impl Default for AdhesionSettings {
    fn default() -> Self {
        Self {
            can_break: false,
            break_force: 10.0,
            rest_length: 2.0,
            linear_spring_stiffness: 50.0,
            linear_spring_damping: 2.0,
            orientation_spring_stiffness: 10.0,
            orientation_spring_damping: 1.0,
            max_angular_deviation: 0.0,
            enable_twist_constraint: false,
            twist_constraint_stiffness: 1.0,
            twist_constraint_damping: 0.5,
        }
    }
}

/// Settings for child cells after division
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildSettings {
    pub mode_number: i32,
    pub orientation: Quat,
    pub keep_adhesion: bool,
    pub enable_angle_snapping: bool,
}

impl Default for ChildSettings {
    fn default() -> Self {
        Self {
            mode_number: 0,
            orientation: Quat::IDENTITY,
            keep_adhesion: false,
            enable_angle_snapping: false,
        }
    }
}

/// Complete settings for a cell mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeSettings {
    pub name: String,
    pub default_name: String,
    pub cell_type: i32,
    pub color: Vec3,
    pub opacity: f32,
    pub emissive: f32,
    
    // Split settings
    pub split_mass: f32,
    pub split_mass_min: Option<f32>,
    pub split_interval: f32,
    pub split_interval_min: Option<f32>,
    pub split_ratio: f32,
    pub max_splits: i32,
    pub mode_a_after_splits: i32,
    pub mode_b_after_splits: i32,
    
    // Growth settings
    pub nutrient_gain_rate: f32,
    pub max_cell_size: f32,
    pub nutrient_priority: f32,
    pub prioritize_when_low: bool,
    
    // Flagellocyte settings
    pub swim_force: f32,
    
    // Split direction
    pub parent_split_direction: Vec3,
    pub enable_parent_angle_snapping: bool,
    
    // Adhesion settings
    pub max_adhesions: i32,
    pub min_adhesions: i32,
    pub parent_make_adhesion: bool,
    pub adhesion_settings: AdhesionSettings,
    
    // Child settings
    pub child_a: ChildSettings,
    pub child_b: ChildSettings,
}

impl ModeSettings {
    pub fn new_self_splitting(mode_number: i32, name: String) -> Self {
        Self {
            name: name.clone(),
            default_name: name,
            cell_type: 0, // Test cell
            color: Vec3::new(0.5, 0.7, 1.0),
            opacity: 1.0,
            emissive: 0.0,
            
            split_mass: 2.0,
            split_mass_min: None,
            split_interval: 10.0,
            split_interval_min: None,
            split_ratio: 0.5,
            max_splits: -1,
            mode_a_after_splits: -1,
            mode_b_after_splits: -1,
            
            nutrient_gain_rate: 0.1,
            max_cell_size: 1.0,
            nutrient_priority: 1.0,
            prioritize_when_low: true,
            
            swim_force: 0.5,
            
            parent_split_direction: Vec3::new(0.0, 0.0, 0.0),
            enable_parent_angle_snapping: false,
            
            max_adhesions: 10,
            min_adhesions: 0,
            parent_make_adhesion: false,
            adhesion_settings: AdhesionSettings::default(),
            
            child_a: ChildSettings {
                mode_number,
                ..Default::default()
            },
            child_b: ChildSettings {
                mode_number,
                ..Default::default()
            },
        }
    }
}

/// Complete genome data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeData {
    pub name: String,
    pub initial_mode: i32,
    pub modes: Vec<ModeSettings>,
}

impl Default for GenomeData {
    fn default() -> Self {
        Self {
            name: "Default Genome".to_string(),
            initial_mode: 0,
            modes: vec![ModeSettings::new_self_splitting(0, "Mode 0".to_string())],
        }
    }
}

impl GenomeData {
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let genome = serde_json::from_str(&json)?;
        Ok(genome)
    }
}

/// Current genome state resource
pub struct CurrentGenome {
    pub genome: GenomeData,
    pub selected_mode_index: i32,
    pub show_mode_glow: bool,
    pub show_genome_graph: bool,
}

impl Default for CurrentGenome {
    fn default() -> Self {
        Self {
            genome: GenomeData::default(),
            selected_mode_index: 0,
            show_mode_glow: false,
            show_genome_graph: false,
        }
    }
}