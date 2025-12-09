use std::path::PathBuf;

/// Configuration for ImGui integration
pub struct ImguiConfig {
    /// Path to the ini file for ImGui settings persistence.
    /// Set to None to disable automatic ini file saving.
    pub ini_filename: Option<PathBuf>,
    
    /// Base font size in pixels
    pub font_size: f32,
    
    /// Horizontal oversampling for font rendering
    pub font_oversample_h: i32,
    
    /// Vertical oversampling for font rendering
    pub font_oversample_v: i32,
    
    /// Whether to apply display scale to font size
    pub apply_display_scale_to_font_size: bool,
    
    /// Whether to apply display scale to font oversampling
    pub apply_display_scale_to_font_oversample: bool,
}

impl Default for ImguiConfig {
    fn default() -> Self {
        Self {
            ini_filename: Some(PathBuf::from("imgui.ini")),
            font_size: 13.0,
            font_oversample_h: 1,
            font_oversample_v: 1,
            apply_display_scale_to_font_size: true,
            apply_display_scale_to_font_oversample: true,
        }
    }
}
