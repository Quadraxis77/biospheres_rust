use crate::genome::{CurrentGenome, Vec3, Quat};
use imgui::{Condition, WindowFlags};

/// Mock cell data for display purposes
#[derive(Debug, Clone)]
pub struct MockCellData {
    pub cell_id: u32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Quat,
    pub angular_velocity: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub mode_index: usize,
    pub birth_time: f32,
    pub split_interval: f32,
    pub split_mass: f32,
    pub split_count: i32,
    pub adhesion_count: usize,
}

impl Default for MockCellData {
    fn default() -> Self {
        Self {
            cell_id: 12345,
            position: Vec3::new(1.23, -0.45, 2.67),
            velocity: Vec3::new(0.12, 0.03, -0.08),
            rotation: Quat { x: 0.1, y: 0.2, z: 0.3, w: 0.9 },
            angular_velocity: Vec3::new(0.05, -0.02, 0.01),
            mass: 1.85,
            radius: 0.92,
            mode_index: 0,
            birth_time: 5.2,
            split_interval: 12.5,
            split_mass: 2.0,
            split_count: 3,
            adhesion_count: 2,
        }
    }
}

/// Cell inspector state
pub struct CellInspectorState {
    pub selected_cell: Option<MockCellData>,
    pub simulation_time: f32,
}

impl Default for CellInspectorState {
    fn default() -> Self {
        Self {
            selected_cell: Some(MockCellData::default()),
            simulation_time: 17.7,
        }
    }
}

/// Render the cell inspector window
pub fn render_cell_inspector_window(
    ui: &imgui::Ui,
    inspector_state: &mut CellInspectorState,
    genome: &CurrentGenome,
    global_ui_state: &super::GlobalUiState,
) {
    // Only show if visibility is enabled
    if !global_ui_state.show_cell_inspector {
        return;
    }

    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };

    ui.window("Cell Inspector")
        .position([1704.0, 741.0], Condition::FirstUseEver)
        .size([212.0, 336.0], Condition::FirstUseEver)
        .size_constraints([250.0, 200.0], [f32::MAX, f32::MAX])
        .collapsible(true)
        .flags(flags)
        .build(|| {
            render_cell_inspector_content(ui, inspector_state, genome);
        });
}

/// Render just the content of the Cell Inspector window (without the window wrapper)
pub fn render_cell_inspector_content(
    ui: &imgui::Ui,
    inspector_state: &mut CellInspectorState,
    genome: &CurrentGenome,
) {
    // Check if we have a cell selected
    if inspector_state.selected_cell.is_none() {
        ui.text("Click on a cell to inspect it");
        ui.text("(or drag a cell)");
        
        ui.spacing();
        ui.separator();
        ui.spacing();
        
        // Button to select mock cell for demo
        if ui.button("Select Demo Cell") {
            inspector_state.selected_cell = Some(MockCellData::default());
        }
        return;
    }
    
    let data = inspector_state.selected_cell.as_ref().unwrap();
    
    // Get mode settings from genome
    let mode = genome.genome.modes.get(data.mode_index);
    let mode_name = mode.map(|m| m.name.as_str()).unwrap_or("Unknown");
    let cell_type_name = mode.map(|m| get_cell_type_name(m.cell_type)).unwrap_or("Unknown");
    
    // Calculate time alive
    let time_alive = inspector_state.simulation_time - data.birth_time;
    
    // === Cell Identity (always visible) ===
    ui.text(format!("Cell Index: {}", 0));
    ui.text(format!("Cell ID: {}", data.cell_id));
    ui.text(format!("Mode: {} ({})", mode_name, data.mode_index));
    ui.text(format!("Type: {}", cell_type_name));
    
    ui.separator();
    
    // === Key Stats (always visible) ===
    // Mass with visual bar
    const MIN_CELL_MASS: f32 = 0.5;
    let split_mass = data.split_mass;
    let max_display_mass = split_mass * 2.0;
    let mass_ratio = ((data.mass - MIN_CELL_MASS) / (max_display_mass - MIN_CELL_MASS)).clamp(0.0, 1.0);
    let bar_width = 16;
    let filled = (mass_ratio * bar_width as f32) as usize;
    let bar_str = format!("[{}{}]", "#".repeat(filled), "-".repeat(bar_width - filled));
    
    // Color based on mass relative to split threshold
    let split_ratio = (data.mass / split_mass).clamp(0.0, 2.0);
    let bar_color = if split_ratio >= 1.0 {
        [0.0, 1.0, 0.0, 1.0] // Green - ready to split
    } else if split_ratio >= 0.5 {
        [1.0, 1.0, 0.0, 1.0] // Yellow - growing
    } else {
        [1.0, 0.5, 0.0, 1.0] // Orange - low/depleted
    };
    
    ui.text("Mass:");
    ui.same_line();
    ui.text_colored(bar_color, format!("{:.2}", data.mass));
    ui.same_line();
    ui.text_colored(bar_color, bar_str);
    
    ui.text(format!("Time Alive: {:.2}s", time_alive));
    ui.text(format!("Split Count: {}", data.split_count));
    
    ui.separator();
    
    // === Nutrient Details ===
    if ui.collapsing_header("Nutrient Details", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        // Nutrient Storage (Mass)
        let storage_cap = split_mass * 2.0;
        let stored_nutrients = (data.mass - MIN_CELL_MASS).max(0.0);
        let storage_percent = (stored_nutrients / (storage_cap - MIN_CELL_MASS) * 100.0).min(100.0);
        
        ui.text("Nutrient Storage:");
        ui.same_line();
        let storage_color = if data.mass >= storage_cap {
            [0.0, 1.0, 0.0, 1.0] // Green - at cap
        } else if data.mass >= split_mass {
            [0.5, 1.0, 0.0, 1.0] // Light green - ready to split
        } else if data.mass >= MIN_CELL_MASS + (storage_cap - MIN_CELL_MASS) * 0.5 {
            [1.0, 1.0, 0.0, 1.0] // Yellow - half full
        } else if data.mass > MIN_CELL_MASS {
            [1.0, 0.5, 0.0, 1.0] // Orange - low
        } else {
            [1.0, 0.0, 0.0, 1.0] // Red - depleted
        };
        ui.text_colored(storage_color, format!("{:.2}/{:.2} ({:.0}%)", stored_nutrients, storage_cap - MIN_CELL_MASS, storage_percent));
        
        ui.spacing();
        ui.text(format!("Current Mass: {:.3}", data.mass));
        ui.text(format!("Split Mass: {:.2}", split_mass));
        ui.text(format!("Storage Cap: {:.2}", storage_cap));
        ui.text(format!("Minimum Mass: {:.2}", MIN_CELL_MASS));
        ui.text(format!("Radius: {:.3}", data.radius));
        
        if let Some(mode) = mode {
            ui.spacing();
            if mode.cell_type == 0 {
                ui.text(format!("Gain Rate: {:.2}/s", mode.nutrient_gain_rate));
            } else if mode.cell_type == 1 {
                ui.text(format!("Swim Force: {:.2}", mode.swim_force));
                ui.text(format!("Consumption: {:.3}/s", mode.swim_force * 0.2));
            }
            ui.text(format!("Max Size: {:.2}", mode.max_cell_size));
            
            // Show base priority and boosted priority if applicable
            let danger_threshold = 0.6;
            let priority_boost = 10.0;
            let is_boosted = mode.prioritize_when_low && data.mass < danger_threshold;
            let effective_priority = if is_boosted {
                mode.nutrient_priority * priority_boost
            } else {
                mode.nutrient_priority
            };
            
            ui.text(format!("Base Priority: {:.2}", mode.nutrient_priority));
            if is_boosted {
                ui.same_line();
                ui.text_colored([1.0, 0.0, 0.0, 1.0], format!("→ {:.1} (BOOSTED!)", effective_priority));
            }
            
            ui.text(format!("Protect Low: {}", if mode.prioritize_when_low { "Yes" } else { "No" }));
            if mode.prioritize_when_low {
                ui.same_line();
                if is_boosted {
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], "(ACTIVE)");
                } else {
                    ui.text_colored([0.5, 0.5, 0.5, 1.0], "(inactive)");
                }
            }
            
            ui.text(format!("Split Ratio: {:.0}%", mode.split_ratio * 100.0));
        }
        
        ui.unindent();
    }
    
    // === Position & Motion ===
    if ui.collapsing_header("Position & Motion", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text(format!("Position: ({:.2}, {:.2}, {:.2})", 
            data.position.x, data.position.y, data.position.z));
        ui.text(format!("Velocity: ({:.2}, {:.2}, {:.2})", 
            data.velocity.x, data.velocity.y, data.velocity.z));
        ui.text(format!("Speed: {:.3}", (data.velocity.x.powi(2) + data.velocity.y.powi(2) + data.velocity.z.powi(2)).sqrt()));
        
        ui.unindent();
    }
    
    // === Rotation ===
    if ui.collapsing_header("Rotation", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        let euler = quat_to_euler_degrees(data.rotation);
        ui.text(format!("Rotation (deg): ({:.1}, {:.1}, {:.1})", 
            euler.x, euler.y, euler.z));
        ui.text(format!("Angular Vel: ({:.2}, {:.2}, {:.2})", 
            data.angular_velocity.x, data.angular_velocity.y, data.angular_velocity.z));
        
        ui.unindent();
    }
    
    // === Division Info ===
    if ui.collapsing_header("Division", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text(format!("Birth Time: {:.2}s", data.birth_time));
        ui.text(format!("Time Alive: {:.2}s", time_alive));
        ui.text(format!("Split Interval: {:.2}s", data.split_interval));
        
        // Time until next split
        let time_until_split = (data.split_interval - time_alive).max(0.0);
        if time_until_split > 0.0 {
            ui.text(format!("Next Split In: {:.2}s", time_until_split));
        } else {
            ui.text_colored([0.0, 1.0, 0.0, 1.0], "Ready to split!");
        }
        
        ui.text(format!("Split Count: {}", data.split_count));
        
        if let Some(mode) = mode {
            if mode.max_splits >= 0 {
                ui.text(format!("Max Splits: {}", mode.max_splits));
                let remaining = (mode.max_splits - data.split_count).max(0);
                ui.text(format!("Remaining: {}", remaining));
            } else {
                ui.text("Max Splits: Infinite");
            }
            ui.text(format!("Min Adhesions: {}", mode.min_adhesions));
        }
        
        ui.unindent();
    }
    
    // === Adhesions ===
    if ui.collapsing_header("Adhesions", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text(format!("Adhesion Count: {}", data.adhesion_count));
        if let Some(mode) = mode {
            ui.text(format!("Max Adhesions: {}", mode.max_adhesions));
        }
        
        ui.unindent();
    }
    
    // === Flagellocyte-specific ===
    if let Some(mode) = mode {
        if mode.cell_type == 1 {
            if ui.collapsing_header("Flagellocyte", imgui::TreeNodeFlags::DEFAULT_OPEN) {
                ui.indent();
                ui.text(format!("Swim Force: {:.2}", mode.swim_force));
                ui.unindent();
            }
        }
    }
    
    ui.separator();
    
    // Clear selection button
    if ui.button("Clear Selection") {
        inspector_state.selected_cell = None;
    }
    
    // Demo controls
    ui.same_line();
    if ui.button("Random Cell") {
        // Generate random cell data for demo
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        inspector_state.simulation_time.to_bits().hash(&mut hasher);
        let seed = hasher.finish();
        
        let random_f32 = |min: f32, max: f32, offset: u64| -> f32 {
            let mut h = DefaultHasher::new();
            (seed + offset).hash(&mut h);
            let r = (h.finish() % 1000) as f32 / 1000.0;
            min + r * (max - min)
        };
        
        inspector_state.selected_cell = Some(MockCellData {
            cell_id: (seed % 99999) as u32,
            position: Vec3::new(
                random_f32(-5.0, 5.0, 1),
                random_f32(-5.0, 5.0, 2),
                random_f32(-5.0, 5.0, 3),
            ),
            velocity: Vec3::new(
                random_f32(-0.5, 0.5, 4),
                random_f32(-0.5, 0.5, 5),
                random_f32(-0.5, 0.5, 6),
            ),
            mass: random_f32(0.8, 3.5, 7),
            radius: random_f32(0.6, 1.2, 8),
            mode_index: (seed % genome.genome.modes.len() as u64) as usize,
            birth_time: random_f32(0.0, inspector_state.simulation_time - 1.0, 9),
            split_count: (seed % 8) as i32,
            adhesion_count: (seed % 6) as usize,
            ..MockCellData::default()
        });
    }
}

/// Get human-readable cell type name
fn get_cell_type_name(cell_type: i32) -> &'static str {
    match cell_type {
        0 => "Test (Nutrient)",
        1 => "Flagellocyte",
        2 => "Photocyte",
        3 => "Phagocyte",
        _ => "Unknown",
    }
}

/// Convert quaternion to euler angles in degrees
fn quat_to_euler_degrees(quat: Quat) -> Vec3 {
    // Simple conversion - this is just for display purposes
    let x = (2.0 * (quat.w * quat.x + quat.y * quat.z)).atan2(1.0 - 2.0 * (quat.x * quat.x + quat.y * quat.y)).to_degrees();
    let y = (2.0 * (quat.w * quat.y - quat.z * quat.x)).clamp(-1.0, 1.0).asin().to_degrees();
    let z = (2.0 * (quat.w * quat.z + quat.x * quat.y)).atan2(1.0 - 2.0 * (quat.y * quat.y + quat.z * quat.z)).to_degrees();
    
    Vec3::new(x, y, z)
}
