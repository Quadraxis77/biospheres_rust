//ar ImGui panel utilities

pub struct ImguiPanelState {
    pub show_debug_info: bool,
}

impl Default for ImguiPanelState {
    fn default() -> Self {
        Self {
            show_debug_info: true,
        }
    }
}

/// Create default imgui.ini for first-time users with proper docked layout
pub fn ensure_default_imgui_ini() {
    use std::path::Path;
    
    let imgui_ini = Path::new("imgui.ini");
    
    // Only create if it doesn't exist (first-time user)
    if !imgui_ini.exists() {
        let default_layout = r#"[Window][Debug##Default]
Pos=60,60
Size=400,400
Collapsed=0

[Window][Genome Editor]
Pos=4,31
Size=894,1408
Collapsed=0

[Window][Scene Manager]
Pos=3079,31
Size=355,312
Collapsed=0
DockId=0x00000002,0

[Window][Time Scrubber]
Pos=900,1227
Size=2169,212
Collapsed=0

[Window][Rendering Controls]
Pos=3079,654
Size=355,411
Collapsed=0
DockId=0x00000005,0

[Window][Cell Inspector]
Pos=3079,1067
Size=355,368
Collapsed=0
DockId=0x00000008,0

[Window][Advanced Performance Monitor]
Pos=3079,345
Size=355,307
Collapsed=0
DockId=0x00000003,0

[Window][Theme Editor]
Pos=994,421
Size=398,615
Collapsed=0

[Window][Camera Settings]
Pos=2223,215
Size=815,613
Collapsed=0

[Window][Exit Confirmation]
Pos=1570,660
Size=300,120
Collapsed=0

[Window][Lighting Settings]
Pos=983,588
Size=730,556
Collapsed=0

[Window][Genome Graph]
Pos=610,430
Size=1000,640
Collapsed=0

[Docking][Data]
DockNode        ID=0x00000001 Pos=3079,31 Size=355,1404 Split=Y
  DockNode      ID=0x00000007 Parent=0x00000001 SizeRef=388,1034 Split=Y
    DockNode    ID=0x00000004 Parent=0x00000007 SizeRef=621,621 Split=Y
      DockNode  ID=0x00000002 Parent=0x00000004 SizeRef=401,312 Selected=0x6B58BA6D
      DockNode  ID=0x00000003 Parent=0x00000004 SizeRef=401,307 Selected=0x9B936203
    DockNode    ID=0x00000005 Parent=0x00000007 SizeRef=403,411 Selected=0x018F13E1
  DockNode      ID=0x00000008 Parent=0x00000001 SizeRef=388,368 Selected=0x0CE0C78D
"#;
        
        if let Err(e) = std::fs::write(imgui_ini, default_layout) {
            eprintln!("Failed to create default imgui.ini: {}", e);
        }
    }
}

/// Clamp all window positions to viewport bounds
/// This ensures the ENTIRE window stays within the visible viewport area
/// Only runs when a window is being actively dragged and windows are not locked
/// 
/// Note: This is a simplified version that doesn't access internal ImGui state.
/// Window clamping may need to be implemented differently depending on your ImGui setup.
pub fn enable_window_clamping(
    _ui: &imgui::Ui,
    _windows_locked: bool,
) {
    // TODO: Implement window clamping using the available imgui-rs API
    // The original Bevy implementation used internal ImGui state that's not
    // directly accessible through imgui-rs 0.12
}
