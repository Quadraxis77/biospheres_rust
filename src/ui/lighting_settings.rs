use imgui::{Condition, WindowFlags};

/// Lighting settings state
pub struct LightingSettingsState {
    // Ambient lighting
    pub ambient_color: [f32; 3],
    pub ambient_intensity: f32,
    
    // Directional light (sun)
    pub sun_enabled: bool,
    pub sun_color: [f32; 3],
    pub sun_intensity: f32,
    pub sun_direction: [f32; 3],
    pub sun_cast_shadows: bool,
    
    // Point lights
    pub point_lights: Vec<PointLight>,
    pub selected_point_light: usize,
    
    // Shadow settings
    pub shadow_quality: usize,
    pub shadow_distance: f32,
    pub shadow_bias: f32,
    
    // Environment
    pub skybox_enabled: bool,
    pub skybox_tint: [f32; 3],
    pub fog_enabled: bool,
    pub fog_color: [f32; 3],
    pub fog_density: f32,
}

#[derive(Debug, Clone)]
pub struct PointLight {
    pub enabled: bool,
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub cast_shadows: bool,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            enabled: true,
            position: [0.0, 5.0, 0.0],
            color: [1.0, 1.0, 1.0],
            intensity: 1.0,
            range: 10.0,
            cast_shadows: false,
        }
    }
}

impl Default for LightingSettingsState {
    fn default() -> Self {
        Self {
            ambient_color: [0.2, 0.3, 0.4],
            ambient_intensity: 0.3,
            
            sun_enabled: true,
            sun_color: [1.0, 0.95, 0.8],
            sun_intensity: 3.0,
            sun_direction: [-0.3, -0.7, -0.6],
            sun_cast_shadows: true,
            
            point_lights: vec![
                PointLight {
                    position: [5.0, 3.0, 5.0],
                    color: [1.0, 0.8, 0.6],
                    intensity: 2.0,
                    range: 15.0,
                    ..Default::default()
                },
                PointLight {
                    position: [-5.0, 3.0, -5.0],
                    color: [0.6, 0.8, 1.0],
                    intensity: 1.5,
                    range: 12.0,
                    ..Default::default()
                },
            ],
            selected_point_light: 0,
            
            shadow_quality: 1, // 0=Low, 1=Medium, 2=High, 3=Ultra
            shadow_distance: 50.0,
            shadow_bias: 0.001,
            
            skybox_enabled: true,
            skybox_tint: [1.0, 1.0, 1.0],
            fog_enabled: false,
            fog_color: [0.7, 0.8, 0.9],
            fog_density: 0.02,
        }
    }
}

/// Render the lighting settings window
pub fn render_lighting_settings_window(
    ui: &imgui::Ui,
    lighting_state: &mut LightingSettingsState,
    global_ui_state: &super::GlobalUiState,
) {
    // Only show if visibility is enabled
    if !global_ui_state.show_lighting_settings {
        return;
    }

    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };

    ui.window("Lighting Settings")
        .position([983.0, 588.0], Condition::FirstUseEver)
        .size([730.0, 556.0], Condition::FirstUseEver)
        .size_constraints([500.0, 400.0], [f32::MAX, f32::MAX])
        .flags(flags)
        .build(|| {
            render_lighting_settings_content(ui, lighting_state);
        });
}

/// Render just the content of the Lighting Settings window (without the window wrapper)
pub fn render_lighting_settings_content(
    ui: &imgui::Ui,
    lighting_state: &mut LightingSettingsState,
) {
    ui.text("Scene Lighting Configuration");
    ui.separator();
    
    // Ambient lighting
    if ui.collapsing_header("Ambient Lighting", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text("Ambient Color:");
        ui.color_edit3("##AmbientColor", &mut lighting_state.ambient_color);
        
        ui.text("Ambient Intensity:");
        ui.slider("##AmbientIntensity", 0.0, 2.0, &mut lighting_state.ambient_intensity);
        if ui.is_item_hovered() {
            ui.tooltip_text("Overall ambient light level in the scene");
        }
        
        ui.unindent();
    }
    
    // Directional light (sun)
    if ui.collapsing_header("Directional Light (Sun)", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.checkbox("Enable Sun", &mut lighting_state.sun_enabled);
        
        if lighting_state.sun_enabled {
            ui.text("Sun Color:");
            ui.color_edit3("##SunColor", &mut lighting_state.sun_color);
            
            ui.text("Sun Intensity:");
            ui.slider("##SunIntensity", 0.0, 10.0, &mut lighting_state.sun_intensity);
            
            ui.text("Sun Direction:");
            ui.text("X:"); ui.same_line();
            ui.slider("##SunDirX", -1.0, 1.0, &mut lighting_state.sun_direction[0]);
            ui.text("Y:"); ui.same_line();
            ui.slider("##SunDirY", -1.0, 1.0, &mut lighting_state.sun_direction[1]);
            ui.text("Z:"); ui.same_line();
            ui.slider("##SunDirZ", -1.0, 1.0, &mut lighting_state.sun_direction[2]);
            if ui.is_item_hovered() {
                ui.tooltip_text("Direction the sun is pointing (X, Y, Z)");
            }
            
            ui.checkbox("Cast Shadows", &mut lighting_state.sun_cast_shadows);
        }
        
        ui.unindent();
    }
    
    // Point lights
    if ui.collapsing_header("Point Lights", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        // Point light list
        ui.text("Point Lights:");
        for (i, light) in lighting_state.point_lights.iter().enumerate() {
            let label = format!("Light {} {}", i + 1, if light.enabled { "●" } else { "○" });
            let is_selected = lighting_state.selected_point_light == i;
            
            if ui.selectable_config(&label).selected(is_selected).build() {
                lighting_state.selected_point_light = i;
            }
        }
        
        // Add/Remove buttons
        if ui.button("Add Light") && lighting_state.point_lights.len() < 8 {
            lighting_state.point_lights.push(PointLight::default());
        }
        
        ui.same_line();
        
        if ui.button("Remove Light") && !lighting_state.point_lights.is_empty() {
            if lighting_state.selected_point_light < lighting_state.point_lights.len() {
                lighting_state.point_lights.remove(lighting_state.selected_point_light);
                if lighting_state.selected_point_light >= lighting_state.point_lights.len() && !lighting_state.point_lights.is_empty() {
                    lighting_state.selected_point_light = lighting_state.point_lights.len() - 1;
                }
            }
        }
        
        ui.separator();
        
        // Selected point light settings
        if lighting_state.selected_point_light < lighting_state.point_lights.len() {
            let light = &mut lighting_state.point_lights[lighting_state.selected_point_light];
            
            ui.text(&format!("Light {} Settings:", lighting_state.selected_point_light + 1));
            
            ui.checkbox("Enabled", &mut light.enabled);
            
            if light.enabled {
                ui.text("Position:");
                ui.text("X:"); ui.same_line();
                ui.slider("##PointPosX", -20.0, 20.0, &mut light.position[0]);
                ui.text("Y:"); ui.same_line();
                ui.slider("##PointPosY", -20.0, 20.0, &mut light.position[1]);
                ui.text("Z:"); ui.same_line();
                ui.slider("##PointPosZ", -20.0, 20.0, &mut light.position[2]);
                
                ui.text("Color:");
                ui.color_edit3("##PointColor", &mut light.color);
                
                ui.text("Intensity:");
                ui.slider("##PointIntensity", 0.0, 10.0, &mut light.intensity);
                
                ui.text("Range:");
                ui.slider("##PointRange", 1.0, 50.0, &mut light.range);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Maximum distance the light reaches");
                }
                
                ui.checkbox("Cast Shadows", &mut light.cast_shadows);
            }
        }
        
        ui.unindent();
    }
    
    // Shadow settings
    if ui.collapsing_header("Shadow Settings", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text("Shadow Quality:");
        let quality_names = ["Low", "Medium", "High", "Ultra"];
        for (i, quality_name) in quality_names.iter().enumerate() {
            let is_selected = lighting_state.shadow_quality == i;
            if ui.radio_button_bool(quality_name, is_selected) {
                lighting_state.shadow_quality = i;
            }
        }
        
        ui.text("Shadow Distance:");
        ui.slider("##ShadowDistance", 10.0, 200.0, &mut lighting_state.shadow_distance);
        if ui.is_item_hovered() {
            ui.tooltip_text("Maximum distance for shadow rendering");
        }
        
        ui.text("Shadow Bias:");
        ui.slider("##ShadowBias", 0.0001, 0.01, &mut lighting_state.shadow_bias);
        if ui.is_item_hovered() {
            ui.tooltip_text("Bias to prevent shadow acne (lower = more accurate, higher = less artifacts)");
        }
        
        ui.unindent();
    }
    
    // Environment
    if ui.collapsing_header("Environment", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        // Skybox
        ui.checkbox("Enable Skybox", &mut lighting_state.skybox_enabled);
        
        if lighting_state.skybox_enabled {
            ui.text("Skybox Tint:");
            ui.color_edit3("##SkyboxTint", &mut lighting_state.skybox_tint);
        }
        
        ui.separator();
        
        // Fog
        ui.checkbox("Enable Fog", &mut lighting_state.fog_enabled);
        
        if lighting_state.fog_enabled {
            ui.text("Fog Color:");
            ui.color_edit3("##FogColor", &mut lighting_state.fog_color);
            
            ui.text("Fog Density:");
            ui.slider("##FogDensity", 0.001, 0.1, &mut lighting_state.fog_density);
            if ui.is_item_hovered() {
                ui.tooltip_text("How thick the fog is (higher = denser fog)");
            }
        }
        
        ui.unindent();
    }
    
    // Presets
    if ui.collapsing_header("Lighting Presets", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        if ui.button("Daylight") {
            apply_daylight_preset(lighting_state);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Bright outdoor daylight scene");
        }
        
        ui.same_line();
        
        if ui.button("Sunset") {
            apply_sunset_preset(lighting_state);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Warm sunset lighting");
        }
        
        if ui.button("Night") {
            apply_night_preset(lighting_state);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Dark night scene with artificial lights");
        }
        
        ui.same_line();
        
        if ui.button("Studio") {
            apply_studio_preset(lighting_state);
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Even studio lighting for detailed viewing");
        }
        
        if ui.button("Reset to Default") {
            *lighting_state = LightingSettingsState::default();
        }
        
        ui.unindent();
    }
    
    ui.separator();
    
    // Lighting summary
    ui.text("Lighting Summary:");
    ui.text(format!("Ambient: {:.2}", lighting_state.ambient_intensity));
    ui.text(format!("Sun: {}", if lighting_state.sun_enabled { "On" } else { "Off" }));
    let active_lights = lighting_state.point_lights.iter().filter(|l| l.enabled).count();
    ui.text(format!("Point Lights: {}/{}", active_lights, lighting_state.point_lights.len()));
    let quality_names = ["Low", "Medium", "High", "Ultra"];
    ui.text(format!("Shadows: {}", quality_names[lighting_state.shadow_quality]));
}

/// Apply daylight preset
fn apply_daylight_preset(lighting_state: &mut LightingSettingsState) {
    lighting_state.ambient_color = [0.4, 0.5, 0.6];
    lighting_state.ambient_intensity = 0.4;
    
    lighting_state.sun_enabled = true;
    lighting_state.sun_color = [1.0, 0.95, 0.8];
    lighting_state.sun_intensity = 4.0;
    lighting_state.sun_direction = [-0.3, -0.7, -0.6];
    lighting_state.sun_cast_shadows = true;
    
    lighting_state.fog_enabled = false;
    lighting_state.skybox_enabled = true;
    lighting_state.skybox_tint = [1.0, 1.0, 1.0];
}

/// Apply sunset preset
fn apply_sunset_preset(lighting_state: &mut LightingSettingsState) {
    lighting_state.ambient_color = [0.6, 0.3, 0.2];
    lighting_state.ambient_intensity = 0.3;
    
    lighting_state.sun_enabled = true;
    lighting_state.sun_color = [1.0, 0.6, 0.3];
    lighting_state.sun_intensity = 2.5;
    lighting_state.sun_direction = [-0.8, -0.3, -0.5];
    lighting_state.sun_cast_shadows = true;
    
    lighting_state.fog_enabled = true;
    lighting_state.fog_color = [0.8, 0.5, 0.3];
    lighting_state.fog_density = 0.01;
    
    lighting_state.skybox_enabled = true;
    lighting_state.skybox_tint = [1.0, 0.8, 0.6];
}

/// Apply night preset
fn apply_night_preset(lighting_state: &mut LightingSettingsState) {
    lighting_state.ambient_color = [0.1, 0.1, 0.2];
    lighting_state.ambient_intensity = 0.1;
    
    lighting_state.sun_enabled = false;
    
    // Enable point lights for night scene
    for light in &mut lighting_state.point_lights {
        light.enabled = true;
        light.intensity = 3.0;
    }
    
    lighting_state.fog_enabled = false;
    lighting_state.skybox_enabled = true;
    lighting_state.skybox_tint = [0.2, 0.2, 0.4];
}

/// Apply studio preset
fn apply_studio_preset(lighting_state: &mut LightingSettingsState) {
    lighting_state.ambient_color = [0.5, 0.5, 0.5];
    lighting_state.ambient_intensity = 0.6;
    
    lighting_state.sun_enabled = true;
    lighting_state.sun_color = [1.0, 1.0, 1.0];
    lighting_state.sun_intensity = 2.0;
    lighting_state.sun_direction = [0.0, -1.0, 0.0];
    lighting_state.sun_cast_shadows = false;
    
    lighting_state.fog_enabled = false;
    lighting_state.skybox_enabled = false;
}
