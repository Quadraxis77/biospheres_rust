#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImguiTheme {
    ModernDark,
    Industrial,
    WarmOrange,
    CellLab,
}

impl ImguiTheme {
    pub fn name(&self) -> &'static str {
        match self {
            ImguiTheme::ModernDark => "Modern Dark",
            ImguiTheme::Industrial => "Industrial",
            ImguiTheme::WarmOrange => "Warm Orange",
            ImguiTheme::CellLab => "Cell Lab",
        }
    }

    pub fn all() -> &'static [ImguiTheme] {
        &[ImguiTheme::ModernDark, ImguiTheme::Industrial, ImguiTheme::WarmOrange, ImguiTheme::CellLab]
    }
}

pub struct ImguiThemeState {
    pub current_theme: ImguiTheme,
    pub theme_changed: bool,
}

impl Default for ImguiThemeState {
    fn default() -> Self {
        Self {
            current_theme: ImguiTheme::ModernDark,
            theme_changed: true, // Apply theme on first frame
        }
    }
}

/// Apply ImGui styling and scaling
pub fn apply_imgui_style(
    ui: &imgui::Ui,
    theme_state: &mut ImguiThemeState,
    ui_scale: f32,
) {
    // Apply theme only when it changes
    if theme_state.theme_changed {
        unsafe {
            apply_theme_unsafe(theme_state.current_theme);
        }
        theme_state.theme_changed = false;
    }

    // Apply UI scale
    unsafe {
        let io_ptr = ui.io() as *const imgui::Io as *mut imgui::Io;
        (*io_ptr).font_global_scale = ui_scale;
    }
}

/// Apply theme using unsafe imgui API to actually change colors
unsafe fn apply_theme_unsafe(theme: ImguiTheme) {
    let style = imgui::sys::igGetStyle();
    
    // Apply rounded styling for most themes
    if theme != ImguiTheme::CellLab {
        apply_rounded_style(style);
    }
    
    match theme {
        ImguiTheme::ModernDark => apply_modern_dark_theme(style),
        ImguiTheme::Industrial => apply_industrial_theme(style),
        ImguiTheme::WarmOrange => apply_warm_orange_theme(style),
        ImguiTheme::CellLab => apply_cell_lab_theme(style),
    }
}

/// Apply rounded styling (common to most themes)
unsafe fn apply_rounded_style(style: *mut imgui::sys::ImGuiStyle) {
    (*style).WindowRounding = 12.0;
    (*style).WindowBorderSize = 1.0;
    (*style).WindowPadding = imgui::sys::ImVec2 { x: 12.0, y: 12.0 };
    (*style).WindowTitleAlign = imgui::sys::ImVec2 { x: 0.5, y: 0.5 };
    
    (*style).FrameRounding = 6.0;
    (*style).FrameBorderSize = 0.0;
    (*style).FramePadding = imgui::sys::ImVec2 { x: 8.0, y: 6.0 };
    
    (*style).ChildRounding = 8.0;
    (*style).ChildBorderSize = 1.0;
    
    (*style).PopupRounding = 8.0;
    (*style).PopupBorderSize = 1.0;
    
    (*style).ScrollbarRounding = 9.0;
    (*style).ScrollbarSize = 14.0;
    
    (*style).GrabRounding = 6.0;
    (*style).GrabMinSize = 12.0;
    
    (*style).TabRounding = 6.0;
    (*style).TabBorderSize = 0.0;
    
    (*style).ItemSpacing = imgui::sys::ImVec2 { x: 10.0, y: 6.0 };
    (*style).ItemInnerSpacing = imgui::sys::ImVec2 { x: 6.0, y: 6.0 };
}

/// Modern Dark theme - Clean, professional, with blue accents
unsafe fn apply_modern_dark_theme(style: *mut imgui::sys::ImGuiStyle) {
    let colors = &mut (*style).Colors;

    // Backgrounds
    colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.10, z: 0.12, w: 0.95 };
    colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.14, w: 0.90 };
    colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.10, z: 0.12, w: 0.98 };
    colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.08, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.08, z: 0.10, w: 0.75 };
    colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.14, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.20, z: 0.24, w: 0.50 };
    colors[imgui::sys::ImGuiCol_BorderShadow as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.00 };
    colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.16, z: 0.18, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.20, z: 0.24, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.24, y: 0.24, z: 0.28, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Tab as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.14, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 0.80 };
    colors[imgui::sys::ImGuiCol_TabActive as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.10, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] = imgui::sys::ImVec4 { x: 0.14, y: 0.14, z: 0.16, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 0.80 };
    colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.36, z: 0.70, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Header as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 0.60 };
    colors[imgui::sys::ImGuiCol_HeaderHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 0.80 };
    colors[imgui::sys::ImGuiCol_HeaderActive as usize] = imgui::sys::ImVec4 { x: 0.24, y: 0.50, z: 0.85, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Separator as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.20, z: 0.24, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 0.78 };
    colors[imgui::sys::ImGuiCol_SeparatorActive as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 0.40 };
    colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 0.67 };
    colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 0.95 };
    colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.08, z: 0.10, w: 0.60 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.20, z: 0.24, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.28, z: 0.32, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] = imgui::sys::ImVec4 { x: 0.36, y: 0.36, z: 0.40, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrab as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 1.00 };
    colors[imgui::sys::ImGuiCol_CheckMark as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.56, z: 0.90, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Text as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.95, z: 0.96, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextDisabled as usize] = imgui::sys::ImVec4 { x: 0.50, y: 0.50, z: 0.52, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.45, z: 0.80, w: 0.35 };
}

/// Industrial theme - Orange, yellow, and black construction/caution aesthetic
unsafe fn apply_industrial_theme(style: *mut imgui::sys::ImGuiStyle) {
    let colors = &mut (*style).Colors;

    // Backgrounds - deep black
    colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.05, y: 0.05, z: 0.05, w: 0.97 };
    colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.07, y: 0.07, z: 0.07, w: 0.93 };
    colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 0.05, y: 0.05, z: 0.05, w: 0.98 };
    colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.15, z: 0.00, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.75, z: 0.05, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.15, z: 0.00, w: 0.75 };
    colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.08, z: 0.08, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.50, z: 0.05, w: 0.75 };
    colors[imgui::sys::ImGuiCol_BorderShadow as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.50 };
    colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.18, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.28, y: 0.24, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Tab as usize] = imgui::sys::ImVec4 { x: 0.15, y: 0.13, z: 0.05, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabHovered as usize] = imgui::sys::ImVec4 { x: 0.90, y: 0.70, z: 0.10, w: 0.90 };
    colors[imgui::sys::ImGuiCol_TabActive as usize] = imgui::sys::ImVec4 { x: 0.85, y: 0.65, z: 0.08, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.10, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.30, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.45, z: 0.05, w: 0.90 };
    colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.55, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.85, y: 0.35, z: 0.00, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Header as usize] = imgui::sys::ImVec4 { x: 0.80, y: 0.55, z: 0.05, w: 0.70 };
    colors[imgui::sys::ImGuiCol_HeaderHovered as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.65, z: 0.10, w: 0.90 };
    colors[imgui::sys::ImGuiCol_HeaderActive as usize] = imgui::sys::ImVec4 { x: 0.90, y: 0.60, z: 0.08, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Separator as usize] = imgui::sys::ImVec4 { x: 0.90, y: 0.75, z: 0.10, w: 0.85 };
    colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.85, z: 0.20, w: 0.95 };
    colors[imgui::sys::ImGuiCol_SeparatorActive as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.90, z: 0.30, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.40, z: 0.00, w: 0.65 };
    colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.50, z: 0.10, w: 0.85 };
    colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.60, z: 0.20, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = imgui::sys::ImVec4 { x: 0.06, y: 0.06, z: 0.06, w: 0.75 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] = imgui::sys::ImVec4 { x: 0.40, y: 0.35, z: 0.20, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] = imgui::sys::ImVec4 { x: 0.60, y: 0.45, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] = imgui::sys::ImVec4 { x: 0.80, y: 0.55, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrab as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.60, z: 0.10, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.75, z: 0.25, w: 1.00 };
    colors[imgui::sys::ImGuiCol_CheckMark as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.90, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_Text as usize] = imgui::sys::ImVec4 { x: 0.98, y: 0.98, z: 0.98, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextDisabled as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.45, z: 0.45, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] = imgui::sys::ImVec4 { x: 0.80, y: 0.50, z: 0.10, w: 0.50 };
}

/// Warm Orange theme - Cozy, warm browns and oranges
unsafe fn apply_warm_orange_theme(style: *mut imgui::sys::ImGuiStyle) {
    let colors = &mut (*style).Colors;

    // Text colors - warm tones
    colors[imgui::sys::ImGuiCol_Text as usize] = imgui::sys::ImVec4 { x: 0.85, y: 0.75, z: 0.68, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextDisabled as usize] = imgui::sys::ImVec4 { x: 0.40, y: 0.35, z: 0.30, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 0.45 };

    // Backgrounds - dark warm browns
    colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.05, y: 0.04, z: 0.04, w: 0.97 };
    colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.07, z: 0.06, w: 0.93 };
    colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 0.05, y: 0.04, z: 0.04, w: 0.98 };
    colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.10, z: 0.09, w: 1.00 };
    
    // Title bars - subtle warm gradient
    colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.10, z: 0.09, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.16, z: 0.14, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.10, z: 0.09, w: 0.75 };

    // Borders - transparent/subtle
    colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.16, z: 0.14, w: 0.30 };
    colors[imgui::sys::ImGuiCol_BorderShadow as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.00 };

    // Frames - dark with warm tint
    colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.10, z: 0.09, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.16, z: 0.14, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.25, y: 0.20, z: 0.17, w: 1.00 };

    // Tabs - warm orange accents
    colors[imgui::sys::ImGuiCol_Tab as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.10, z: 0.09, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabHovered as usize] = imgui::sys::ImVec4 { x: 0.66, y: 0.42, z: 0.28, w: 0.85 };
    colors[imgui::sys::ImGuiCol_TabActive as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.08, z: 0.07, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.16, z: 0.14, w: 1.00 };

    // Buttons - orange accent
    colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 0.90 };
    colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 0.48, y: 0.32, z: 0.22, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.42, y: 0.28, z: 0.20, w: 1.00 };

    // Headers - orange accent
    colors[imgui::sys::ImGuiCol_Header as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 0.70 };
    colors[imgui::sys::ImGuiCol_HeaderHovered as usize] = imgui::sys::ImVec4 { x: 0.60, y: 0.38, z: 0.26, w: 0.85 };
    colors[imgui::sys::ImGuiCol_HeaderActive as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 1.00 };

    // Separator - subtle warm
    colors[imgui::sys::ImGuiCol_Separator as usize] = imgui::sys::ImVec4 { x: 0.40, y: 0.32, z: 0.27, w: 0.80 };
    colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] = imgui::sys::ImVec4 { x: 0.60, y: 0.38, z: 0.26, w: 0.85 };
    colors[imgui::sys::ImGuiCol_SeparatorActive as usize] = imgui::sys::ImVec4 { x: 0.66, y: 0.42, z: 0.28, w: 1.00 };

    // Resize grip - orange accent
    colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = imgui::sys::ImVec4 { x: 0.54, y: 0.35, z: 0.24, w: 0.60 };
    colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] = imgui::sys::ImVec4 { x: 0.60, y: 0.38, z: 0.26, w: 0.80 };
    colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] = imgui::sys::ImVec4 { x: 0.66, y: 0.42, z: 0.28, w: 1.00 };

    // Scrollbar - dark with warm accents
    colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = imgui::sys::ImVec4 { x: 0.08, y: 0.07, z: 0.06, w: 0.70 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] = imgui::sys::ImVec4 { x: 0.25, y: 0.20, z: 0.17, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.28, z: 0.24, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.36, z: 0.30, w: 1.00 };

    // Sliders - orange accent
    colors[imgui::sys::ImGuiCol_SliderGrab as usize] = imgui::sys::ImVec4 { x: 0.60, y: 0.38, z: 0.26, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] = imgui::sys::ImVec4 { x: 0.66, y: 0.42, z: 0.28, w: 1.00 };

    // Checkmark - bright orange
    colors[imgui::sys::ImGuiCol_CheckMark as usize] = imgui::sys::ImVec4 { x: 0.66, y: 0.42, z: 0.28, w: 1.00 };
}

/// Cell Lab theme - Matching the actual Cell Lab game interface
unsafe fn apply_cell_lab_theme(style: *mut imgui::sys::ImGuiStyle) {
    // Cell Lab specific rounding - very round sliders and handles!
    (*style).WindowRounding = 8.0;
    (*style).WindowBorderSize = 1.0;
    (*style).WindowPadding = imgui::sys::ImVec2 { x: 12.0, y: 12.0 };
    (*style).WindowTitleAlign = imgui::sys::ImVec2 { x: 0.5, y: 0.5 };
    
    (*style).FrameRounding = 12.0; // Very round for slider tracks!
    (*style).FrameBorderSize = 0.0;
    (*style).FramePadding = imgui::sys::ImVec2 { x: 8.0, y: 6.0 };
    
    (*style).ChildRounding = 6.0;
    (*style).ChildBorderSize = 1.0;
    
    (*style).PopupRounding = 6.0;
    (*style).PopupBorderSize = 1.0;
    
    (*style).ScrollbarRounding = 12.0; // Round scrollbars
    (*style).ScrollbarSize = 14.0;
    
    (*style).GrabRounding = 12.0; // VERY ROUND handles like Cell Lab!
    (*style).GrabMinSize = 20.0; // Bigger handles
    
    (*style).TabRounding = 6.0;
    (*style).TabBorderSize = 0.0;
    
    (*style).ItemSpacing = imgui::sys::ImVec2 { x: 10.0, y: 6.0 };
    (*style).ItemInnerSpacing = imgui::sys::ImVec2 { x: 6.0, y: 6.0 };
    
    let colors = &mut (*style).Colors;

    // Backgrounds - dark grey like the game interface
    colors[imgui::sys::ImGuiCol_WindowBg as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.16, z: 0.16, w: 0.97 };
    colors[imgui::sys::ImGuiCol_ChildBg as usize] = imgui::sys::ImVec4 { x: 0.18, y: 0.18, z: 0.18, w: 0.93 };
    colors[imgui::sys::ImGuiCol_PopupBg as usize] = imgui::sys::ImVec4 { x: 0.16, y: 0.16, z: 0.16, w: 0.98 };
    colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = imgui::sys::ImVec4 { x: 0.14, y: 0.14, z: 0.14, w: 1.00 };
    
    // Title bars - dark with lime green active
    colors[imgui::sys::ImGuiCol_TitleBg as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgActive as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.25, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.12, w: 0.75 };

    // Borders - bright lime green like the game
    colors[imgui::sys::ImGuiCol_Border as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 0.80 };
    colors[imgui::sys::ImGuiCol_BorderShadow as usize] = imgui::sys::ImVec4 { x: 0.00, y: 0.00, z: 0.00, w: 0.00 };

    // Frames - LIME GREEN for slider tracks (like the game!)
    colors[imgui::sys::ImGuiCol_FrameBg as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.60, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_FrameBgActive as usize] = imgui::sys::ImVec4 { x: 0.55, y: 0.85, z: 0.20, w: 1.00 };

    // Tabs - matching the game's tab style
    colors[imgui::sys::ImGuiCol_Tab as usize] = imgui::sys::ImVec4 { x: 0.14, y: 0.14, z: 0.14, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabHovered as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 0.90 };
    colors[imgui::sys::ImGuiCol_TabActive as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.60, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = imgui::sys::ImVec4 { x: 0.12, y: 0.12, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] = imgui::sys::ImVec4 { x: 0.18, y: 0.18, z: 0.18, w: 1.00 };

    // Buttons - subtle like the game's Save/Load buttons
    colors[imgui::sys::ImGuiCol_Button as usize] = imgui::sys::ImVec4 { x: 0.22, y: 0.22, z: 0.22, w: 0.90 };
    colors[imgui::sys::ImGuiCol_ButtonHovered as usize] = imgui::sys::ImVec4 { x: 0.30, y: 0.35, z: 0.25, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ButtonActive as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.45, z: 0.25, w: 1.00 };

    // Headers - subtle with lime green highlights
    colors[imgui::sys::ImGuiCol_Header as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.20, z: 0.20, w: 0.70 };
    colors[imgui::sys::ImGuiCol_HeaderHovered as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.60, z: 0.12, w: 0.90 };
    colors[imgui::sys::ImGuiCol_HeaderActive as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 1.00 };

    // Separator - lime green like the game borders
    colors[imgui::sys::ImGuiCol_Separator as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 0.80 };
    colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] = imgui::sys::ImVec4 { x: 0.55, y: 0.85, z: 0.20, w: 0.90 };
    colors[imgui::sys::ImGuiCol_SeparatorActive as usize] = imgui::sys::ImVec4 { x: 0.65, y: 0.95, z: 0.25, w: 1.00 };

    // Resize grip - orange/yellow like slider handles
    colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.65, z: 0.15, w: 0.70 };
    colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.75, z: 0.25, w: 0.85 };
    colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.80, z: 0.30, w: 1.00 };

    // Scrollbar - dark with lime green
    colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = imgui::sys::ImVec4 { x: 0.10, y: 0.10, z: 0.10, w: 0.75 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] = imgui::sys::ImVec4 { x: 0.35, y: 0.60, z: 0.12, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] = imgui::sys::ImVec4 { x: 0.45, y: 0.75, z: 0.15, w: 1.00 };
    colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] = imgui::sys::ImVec4 { x: 0.55, y: 0.85, z: 0.20, w: 1.00 };

    // Sliders - BRIGHT LIME GREEN bars with ORANGE/YELLOW handles (like the game!)
    colors[imgui::sys::ImGuiCol_SliderGrab as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.70, z: 0.20, w: 1.00 };
    colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.80, z: 0.30, w: 1.00 };

    // Checkmark - orange/yellow like the game's checkboxes
    colors[imgui::sys::ImGuiCol_CheckMark as usize] = imgui::sys::ImVec4 { x: 1.00, y: 0.75, z: 0.25, w: 1.00 };

    // Text - WHITE for visibility on dark backgrounds
    colors[imgui::sys::ImGuiCol_Text as usize] = imgui::sys::ImVec4 { x: 0.95, y: 0.95, z: 0.95, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextDisabled as usize] = imgui::sys::ImVec4 { x: 0.50, y: 0.50, z: 0.50, w: 1.00 };
    colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] = imgui::sys::ImVec4 { x: 0.20, y: 0.35, z: 0.10, w: 0.60 };
}

