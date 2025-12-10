use imgui::{Condition, WindowFlags};

/// Camera settings state
pub struct CameraSettingsState {
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub field_of_view: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub invert_y: bool,
    pub smooth_movement: bool,
    pub auto_focus: bool,
    pub focus_distance: f32,
    pub camera_mode: usize,
}

impl Default for CameraSettingsState {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
            mouse_sensitivity: 2.0,
            field_of_view: 75.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            invert_y: false,
            smooth_movement: true,
            auto_focus: false,
            focus_distance: 10.0,
            camera_mode: 0, // 0 = Free, 1 = Orbit, 2 = Follow
        }
    }
}

/// Render the camera settings window
pub fn render_camera_settings_window(
    ui: &imgui::Ui,
    camera_state: &mut CameraSettingsState,
    global_ui_state: &super::GlobalUiState,
) {
    // Only show if visibility is enabled
    if !global_ui_state.show_camera_settings {
        return;
    }

    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };

    ui.window("Camera Settings")
        .position([2223.0, 215.0], Condition::FirstUseEver)
        .size([815.0, 613.0], Condition::FirstUseEver)
        .size_constraints([400.0, 300.0], [f32::MAX, f32::MAX])
        .flags(flags)
        .build(|| {
            render_camera_settings_content(ui, camera_state);
        });
}

/// Render just the content of the Camera Settings window (without the window wrapper)
pub fn render_camera_settings_content(
    ui: &imgui::Ui,
    camera_state: &mut CameraSettingsState,
) {
    ui.text("Camera Control Settings");
    ui.separator();
    
    // Camera mode selection
    ui.text("Camera Mode:");
    let camera_modes = ["Free Camera", "Orbit Camera", "Follow Camera"];
    for (i, mode_name) in camera_modes.iter().enumerate() {
        let is_selected = camera_state.camera_mode == i;
        if ui.radio_button_bool(mode_name, is_selected) {
            camera_state.camera_mode = i;
        }
    }
    
    ui.separator();
    
    // Movement settings
    if ui.collapsing_header("Movement", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text("Movement Speed:");
        ui.slider("##MovementSpeed", 0.1, 20.0, &mut camera_state.movement_speed);
        if ui.is_item_hovered() {
            ui.tooltip_text("How fast the camera moves when using WASD keys");
        }
        
        ui.text("Mouse Sensitivity:");
        ui.slider("##MouseSensitivity", 0.1, 10.0, &mut camera_state.mouse_sensitivity);
        if ui.is_item_hovered() {
            ui.tooltip_text("How sensitive mouse movement is for camera rotation");
        }
        
        ui.checkbox("Invert Y Axis", &mut camera_state.invert_y);
        if ui.is_item_hovered() {
            ui.tooltip_text("Invert vertical mouse movement (flight sim style)");
        }
        
        ui.checkbox("Smooth Movement", &mut camera_state.smooth_movement);
        if ui.is_item_hovered() {
            ui.tooltip_text("Enable smooth camera movement and rotation");
        }
        
        ui.unindent();
    }
    
    // View settings
    if ui.collapsing_header("View Settings", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text("Field of View:");
        ui.slider("##FieldOfView", 30.0, 120.0, &mut camera_state.field_of_view);
        if ui.is_item_hovered() {
            ui.tooltip_text("Camera field of view in degrees (higher = wider view)");
        }
        
        ui.text("Near Plane:");
        ui.slider("##NearPlane", 0.01, 10.0, &mut camera_state.near_plane);
        if ui.is_item_hovered() {
            ui.tooltip_text("Closest distance the camera can see");
        }
        
        ui.text("Far Plane:");
        ui.slider("##FarPlane", 100.0, 10000.0, &mut camera_state.far_plane);
        if ui.is_item_hovered() {
            ui.tooltip_text("Farthest distance the camera can see");
        }
        
        ui.unindent();
    }
    
    // Focus settings (for orbit and follow modes)
    if camera_state.camera_mode > 0 {
        if ui.collapsing_header("Focus Settings", imgui::TreeNodeFlags::DEFAULT_OPEN) {
            ui.indent();
            
            ui.checkbox("Auto Focus", &mut camera_state.auto_focus);
            if ui.is_item_hovered() {
                ui.tooltip_text("Automatically focus on selected objects");
            }
            
            if !camera_state.auto_focus {
                ui.text("Focus Distance:");
                ui.slider("##FocusDistance", 1.0, 100.0, &mut camera_state.focus_distance);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Distance from the focus point");
                }
            }
            
            ui.unindent();
        }
    }
    
    // Controls help
    if ui.collapsing_header("Controls", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        match camera_state.camera_mode {
            0 => { // Free Camera
                ui.text("Free Camera Controls:");
                ui.bullet_text("WASD - Move forward/back/left/right");
                ui.bullet_text("QE - Move up/down");
                ui.bullet_text("Mouse - Look around");
                ui.bullet_text("Shift - Move faster");
                ui.bullet_text("Ctrl - Move slower");
            }
            1 => { // Orbit Camera
                ui.text("Orbit Camera Controls:");
                ui.bullet_text("Mouse - Orbit around focus point");
                ui.bullet_text("Scroll - Zoom in/out");
                ui.bullet_text("Middle Mouse - Pan");
                ui.bullet_text("F - Focus on selected object");
            }
            2 => { // Follow Camera
                ui.text("Follow Camera Controls:");
                ui.bullet_text("Mouse - Adjust view angle");
                ui.bullet_text("Scroll - Adjust follow distance");
                ui.bullet_text("Tab - Cycle through objects to follow");
            }
            _ => {}
        }
        
        ui.unindent();
    }
    
    // Presets
    if ui.collapsing_header("Presets", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        if ui.button("Gaming Preset") {
            camera_state.movement_speed = 8.0;
            camera_state.mouse_sensitivity = 3.0;
            camera_state.field_of_view = 90.0;
            camera_state.invert_y = false;
            camera_state.smooth_movement = false;
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Fast, responsive settings for gaming");
        }
        
        ui.same_line();
        
        if ui.button("Cinematic Preset") {
            camera_state.movement_speed = 2.0;
            camera_state.mouse_sensitivity = 1.0;
            camera_state.field_of_view = 60.0;
            camera_state.invert_y = false;
            camera_state.smooth_movement = true;
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Smooth, cinematic camera movement");
        }
        
        if ui.button("Flight Sim Preset") {
            camera_state.movement_speed = 5.0;
            camera_state.mouse_sensitivity = 2.0;
            camera_state.field_of_view = 75.0;
            camera_state.invert_y = true;
            camera_state.smooth_movement = true;
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Flight simulator style controls");
        }
        
        ui.same_line();
        
        if ui.button("Reset to Default") {
            *camera_state = CameraSettingsState::default();
        }
        
        ui.unindent();
    }
    
    ui.separator();
    
    // Current camera info
    ui.text("Current Camera Info:");
    ui.text(format!("Mode: {}", camera_modes[camera_state.camera_mode]));
    ui.text(format!("FOV: {:.1}°", camera_state.field_of_view));
    ui.text(format!("Speed: {:.1}", camera_state.movement_speed));
    ui.text(format!("Sensitivity: {:.1}", camera_state.mouse_sensitivity));
}
