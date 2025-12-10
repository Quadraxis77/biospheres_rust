use imgui::{self, StyleColor, WindowFlags, Condition};
use crate::simulation::{SimulationState, SimulationMode};

/// State for the time scrubber UI
pub struct TimeScrubberState {
    /// Maximum time to display on the scrubber (in seconds)
    pub max_time: f32,
    /// Whether the scrubber is being actively dragged
    pub is_dragging: bool,
}

impl Default for TimeScrubberState {
    fn default() -> Self {
        Self {
            max_time: 50.0,
            is_dragging: false,
        }
    }
}

/// Render the time scrubber UI panel
pub fn render_time_scrubber(
    ui: &imgui::Ui,
    scrubber_state: &mut TimeScrubberState,
    sim_state: &mut SimulationState,
    global_ui_state: &super::GlobalUiState,
) {
    // Only show time scrubber in Preview mode
    if sim_state.mode != SimulationMode::Preview {
        return;
    }

    // Only show if visibility is enabled
    if !global_ui_state.show_time_scrubber {
        return;
    }

    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };
    
    // Create time scrubber window
    ui.window("Time Scrubber")
        .size([2169.0, 212.0], Condition::FirstUseEver)
        .position([900.0, 1227.0], Condition::FirstUseEver)
        .flags(flags)
        .build(|| {
            let mut current_time = sim_state.current_time;
            
            // Time display
            ui.text(format!("Current Time: {:.2}s", current_time));
            ui.same_line();
            ui.text(format!("/ {:.0}s", scrubber_state.max_time));
            
            ui.separator();
            
            // Main time slider
            ui.text("Scrub Time:");
            ui.set_next_item_width(-1.0); // Full width
            
            let slider_changed = ui
                .slider_config("##time_slider", 0.0, scrubber_state.max_time)
                .display_format("%.2fs")
                .build(&mut current_time);
            
            // Check if slider is being actively dragged
            let is_active = ui.is_item_active();
            
            if slider_changed {
                // Update target time for resimulation
                sim_state.target_time = Some(current_time);
                scrubber_state.is_dragging = is_active;
            } else if scrubber_state.is_dragging && !is_active {
                // Just finished dragging
                scrubber_state.is_dragging = false;
            }
            
            ui.separator();
            
            // Info about simulation state
            if sim_state.is_resimulating {
                let col_yellow = ui.style_color(StyleColor::PlotHistogram);
                ui.text_colored(col_yellow, "Simulating...");
            } else {
                let col_green = ui.style_color(StyleColor::PlotLines);
                ui.text_colored(col_green, "Ready");
            }
            
            // Display simulation info
            ui.separator();
            ui.text(format!("Timestep: {:.4}s", 0.016)); // Fixed timestep placeholder
        });
}
/// Render just the content of the Time Scrubber window (without the window wrapper)
pub fn render_time_scrubber_content(
    ui: &imgui::Ui,
    scrubber_state: &mut TimeScrubberState,
    sim_state: &mut SimulationState,
) {
    let mut current_time = sim_state.current_time;
    
    // Time display
    ui.text(format!("Current Time: {:.2}s", current_time));
    ui.same_line();
    ui.text(format!("/ {:.0}s", scrubber_state.max_time));
    
    ui.separator();
    
    // Main time slider
    ui.text("Scrub Time:");
    ui.set_next_item_width(-1.0); // Full width
    
    let slider_changed = ui
        .slider_config("##time_slider", 0.0, scrubber_state.max_time)
        .display_format("%.2fs")
        .build(&mut current_time);
    
    // Check if slider is being actively dragged
    let is_active = ui.is_item_active();
    
    if slider_changed {
        // Update target time for resimulation
        sim_state.target_time = Some(current_time);
        scrubber_state.is_dragging = is_active;
    } else if scrubber_state.is_dragging && !is_active {
        // Just finished dragging
        scrubber_state.is_dragging = false;
    }
    
    ui.separator();
    
    // Info about simulation state
    if sim_state.is_resimulating {
        let col_yellow = ui.style_color(StyleColor::PlotHistogram);
        ui.text_colored(col_yellow, "Simulating...");
    } else {
        let col_green = ui.style_color(StyleColor::PlotLines);
        ui.text_colored(col_green, "Ready");
    }
    
    // Display simulation info
    ui.separator();
    ui.text(format!("Timestep: {:.4}s", 0.016)); // Fixed timestep placeholder
}