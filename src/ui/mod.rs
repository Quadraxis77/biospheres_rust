pub mod camera;
pub mod camera_settings;
pub mod cell_inspector;
pub mod debug_info;
pub mod genome_editor;
pub mod imgui_integration;
pub mod imgui_panel;
pub mod imgui_style;
pub mod imgui_widgets;
pub mod imnodes_extensions;
pub mod lighting_settings;
pub mod main_menu_bar;
pub mod performance_monitor;
pub mod rendering_controls;
pub mod scene_manager;
pub mod settings;
pub mod system_tray;
pub mod theme_editor;
pub mod time_scrubber;
pub mod edge_resize;

/// Global UI state shared across all UI components
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GlobalUiState {
    pub windows_locked: bool,
    pub ui_scale: f32,
    // Window visibility toggles
    pub show_cell_inspector: bool,
    pub show_genome_editor: bool,
    pub show_scene_manager: bool,
    pub show_performance_monitor: bool,
    pub show_rendering_controls: bool,
    pub show_time_scrubber: bool,
    pub show_theme_editor: bool,
    pub show_camera_settings: bool,
    pub show_lighting_settings: bool,
}

impl Default for GlobalUiState {
    fn default() -> Self {
        Self {
            windows_locked: false,
            ui_scale: 1.0,
            show_cell_inspector: true,
            show_genome_editor: true,
            show_scene_manager: true,
            show_performance_monitor: true,
            show_rendering_controls: true,
            show_time_scrubber: true,
            show_theme_editor: true,
            show_camera_settings: true,
            show_lighting_settings: true,
        }
    }
}

impl GlobalUiState {
    /// Save settings to file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load settings from file, falling back to default if file doesn't exist or is invalid
    pub fn load_from_file(path: &std::path::Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(json) => {
                match serde_json::from_str(&json) {
                    Ok(settings) => settings,
                    Err(e) => {
                        eprintln!("Failed to parse settings file: {}. Using defaults.", e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                // File doesn't exist or can't be read, use defaults
                Self::default()
            }
        }
    }

    /// Get the default settings file path
    pub fn default_settings_path() -> std::path::PathBuf {
        std::path::PathBuf::from("ui_settings.json")
    }
}
