use imgui::{Condition, WindowFlags};

/// System to render the rendering controls UI panel
pub fn render_controls_ui(
    ui: &imgui::Ui,
    global_ui_state: &mut super::GlobalUiState,
) {
    // Only show if visibility is enabled
    if !global_ui_state.show_rendering_controls {
        return;
    }
    
    // Build flags based on lock state
    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };
    
    ui.window("Rendering Controls")
        .size([355.0, 411.0], Condition::FirstUseEver)
        .position([3079.0, 654.0], Condition::FirstUseEver)
        .flags(flags)
        .build(|| {
            // Window Controls
            ui.text("Window Controls:");
            ui.separator();
            
            if ui.checkbox("Lock Window Positions/Sizes", &mut global_ui_state.windows_locked) {
                // Window lock state changed
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("When unchecked, windows can be moved and resized by dragging edges/corners");
            }
            
            ui.separator();
            ui.text("Visualization:");
            ui.separator();
            
            // Placeholder checkboxes - these would connect to actual rendering config
            let mut show_orientation_gizmos = false;
            if ui.checkbox("Show Orientation Gizmos", &mut show_orientation_gizmos) {
                // Update rendering config
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Display forward (blue), right (green), and up (red) orientation axes for each cell");
            }
            
            let mut show_split_plane_gizmos = false;
            if ui.checkbox("Show Split Plane Gizmos", &mut show_split_plane_gizmos) {
                // Update rendering config
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Display split plane rings showing the division direction for each cell");
            }
            
            let mut show_adhesions = false;
            if ui.checkbox("Show Adhesions", &mut show_adhesions) {
                // Update rendering config
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Display adhesion connections between cells");
            }
            
            ui.separator();
            let mut wireframe_mode = false;
            ui.checkbox("Wireframe Mode", &mut wireframe_mode);
            
            // World Sphere Settings
            ui.separator();
            ui.text("World Sphere:");
            
            ui.text("Opacity:");
            let mut world_opacity = 0.1;
            ui.slider("##world_opacity", 0.0, 1.0, &mut world_opacity);
            if ui.is_item_hovered() {
                ui.tooltip_text("Transparency of the world boundary sphere");
            }
            
            ui.text("Color:");
            let mut world_color = [0.5, 0.5, 0.5];
            if ui.color_edit3("##world_color", &mut world_color) {
                // Update world sphere color
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Base color of the world sphere");
            }
            
            ui.text("Edge Glow:");
            let mut world_emissive = 0.1;
            ui.slider("##world_emissive", 0.0, 0.5, &mut world_emissive);
            if ui.is_item_hovered() {
                ui.tooltip_text("Emissive lighting intensity for Fresnel edge glow");
            }
            
            // Volumetric Fog Settings
            ui.separator();
            ui.text("Volumetric Fog:");
            
            let mut fog_enabled = true;
            if ui.checkbox("Enable Fog", &mut fog_enabled) {
                // Update fog settings
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Toggle volumetric fog rendering");
            }
            
            ui.text("Density:");
            let mut fog_density = 0.5;
            ui.slider("##fog_density", 0.0, 1.0, &mut fog_density);
            if ui.is_item_hovered() {
                ui.tooltip_text("Overall fog density");
            }
            
            ui.text("Absorption:");
            let mut fog_absorption = 0.3;
            ui.slider("##fog_absorption", 0.0, 1.0, &mut fog_absorption);
            if ui.is_item_hovered() {
                ui.tooltip_text("How much light is absorbed by the fog");
            }
            
            ui.text("Scattering:");
            let mut fog_scattering = 0.7;
            ui.slider("##fog_scattering", 0.0, 1.0, &mut fog_scattering);
            if ui.is_item_hovered() {
                ui.tooltip_text("How much light is scattered by the fog");
            }
            
            ui.text("Ambient:");
            let mut fog_ambient = 0.05;
            ui.slider("##fog_ambient", 0.0, 0.1, &mut fog_ambient);
            if ui.is_item_hovered() {
                ui.tooltip_text("Ambient light intensity in fog");
            }
            
            ui.text("Fog Color:");
            let mut fog_color = [0.2, 0.3, 0.4];
            if ui.color_edit3("##fog_color", &mut fog_color) {
                // Update fog color
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Color of the volumetric fog");
            }
            
            // Bloom Settings
            ui.separator();
            ui.text("Bloom (Emissive Glow):");
            
            let mut bloom_enabled = true;
            ui.checkbox("Enable Bloom", &mut bloom_enabled);
            if ui.is_item_hovered() {
                ui.tooltip_text("Enable bloom effect for emissive materials");
            }
            
            if bloom_enabled {
                ui.text("Intensity:");
                let mut bloom_intensity = 0.3;
                ui.slider("##bloom_intensity", 0.0, 1.0, &mut bloom_intensity);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Overall bloom intensity");
                }
                
                ui.text("Low Freq Boost:");
                let mut bloom_low_freq = 0.7;
                ui.slider("##bloom_low_freq", 0.0, 1.0, &mut bloom_low_freq);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Boost for soft, wide glow (low frequency)");
                }
                
                ui.text("High Pass:");
                let mut bloom_high_pass = 0.6;
                ui.slider("##bloom_high_pass", 0.0, 1.0, &mut bloom_high_pass);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Threshold for bloom - higher values = only brightest emissives bloom");
                }
                
                // Composite mode selector
                ui.text("Composite Mode:");
                let is_additive = true;
                if ui.radio_button_bool("Additive", is_additive) {
                    // Update composite mode to additive
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Adds bloom on top - brighter but can wash out");
                }
                if ui.radio_button_bool("Energy Conserving", !is_additive) {
                    // Update composite mode to energy conserving
                }
                if ui.is_item_hovered() {
                    ui.tooltip_text("Preserves overall brightness - more natural look");
                }
            }
            
            // Theme selector
            ui.separator();
            ui.text("UI Theme:");
            
            let themes = ["Dark", "Light", "Classic"];
            let mut current_theme = 0;
            for (i, theme) in themes.iter().enumerate() {
                let is_selected = current_theme == i;
                if ui.radio_button_bool(theme, is_selected) && !is_selected {
                    current_theme = i;
                }
            }
        });
}
/// Render just the content of the Rendering Controls window (without the window wrapper)
pub fn render_controls_content(
    ui: &imgui::Ui,
    global_ui_state: &mut super::GlobalUiState,
) {
    // Window Controls
    ui.text("Window Controls:");
    ui.separator();
    
    if ui.checkbox("Lock Window Positions/Sizes", &mut global_ui_state.windows_locked) {
        // Window lock state changed
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("When unchecked, windows can be moved and resized by dragging edges/corners");
    }
    
    ui.separator();
    ui.text("Visualization:");
    ui.separator();
    
    // Placeholder checkboxes - these would connect to actual rendering config
    let mut show_orientation_gizmos = false;
    if ui.checkbox("Show Orientation Gizmos", &mut show_orientation_gizmos) {
        // Update rendering config
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Display forward (blue), right (green), and up (red) orientation axes for each cell");
    }
    
    let mut show_split_plane_gizmos = false;
    if ui.checkbox("Show Split Plane Gizmos", &mut show_split_plane_gizmos) {
        // Update rendering config
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Display split plane rings showing the division direction for each cell");
    }
    
    let mut show_adhesions = false;
    if ui.checkbox("Show Adhesions", &mut show_adhesions) {
        // Update rendering config
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Display adhesion connections between cells");
    }
    
    ui.separator();
    let mut wireframe_mode = false;
    ui.checkbox("Wireframe Mode", &mut wireframe_mode);
    
    // World Sphere Settings
    ui.separator();
    ui.text("World Sphere:");
    
    ui.text("Opacity:");
    let mut world_opacity = 0.1;
    ui.slider("##world_opacity", 0.0, 1.0, &mut world_opacity);
    if ui.is_item_hovered() {
        ui.tooltip_text("Transparency of the world boundary sphere");
    }
    
    ui.text("Color:");
    let mut world_color = [0.5, 0.5, 0.5];
    if ui.color_edit3("##world_color", &mut world_color) {
        // Update world sphere color
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Base color of the world sphere");
    }
    
    ui.text("Edge Glow:");
    let mut world_emissive = 0.1;
    ui.slider("##world_emissive", 0.0, 0.5, &mut world_emissive);
    if ui.is_item_hovered() {
        ui.tooltip_text("Emissive lighting intensity for Fresnel edge glow");
    }
    
    // Volumetric Fog Settings
    ui.separator();
    ui.text("Volumetric Fog:");
    
    let mut fog_enabled = true;
    if ui.checkbox("Enable Fog", &mut fog_enabled) {
        // Update fog settings
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Toggle volumetric fog rendering");
    }
    
    ui.text("Density:");
    let mut fog_density = 0.5;
    ui.slider("##fog_density", 0.0, 1.0, &mut fog_density);
    if ui.is_item_hovered() {
        ui.tooltip_text("Overall fog density");
    }
    
    ui.text("Absorption:");
    let mut fog_absorption = 0.3;
    ui.slider("##fog_absorption", 0.0, 1.0, &mut fog_absorption);
    if ui.is_item_hovered() {
        ui.tooltip_text("How much light is absorbed by the fog");
    }
    
    ui.text("Scattering:");
    let mut fog_scattering = 0.7;
    ui.slider("##fog_scattering", 0.0, 1.0, &mut fog_scattering);
    if ui.is_item_hovered() {
        ui.tooltip_text("How much light is scattered by the fog");
    }
    
    ui.text("Ambient:");
    let mut fog_ambient = 0.05;
    ui.slider("##fog_ambient", 0.0, 0.1, &mut fog_ambient);
    if ui.is_item_hovered() {
        ui.tooltip_text("Ambient light intensity in fog");
    }
    
    ui.text("Fog Color:");
    let mut fog_color = [0.2, 0.3, 0.4];
    if ui.color_edit3("##fog_color", &mut fog_color) {
        // Update fog color
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Color of the volumetric fog");
    }
    
    // Bloom Settings
    ui.separator();
    ui.text("Bloom (Emissive Glow):");
    
    let mut bloom_enabled = true;
    ui.checkbox("Enable Bloom", &mut bloom_enabled);
    if ui.is_item_hovered() {
        ui.tooltip_text("Enable bloom effect for emissive materials");
    }
    
    if bloom_enabled {
        ui.text("Intensity:");
        let mut bloom_intensity = 0.3;
        ui.slider("##bloom_intensity", 0.0, 1.0, &mut bloom_intensity);
        if ui.is_item_hovered() {
            ui.tooltip_text("Overall bloom intensity");
        }
        
        ui.text("Low Freq Boost:");
        let mut bloom_low_freq = 0.7;
        ui.slider("##bloom_low_freq", 0.0, 1.0, &mut bloom_low_freq);
        if ui.is_item_hovered() {
            ui.tooltip_text("Boost for soft, wide glow (low frequency)");
        }
        
        ui.text("High Pass:");
        let mut bloom_high_pass = 0.6;
        ui.slider("##bloom_high_pass", 0.0, 1.0, &mut bloom_high_pass);
        if ui.is_item_hovered() {
            ui.tooltip_text("Threshold for bloom - higher values = only brightest emissives bloom");
        }
        
        // Composite mode selector
        ui.text("Composite Mode:");
        let is_additive = true;
        if ui.radio_button_bool("Additive", is_additive) {
            // Update composite mode to additive
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Adds bloom on top - brighter but can wash out");
        }
        if ui.radio_button_bool("Energy Conserving", !is_additive) {
            // Update composite mode to energy conserving
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Preserves overall brightness - more natural look");
        }
    }
    
    // Theme selector
    ui.separator();
    ui.text("UI Theme:");
    
    let themes = ["Dark", "Light", "Classic"];
    let mut current_theme = 0;
    for (i, theme) in themes.iter().enumerate() {
        let is_selected = current_theme == i;
        if ui.radio_button_bool(theme, is_selected) && !is_selected {
            current_theme = i;
        }
    }
}