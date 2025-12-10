use crate::ui::{GlobalUiState, imgui_style::{ImguiThemeState, ImguiTheme}};
use crate::simulation::SimulationState;
use imgui::Ui;

/// Render the main menu bar at the top of the screen
/// Returns true if manual save was requested
pub fn render_main_menu_bar(
    ui: &Ui,
    global_ui_state: &mut GlobalUiState,
    _simulation_state: &mut SimulationState,
    theme_state: &mut ImguiThemeState,
) -> bool {
    let mut manual_save_requested = false;
    if let Some(_menu_bar) = ui.begin_main_menu_bar() {
        // File menu
        if let Some(_menu) = ui.begin_menu("File") {
            if ui.menu_item_config("New Genome").shortcut("Ctrl+N").build() {
                println!("New genome requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Create a new genome from template");
            }
            
            ui.separator();
            
            if ui.menu_item_config("Load Genome...").shortcut("Ctrl+O").build() {
                println!("Load genome requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Load genome from file");
            }
            
            if ui.menu_item_config("Save Genome...").shortcut("Ctrl+S").build() {
                println!("Save genome requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Save current genome to file");
            }
            
            ui.separator();
            
            if ui.menu_item("Export Scene...") {
                println!("Export scene requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Export current scene state");
            }
            
            ui.separator();
            
            if ui.menu_item("Exit") {
                println!("Exit requested");
                // TODO: Implement proper exit handling
            }
        }
        
        // Theme menu
        if let Some(_menu) = ui.begin_menu("Theme") {
            if ui.menu_item("Theme Editor") {
                global_ui_state.show_theme_editor = !global_ui_state.show_theme_editor;
            }
            ui.separator();
            
            // Quick theme selection - Presets
            ui.text("Preset Themes:");
            for theme in ImguiTheme::all() {
                let is_current = theme_state.current_theme == *theme;
                if ui.menu_item_config(theme.name()).selected(is_current).build() {
                    if theme_state.current_theme != *theme {
                        theme_state.current_theme = *theme;
                        theme_state.theme_changed = true;
                    }
                }
            }
        }
        
        // Windows menu - for toggling window visibility
        if let Some(_menu) = ui.begin_menu("Windows") {
            ui.checkbox("Cell Inspector", &mut global_ui_state.show_cell_inspector);
            
            ui.checkbox("Genome Editor", &mut global_ui_state.show_genome_editor);
            if ui.is_item_hovered() {
                ui.tooltip_text("Edit genome modes and connections");
            }
            
            ui.checkbox("Scene Manager", &mut global_ui_state.show_scene_manager);
            ui.checkbox("Performance Monitor", &mut global_ui_state.show_performance_monitor);
            ui.checkbox("Rendering Controls", &mut global_ui_state.show_rendering_controls);
            ui.checkbox("Camera Settings", &mut global_ui_state.show_camera_settings);
            ui.checkbox("Lighting Settings", &mut global_ui_state.show_lighting_settings);
            ui.checkbox("Time Scrubber", &mut global_ui_state.show_time_scrubber);
            ui.checkbox("Theme Editor", &mut global_ui_state.show_theme_editor);
        }
        
        // Options menu
        if let Some(_menu) = ui.begin_menu("Options") {
            // Window lock toggle
            let lock_text = if global_ui_state.windows_locked {
                "🔒 Unlock Windows"
            } else {
                "🔓 Lock Windows"
            };

            if ui.menu_item(lock_text) {
                global_ui_state.windows_locked = !global_ui_state.windows_locked;
                println!("Windows locked: {}", global_ui_state.windows_locked);
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Lock windows to prevent moving/resizing");
            }

            ui.separator();

            // UI Scale radio buttons
            ui.text("UI Scale");

            let scale_options = [
                (0.75, "75%"),
                (1.0, "100%"),
                (1.25, "125%"),
                (1.5, "150%"),
                (1.75, "175%"),
                (2.0, "200%"),
                (2.5, "250%"),
                (3.0, "300%"),
                (3.5, "350%"),
                (4.0, "400%"),
            ];

            let mut current_scale = global_ui_state.ui_scale;
            for (scale_value, label) in scale_options.iter() {
                if ui.radio_button(label, &mut current_scale, *scale_value) {
                    global_ui_state.ui_scale = *scale_value;
                }
                ui.same_line();
            }
            ui.new_line();

            ui.separator();

            // Simulation options
            ui.text("Simulation");
            
            // Placeholder for simulation controls
            ui.text_disabled("Simulation controls coming soon");
            
            ui.separator();

            // Manual save settings option
            if ui.menu_item("Save Settings Now") {
                manual_save_requested = true;
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Manually save current UI and theme settings");
            }

            ui.separator();

            // Show lock status
            let status = if global_ui_state.windows_locked {
                "Windows: LOCKED"
            } else {
                "Windows: Unlocked"
            };
            ui.text(status);
        }
        
        // Simulation menu
        if let Some(_menu) = ui.begin_menu("Simulation") {
            ui.text("Mode:");
            ui.same_line();
            ui.text_colored([0.7, 0.7, 0.7, 1.0], "Preview");
            
            ui.separator();
            
            if ui.menu_item_config("Reset Simulation").shortcut("Ctrl+R").build() {
                println!("Reset simulation requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the simulation to initial state");
            }
            
            if ui.menu_item_config("Pause/Resume").shortcut("Space").build() {
                println!("Pause/Resume simulation requested");
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Toggle simulation pause state");
            }
            
            ui.separator();
            
            ui.text("Speed:");
            ui.same_line();
            ui.text_disabled("1.0x");
            
            ui.separator();
            
            ui.text("Physics:");
            ui.menu_item_config("CPU Physics").enabled(true).build();
            ui.menu_item_config("GPU Physics").enabled(false).build();
            if ui.is_item_hovered() {
                ui.tooltip_text("GPU physics not available");
            }
        }
        
        // Help menu
        if let Some(_menu) = ui.begin_menu("Help") {
            if ui.menu_item("About") {
                // Placeholder for about dialog
                println!("About dialog requested");
            }
            
            ui.separator();
            
            if ui.menu_item("Controls") {
                // Placeholder for controls help
                println!("Controls help requested");
            }
            
            if ui.menu_item("Documentation") {
                // Placeholder for documentation
                println!("Documentation requested");
            }
        }
        
        // Add version text on the right side of the menu bar
        let version_text = "BioSpheres (v0.1.8)";
        let text_width = ui.calc_text_size(version_text)[0];
        let window_width = ui.window_size()[0];
        let padding = 10.0;
        
        // Position cursor to the right side
        ui.set_cursor_pos([window_width - text_width - padding, ui.cursor_pos()[1]]);
        ui.text(version_text);
    }
    
    manual_save_requested
}