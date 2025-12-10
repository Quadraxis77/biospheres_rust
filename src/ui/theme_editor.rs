use imgui::{Condition, WindowFlags};

/// Theme editor state
pub struct ThemeEditorState {
    pub current_theme: usize,
    pub custom_colors: Vec<[f32; 4]>,
    pub font_size: f32,
    pub window_rounding: f32,
    pub frame_rounding: f32,
    pub item_spacing: [f32; 2],
    pub window_padding: [f32; 2],
}

impl Default for ThemeEditorState {
    fn default() -> Self {
        Self {
            current_theme: 0,
            custom_colors: vec![
                [0.26, 0.59, 0.98, 1.00], // Primary
                [0.06, 0.53, 0.98, 1.00], // Secondary
                [0.20, 0.22, 0.27, 1.00], // Background
                [0.15, 0.16, 0.21, 1.00], // Surface
            ],
            font_size: 13.0,
            window_rounding: 5.0,
            frame_rounding: 3.0,
            item_spacing: [8.0, 4.0],
            window_padding: [8.0, 8.0],
        }
    }
}

/// Render the theme editor window
pub fn render_theme_editor_window(
    ui: &imgui::Ui,
    theme_state: &mut ThemeEditorState,
    global_ui_state: &super::GlobalUiState,
) {
    // Only show if visibility is enabled
    if !global_ui_state.show_theme_editor {
        return;
    }

    let flags = if global_ui_state.windows_locked {
        WindowFlags::NO_MOVE | WindowFlags::NO_RESIZE
    } else {
        WindowFlags::empty()
    };

    ui.window("Theme Editor")
        .position([994.0, 421.0], Condition::FirstUseEver)
        .size([398.0, 615.0], Condition::FirstUseEver)
        .size_constraints([300.0, 400.0], [f32::MAX, f32::MAX])
        .flags(flags)
        .build(|| {
            render_theme_editor_content(ui, theme_state);
        });
}

/// Render just the content of the Theme Editor window (without the window wrapper)
pub fn render_theme_editor_content(
    ui: &imgui::Ui,
    theme_state: &mut ThemeEditorState,
) {
    ui.text("UI Theme Customization");
    ui.separator();
    
    // Theme presets
    ui.text("Theme Presets:");
    let themes = ["Dark", "Light", "Classic", "Custom"];
    for (i, theme_name) in themes.iter().enumerate() {
        let is_selected = theme_state.current_theme == i;
        if ui.radio_button_bool(theme_name, is_selected) {
            theme_state.current_theme = i;
            apply_theme_preset(ui, i);
        }
    }
    
    ui.separator();
    
    // Color customization
    if ui.collapsing_header("Colors", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text("Primary Color:");
        ui.color_edit4("##Primary", &mut theme_state.custom_colors[0]);
        
        ui.text("Secondary Color:");
        ui.color_edit4("##Secondary", &mut theme_state.custom_colors[1]);
        
        ui.text("Background Color:");
        ui.color_edit4("##Background", &mut theme_state.custom_colors[2]);
        
        ui.text("Surface Color:");
        ui.color_edit4("##Surface", &mut theme_state.custom_colors[3]);
        
        if ui.button("Apply Custom Colors") {
            apply_custom_colors(ui, &theme_state.custom_colors);
        }
        
        ui.unindent();
    }
    
    // Typography
    if ui.collapsing_header("Typography", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text("Font Size:");
        ui.slider("##FontSize", 8.0, 24.0, &mut theme_state.font_size);
        
        ui.text("Sample text with current font");
        ui.text_colored([0.7, 0.7, 0.7, 1.0], "Secondary text example");
        
        ui.unindent();
    }
    
    // Layout & Spacing
    if ui.collapsing_header("Layout & Spacing", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text("Window Rounding:");
        if ui.slider("##WindowRounding", 0.0, 12.0, &mut theme_state.window_rounding) {
            // Apply immediately
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).WindowRounding = theme_state.window_rounding;
            }
        }
        
        ui.text("Frame Rounding:");
        if ui.slider("##FrameRounding", 0.0, 12.0, &mut theme_state.frame_rounding) {
            // Apply immediately
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).FrameRounding = theme_state.frame_rounding;
            }
        }
        
        ui.text("Item Spacing:");
        ui.text("X:"); ui.same_line();
        if ui.slider("##ItemSpacingX", 0.0, 20.0, &mut theme_state.item_spacing[0]) {
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).ItemSpacing.x = theme_state.item_spacing[0];
            }
        }
        ui.text("Y:"); ui.same_line();
        if ui.slider("##ItemSpacingY", 0.0, 20.0, &mut theme_state.item_spacing[1]) {
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).ItemSpacing.y = theme_state.item_spacing[1];
            }
        }
        
        ui.text("Window Padding:");
        ui.text("X:"); ui.same_line();
        if ui.slider("##WindowPaddingX", 0.0, 20.0, &mut theme_state.window_padding[0]) {
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).WindowPadding.x = theme_state.window_padding[0];
            }
        }
        ui.text("Y:"); ui.same_line();
        if ui.slider("##WindowPaddingY", 0.0, 20.0, &mut theme_state.window_padding[1]) {
            unsafe {
                let style = imgui::sys::igGetStyle();
                (*style).WindowPadding.y = theme_state.window_padding[1];
            }
        }
        
        ui.unindent();
    }
    
    // Preview section
    if ui.collapsing_header("Preview", imgui::TreeNodeFlags::DEFAULT_OPEN) {
        ui.indent();
        
        ui.text("Preview of current theme:");
        
        // Sample button
        if ui.button("Sample Button") {
            // Button clicked
        }
        
        ui.same_line();
        
        // Sample checkbox
        let mut sample_check = true;
        ui.checkbox("Sample Checkbox", &mut sample_check);
        
        // Sample slider
        let mut sample_value = 0.5;
        ui.text("Sample Slider:");
        ui.slider("##SampleSlider", 0.0, 1.0, &mut sample_value);
        
        // Sample input
        let mut sample_text = "Sample input text".to_string();
        ui.text("Sample Input:");
        ui.input_text("##SampleInput", &mut sample_text).build();
        
        // Sample combo
        ui.text("Sample Combo:");
        if let Some(_token) = ui.begin_combo("##SampleCombo", "Option 1") {
            ui.selectable("Option 1");
            ui.selectable("Option 2");
            ui.selectable("Option 3");
        }
        
        ui.unindent();
    }
    
    ui.separator();
    
    // Test scrolling section with extra content
    if ui.collapsing_header("Advanced Settings", imgui::TreeNodeFlags::empty()) {
        ui.indent();
        
        ui.text("Additional theme options:");
        for i in 1..=20 {
            ui.text(format!("Setting {}: Placeholder option", i));
            if i % 5 == 0 {
                ui.separator();
            }
        }
        
        ui.unindent();
    }
    
    ui.separator();
    
    // Actions
    if ui.button("Reset to Default") {
        theme_state.current_theme = 0;
        apply_theme_preset(ui, 0);
        *theme_state = ThemeEditorState::default();
    }
    
    ui.same_line();
    
    if ui.button("Save Theme") {
        // Placeholder for save functionality
        println!("Save current theme settings");
    }
    
    ui.same_line();
    
    if ui.button("Load Theme") {
        // Placeholder for load functionality
        println!("Load theme from file");
    }
}

/// Apply a theme preset
fn apply_theme_preset(ui: &imgui::Ui, theme_index: usize) {
    match theme_index {
        0 => apply_dark_theme(ui),
        1 => apply_light_theme(ui),
        2 => apply_classic_theme(ui),
        _ => {} // Custom theme - don't change anything
    }
}

/// Apply dark theme
fn apply_dark_theme(_ui: &imgui::Ui) {
    unsafe {
        let style = imgui::sys::igGetStyle();
        let colors = std::slice::from_raw_parts_mut((*style).Colors.as_mut_ptr(), imgui::sys::ImGuiCol_COUNT as usize);
        
        // Dark theme colors
        colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.06, y: 0.06, z: 0.06, w: 0.94 };
        colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.00 };
        colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.08, z: 0.08, w: 0.94 };
        colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 0.43, y: 0.43, z: 0.50, w: 0.50 };
        colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.29, z: 0.48, w: 0.54 };
        colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.40 };
        colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.67 };
        colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.04, y: 0.04, z: 0.04, w: 1.00 };
        colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.29, z: 0.48, w: 1.00 };
        colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.40 };
        colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 1.00 };
        colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.06, y: 0.53, z: 0.98, w: 1.00 };
    }
}

/// Apply light theme
fn apply_light_theme(_ui: &imgui::Ui) {
    unsafe {
        let style = imgui::sys::igGetStyle();
        let colors = std::slice::from_raw_parts_mut((*style).Colors.as_mut_ptr(), imgui::sys::ImGuiCol_COUNT as usize);
        
        // Light theme colors
        colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.94, y: 0.94, z: 0.94, w: 1.00 };
        colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.00 };
        colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 1.00, y: 1.00, z: 1.00, w: 0.98 };
        colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.30 };
        colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 1.00, y: 1.00, z: 1.00, w: 1.00 };
        colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.40 };
        colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.67 };
        colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.96, y: 0.96, z: 0.96, w: 1.00 };
        colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.82, y: 0.82, z: 0.82, w: 1.00 };
        colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 0.40 };
        colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 0.26, y: 0.59, z: 0.98, w: 1.00 };
        colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.06, y: 0.53, z: 0.98, w: 1.00 };
        colors[imgui::sys::ImGuiCol_Text as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 1.00 };
    }
}

/// Apply classic theme
fn apply_classic_theme(_ui: &imgui::Ui) {
    unsafe {
        imgui::sys::igStyleColorsClassic(imgui::sys::igGetStyle());
    }
}

/// Apply custom colors
fn apply_custom_colors(_ui: &imgui::Ui, colors: &[[f32; 4]]) {
    unsafe {
        let style = imgui::sys::igGetStyle();
        let style_colors = std::slice::from_raw_parts_mut((*style).Colors.as_mut_ptr(), imgui::sys::ImGuiCol_COUNT as usize);
        
        // Apply primary color to buttons
        if colors.len() > 0 {
            style_colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { 
                x: colors[0][0], y: colors[0][1], z: colors[0][2], w: colors[0][3] 
            };
        }
        
        // Apply secondary color to hovered states
        if colors.len() > 1 {
            style_colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { 
                x: colors[1][0], y: colors[1][1], z: colors[1][2], w: colors[1][3] 
            };
        }
        
        // Apply background color
        if colors.len() > 2 {
            style_colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { 
                x: colors[2][0], y: colors[2][1], z: colors[2][2], w: colors[2][3] 
            };
        }
        
        // Apply surface color to frames
        if colors.len() > 3 {
            style_colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { 
                x: colors[3][0], y: colors[3][1], z: colors[3][2], w: colors[3][3] 
            };
        }
    }
}
