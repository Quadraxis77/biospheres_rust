use crate::simulation::{SimulationState, SimulationMode};
use imgui::{Condition, StyleColor, WindowFlags};

/// Resource to track Scene Manager window state
pub struct SceneManagerState {
    pub window_open: bool,
    pub show_exit_confirmation: bool,
}

impl Default for SceneManagerState {
    fn default() -> Self {
        Self {
            window_open: true,
            show_exit_confirmation: false,
        }
    }
}

/// Resource to store CPU scene cell capacity setting
pub struct CpuCellCapacity {
    pub capacity: usize,
}

impl Default for CpuCellCapacity {
    fn default() -> Self {
        Self {
            capacity: 4096,
        }
    }
}

/// Resource to store pending grid density change (applied on scene reset)
pub struct PendingGridDensity {
    pub density: u32,
}

impl Default for PendingGridDensity {
    fn default() -> Self {
        Self {
            density: 64, // Default 64x64x64 grid
        }
    }
}

/// Main Scene Manager window rendering function
pub fn render_scene_manager_window(
    ui: &imgui::Ui,
    scene_manager_state: &mut SceneManagerState,
    simulation_state: &mut SimulationState,
    global_ui_state: &super::GlobalUiState,
) {
    // Only render if window is open
    if !scene_manager_state.window_open {
        return;
    }

    // Only show if visibility is enabled
    if !global_ui_state.show_scene_manager {
        return;
    }

    // Build flags based on lock state
    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };

    ui.window("Scene Manager")
        .position([3079.0, 31.0], Condition::FirstUseEver)
        .size([355.0, 312.0], Condition::FirstUseEver)
        .size_constraints([250.0, 150.0], [f32::MAX, f32::MAX])
        .collapsible(true)
        .flags(flags)
        .build(|| {
            // Exit button at the top in red
            let red = [0.8, 0.2, 0.2, 1.0];
            let red_hovered = [1.0, 0.3, 0.3, 1.0];
            let red_active = [0.6, 0.1, 0.1, 1.0];
            
            let _button_color = ui.push_style_color(StyleColor::Button, red);
            let _button_hovered = ui.push_style_color(StyleColor::ButtonHovered, red_hovered);
            let _button_active = ui.push_style_color(StyleColor::ButtonActive, red_active);
            
            if ui.button("Exit Application") {
                scene_manager_state.show_exit_confirmation = true;
            }
            
            ui.separator();
            
            ui.text("Scene Selection");
            ui.separator();
            
            // Scene selection using selectable items (radio button behavior)
            let mut selected_mode = simulation_state.mode;
            
            if ui.selectable_config("Genome Editor")
                .selected(selected_mode == SimulationMode::Preview)
                .build()
            {
                selected_mode = SimulationMode::Preview;
            }
            
            if ui.selectable_config("CPU Scene")
                .selected(selected_mode == SimulationMode::Cpu)
                .build()
            {
                selected_mode = SimulationMode::Cpu;
            }
            
            // Handle scene transition if mode changed
            if selected_mode != simulation_state.mode {
                simulation_state.mode = selected_mode;
            }
            
            ui.separator();
            
            // Reset scene button (only for CPU scene)
            if simulation_state.mode != SimulationMode::Preview {
                if ui.button("Reset Scene") {
                    // Handle reset scene event
                }
                
                ui.separator();
            }
            
            // Time controls section
            ui.text("Time Controls");
            ui.separator();
            
            // Show pause/play toggle for CPU mode
            // Show message for Preview mode
            match simulation_state.mode {
                SimulationMode::Cpu => {
                    // Toggle pause/play button
                    let button_label = if simulation_state.paused { "Play" } else { "Pause" };
                    if ui.button(button_label) {
                        simulation_state.paused = !simulation_state.paused;
                    }
                    
                    ui.spacing();
                    
                    // Simulation speed control
                    ui.text("Simulation Speed");
                    
                    // Speed preset buttons
                    let speed_presets = [
                        ("0.5x", 0.5),
                        ("1x", 1.0),
                        ("2x", 2.0),
                    ];
                    
                    for (i, (label, speed)) in speed_presets.iter().enumerate() {
                        if i > 0 {
                            ui.same_line();
                        }
                        
                        let is_current = (simulation_state.speed_multiplier - speed).abs() < 0.01;
                        if is_current {
                            let _style = ui.push_style_color(StyleColor::Button, [0.0, 0.5, 0.8, 1.0]);
                            ui.button(label);
                        } else if ui.button(label) {
                            simulation_state.speed_multiplier = *speed;
                        }
                    }
                }
                SimulationMode::Preview => {
                    // Display message for Preview mode
                    ui.text("Time control handled by Time Scrubber");
                }
            }
            
            ui.separator();
        });
    
    // Exit confirmation modal
    if scene_manager_state.show_exit_confirmation {
        // Get display size to center the dialog
        let display_size = ui.io().display_size;
        let center_x = display_size[0] * 0.5;
        let center_y = display_size[1] * 0.5;
        
        ui.window("Exit Confirmation")
            .position([center_x, center_y], Condition::Always)
            .position_pivot([0.5, 0.5])
            .size([300.0, 120.0], Condition::Always)
            .collapsible(false)
            .resizable(false)
            .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
            .build(|| {
                ui.text("Are you sure you want to exit?");
                ui.spacing();
                ui.separator();
                ui.spacing();
                
                // Center the buttons
                let button_width = 120.0;
                let spacing = 10.0;
                let total_width = button_width * 2.0 + spacing;
                let window_width = 300.0;
                let offset = (window_width - total_width) * 0.5;
                
                ui.set_cursor_pos([offset, ui.cursor_pos()[1]]);
                
                // Yes button (red)
                let red = [0.8, 0.2, 0.2, 1.0];
                let red_hovered = [1.0, 0.3, 0.3, 1.0];
                let red_active = [0.6, 0.1, 0.1, 1.0];
                
                let _button_color = ui.push_style_color(StyleColor::Button, red);
                let _button_hovered = ui.push_style_color(StyleColor::ButtonHovered, red_hovered);
                let _button_active = ui.push_style_color(StyleColor::ButtonActive, red_active);
                
                if ui.button_with_size("Yes", [button_width, 0.0]) {
                    // Handle exit
                    scene_manager_state.show_exit_confirmation = false;
                }
                
                ui.same_line();
                
                // No button (default style)
                if ui.button_with_size("No", [button_width, 0.0]) {
                    scene_manager_state.show_exit_confirmation = false;
                }
            });
    }
}

/// Render just the content of the Scene Manager window (without the window wrapper)
pub fn render_scene_manager_content(
    ui: &imgui::Ui,
    scene_manager_state: &mut SceneManagerState,
    simulation_state: &mut SimulationState,
) {
    // Exit button at the top in red
    let red = [0.8, 0.2, 0.2, 1.0];
    let red_hovered = [1.0, 0.3, 0.3, 1.0];
    let red_active = [0.6, 0.1, 0.1, 1.0];
    
    let _button_color = ui.push_style_color(StyleColor::Button, red);
    let _button_hovered = ui.push_style_color(StyleColor::ButtonHovered, red_hovered);
    let _button_active = ui.push_style_color(StyleColor::ButtonActive, red_active);
    
    if ui.button("Exit Application") {
        scene_manager_state.show_exit_confirmation = true;
    }
    
    ui.separator();
    
    ui.text("Scene Selection");
    ui.separator();
    
    // Scene selection using selectable items (radio button behavior)
    let mut selected_mode = simulation_state.mode;
    
    if ui.selectable_config("Genome Editor")
        .selected(selected_mode == SimulationMode::Preview)
        .build()
    {
        selected_mode = SimulationMode::Preview;
    }
    
    if ui.selectable_config("CPU Scene")
        .selected(selected_mode == SimulationMode::Cpu)
        .build()
    {
        selected_mode = SimulationMode::Cpu;
    }
    
    // Handle scene transition if mode changed
    if selected_mode != simulation_state.mode {
        simulation_state.mode = selected_mode;
    }
    
    ui.separator();
    
    // Reset scene button (only for CPU scene)
    if simulation_state.mode != SimulationMode::Preview {
        if ui.button("Reset Scene") {
            // Handle reset scene event
        }
        
        ui.separator();
    }
    
    // Time controls section
    ui.text("Time Controls");
    ui.separator();
    
    // Show pause/play toggle for CPU mode
    // Show message for Preview mode
    match simulation_state.mode {
        SimulationMode::Cpu => {
            // Toggle pause/play button
            let button_label = if simulation_state.paused { "Play" } else { "Pause" };
            if ui.button(button_label) {
                simulation_state.paused = !simulation_state.paused;
            }
            
            ui.spacing();
            
            // Simulation speed control
            ui.text("Simulation Speed");
            
            // Speed preset buttons
            let speed_presets = [
                ("0.5x", 0.5),
                ("1x", 1.0),
                ("2x", 2.0),
            ];
            
            for (i, (label, speed)) in speed_presets.iter().enumerate() {
                if i > 0 {
                    ui.same_line();
                }
                
                let is_current = (simulation_state.speed_multiplier - speed).abs() < 0.01;
                if is_current {
                    let _style = ui.push_style_color(StyleColor::Button, [0.0, 0.5, 0.8, 1.0]);
                    ui.button(label);
                } else if ui.button(label) {
                    simulation_state.speed_multiplier = *speed;
                }
            }
        }
        SimulationMode::Preview => {
            // Display message for Preview mode
            ui.text("Time control handled by Time Scrubber");
        }
    }
    
    ui.separator();
}