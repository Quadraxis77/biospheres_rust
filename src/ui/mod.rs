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
pub mod performance_monitor;
pub mod rendering_controls;
pub mod scene_manager;
pub mod settings;
pub mod system_tray;
pub mod theme_editor;
pub mod time_scrubber;
pub mod edge_resize;

/// Global UI state shared across all UI components
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
