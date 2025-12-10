use std::sync::Arc;
use wgpu;
use winit::window::Window;
use winit::event::WindowEvent;
use crate::ui::{
    imgui_integration::{ImguiManager, ImguiConfig},
    GlobalUiState,
    scene_manager::{SceneManagerState, render_scene_manager_window, render_scene_manager_content},
    time_scrubber::{TimeScrubberState, render_time_scrubber, render_time_scrubber_content},
    rendering_controls::{render_controls_ui, render_controls_content},
    performance_monitor::{PerformanceMonitor, render_performance_window, render_performance_content, update_performance_metrics},
    edge_resize::{EdgeResizableWindow, EdgeResizeState},
};
use crate::simulation::SimulationState;
use std::time::Instant;

/// Basic scene that renders a simple background color with ImGui UI
/// This provides the foundation for the complete UI layout
pub struct BasicScene {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    background_color: wgpu::Color,
    
    // ImGui integration
    imgui_manager: ImguiManager,
    
    // UI state
    global_ui_state: GlobalUiState,
    scene_manager_state: SceneManagerState,
    time_scrubber_state: TimeScrubberState,
    performance_monitor: PerformanceMonitor,
    simulation_state: SimulationState,
    
    // Edge resize states for all windows
    cell_inspector_resize: EdgeResizeState,
    genome_editor_resize: EdgeResizeState,
    camera_settings_resize: EdgeResizeState,
    theme_editor_resize: EdgeResizeState,
    lighting_settings_resize: EdgeResizeState,
    scene_manager_resize: EdgeResizeState,
    time_scrubber_resize: EdgeResizeState,
    performance_monitor_resize: EdgeResizeState,
    rendering_controls_resize: EdgeResizeState,
    
    // Cursor state for edge resizing
    pending_cursor: Option<imgui::MouseCursor>,
    cursor_priority: i32, // Higher values take priority
    
    // Timing
    last_frame_time: Instant,
}

impl BasicScene {
    /// Create a new BasicScene with the given window
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        
        // Create wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Create surface
        let surface = instance.create_surface(window.clone()).unwrap();
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        
        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                },
            )
            .await
            .unwrap();
        
        // Get surface capabilities and configure
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        
        surface.configure(&device, &config);
        
        // Default background color - dark gray
        let background_color = wgpu::Color {
            r: 0.1,
            g: 0.1,
            b: 0.15,
            a: 1.0,
        };
        
        // Initialize ImGui
        let imgui_config = ImguiConfig::default();
        let imgui_manager = ImguiManager::new(&device, &queue, surface_format, imgui_config);
        
        // Ensure default imgui.ini layout is created
        crate::ui::imgui_panel::ensure_default_imgui_ini();
        
        // Initialize UI state
        let global_ui_state = GlobalUiState::default();
        let scene_manager_state = SceneManagerState::default();
        let time_scrubber_state = TimeScrubberState::default();
        let performance_monitor = PerformanceMonitor::default();
        let simulation_state = SimulationState::default();
        
        Self {
            surface,
            device,
            queue,
            config,
            background_color,
            imgui_manager,
            global_ui_state,
            scene_manager_state,
            time_scrubber_state,
            performance_monitor,
            simulation_state,
            cell_inspector_resize: EdgeResizeState::default(),
            genome_editor_resize: EdgeResizeState::default(),
            camera_settings_resize: EdgeResizeState::default(),
            theme_editor_resize: EdgeResizeState::default(),
            lighting_settings_resize: EdgeResizeState::default(),
            scene_manager_resize: EdgeResizeState::default(),
            time_scrubber_resize: EdgeResizeState::default(),
            performance_monitor_resize: EdgeResizeState::default(),
            rendering_controls_resize: EdgeResizeState::default(),
            pending_cursor: None,
            cursor_priority: 0,
            last_frame_time: Instant::now(),
        }
    }
    
    /// Get reference to the device
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    
    /// Get reference to the queue
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
    
    /// Get the surface format
    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.config.format
    }
    
    /// Set the background color
    pub fn set_background_color(&mut self, color: wgpu::Color) {
        self.background_color = color;
    }
    
    /// Resize the surface
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    /// Render the scene with ImGui UI
    pub fn render(&mut self, window: &Window) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView, wgpu::CommandEncoder), wgpu::SurfaceError> {
        // Update timing
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
        let current_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        
        // Update performance metrics
        update_performance_metrics(&mut self.performance_monitor, delta_time, current_time);
        
        // Get the current frame
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        
        // Create render pass that clears to background color
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        
        // Prepare ImGui frame and render UI windows
        let cursor_requests = {
            let ui = self.imgui_manager.prepare_frame(window);
            
            // Collect cursor requests from all windows
            let mut cursor_requests = Vec::new();
            
            // Create a dockspace that covers the entire viewport
            // This allows windows to be docked anywhere in the application
            ui.dockspace_over_main_viewport();
            
            // Render all UI windows inline to avoid borrow checker issues
            // Scene Manager
            if self.global_ui_state.show_scene_manager {
                if self.global_ui_state.windows_locked {
                    render_scene_manager_window(
                        ui,
                        &mut self.scene_manager_state,
                        &mut self.simulation_state,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Scene Manager", &mut self.scene_manager_resize)
                        .size([355.0, 312.0], imgui::Condition::FirstUseEver)
                        .position([3079.0, 31.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([250.0, 150.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_scene_manager_content(ui, &mut self.scene_manager_state, &mut self.simulation_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Time Scrubber
            if self.global_ui_state.show_time_scrubber {
                if self.global_ui_state.windows_locked {
                    render_time_scrubber(
                        ui,
                        &mut self.time_scrubber_state,
                        &mut self.simulation_state,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Time Scrubber", &mut self.time_scrubber_resize)
                        .size([2169.0, 212.0], imgui::Condition::FirstUseEver)
                        .position([900.0, 1227.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 100.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_time_scrubber_content(ui, &mut self.time_scrubber_state, &mut self.simulation_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Rendering Controls
            if self.global_ui_state.show_rendering_controls {
                if self.global_ui_state.windows_locked {
                    render_controls_ui(ui, &mut self.global_ui_state);
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Rendering Controls", &mut self.rendering_controls_resize)
                        .size([400.0, 300.0], imgui::Condition::FirstUseEver)
                        .position([100.0, 100.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([250.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_controls_content(ui, &mut self.global_ui_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Advanced Performance Monitor
            if self.global_ui_state.show_performance_monitor {
                if self.global_ui_state.windows_locked {
                    render_performance_window(ui, &self.performance_monitor, &self.global_ui_state);
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Performance Monitor", &mut self.performance_monitor_resize)
                        .size([400.0, 300.0], imgui::Condition::FirstUseEver)
                        .position([200.0, 200.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_performance_content(ui, &self.performance_monitor);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Cell Inspector
            if self.global_ui_state.show_cell_inspector {
                if self.global_ui_state.windows_locked {
                    // Use regular window when locked
                    ui.window("Cell Inspector")
                        .position([3079.0, 1067.0], imgui::Condition::FirstUseEver)
                        .size([355.0, 368.0], imgui::Condition::FirstUseEver)
                        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE)
                        .build(|| {
                            ui.text("Cell Inspector");
                            ui.separator();
                            ui.text("No cell selected");
                            ui.spacing();
                            ui.text("Select a cell to view its properties:");
                            ui.bullet_text("Position and velocity");
                            ui.bullet_text("Mass and radius");
                            ui.bullet_text("Genome information");
                            ui.bullet_text("Current mode and state");
                        });
                } else {
                    // Use edge-resizable window when unlocked
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Cell Inspector", &mut self.cell_inspector_resize)
                        .size([355.0, 368.0], imgui::Condition::FirstUseEver)
                        .position([3079.0, 1067.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([200.0, 150.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            ui.text("Cell Inspector");
                            ui.separator();
                            ui.text("No cell selected");
                            ui.spacing();
                            ui.text("Select a cell to view its properties:");
                            ui.bullet_text("Position and velocity");
                            ui.bullet_text("Mass and radius");
                            ui.bullet_text("Genome information");
                            ui.bullet_text("Current mode and state");
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Genome Editor
            if self.global_ui_state.show_genome_editor {
                if self.global_ui_state.windows_locked {
                    // Use regular window when locked
                    ui.window("Genome Editor")
                        .position([4.0, 31.0], imgui::Condition::FirstUseEver)
                        .size([894.0, 1408.0], imgui::Condition::FirstUseEver)
                        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE)
                        .build(|| {
                            ui.text("Genome Editor");
                            ui.separator();
                            ui.text("Node-based genome editing interface");
                            ui.spacing();
                            ui.text("Features:");
                            ui.bullet_text("Visual node graph");
                            ui.bullet_text("Mode configuration");
                            ui.bullet_text("Behavior programming");
                            ui.bullet_text("Real-time preview");
                        });
                } else {
                    // Use edge-resizable window when unlocked
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Genome Editor", &mut self.genome_editor_resize)
                        .size([894.0, 1408.0], imgui::Condition::FirstUseEver)
                        .position([4.0, 31.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([400.0, 300.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            ui.text("Genome Editor");
                            ui.separator();
                            ui.text("Node-based genome editing interface");
                            ui.spacing();
                            ui.text("Features:");
                            ui.bullet_text("Visual node graph");
                            ui.bullet_text("Mode configuration");
                            ui.bullet_text("Behavior programming");
                            ui.bullet_text("Real-time preview");
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Camera Settings
            if self.global_ui_state.show_camera_settings {
                if self.global_ui_state.windows_locked {
                    // Use regular window when locked
                    ui.window("Camera Settings")
                        .position([2223.0, 215.0], imgui::Condition::FirstUseEver)
                        .size([815.0, 613.0], imgui::Condition::FirstUseEver)
                        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE)
                        .build(|| {
                            ui.text("Camera Settings");
                            ui.separator();
                            ui.text("Camera control options");
                            ui.spacing();
                            ui.text("Settings:");
                            ui.bullet_text("Movement speed");
                            ui.bullet_text("Mouse sensitivity");
                            ui.bullet_text("Field of view");
                            ui.bullet_text("Near/far planes");
                        });
                } else {
                    // Use edge-resizable window when unlocked
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Camera Settings", &mut self.camera_settings_resize)
                        .size([815.0, 613.0], imgui::Condition::FirstUseEver)
                        .position([2223.0, 215.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            ui.text("Camera Settings");
                            ui.separator();
                            ui.text("Camera control options");
                            ui.spacing();
                            ui.text("Settings:");
                            ui.bullet_text("Movement speed");
                            ui.bullet_text("Mouse sensitivity");
                            ui.bullet_text("Field of view");
                            ui.bullet_text("Near/far planes");
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Theme Editor
            if self.global_ui_state.show_theme_editor {
                if self.global_ui_state.windows_locked {
                    // Use regular window when locked
                    ui.window("Theme Editor")
                        .position([994.0, 421.0], imgui::Condition::FirstUseEver)
                        .size([398.0, 615.0], imgui::Condition::FirstUseEver)
                        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE)
                        .build(|| {
                            ui.text("Theme Editor");
                            ui.separator();
                            ui.text("Customize UI appearance");
                            ui.spacing();
                            ui.text("Options:");
                            ui.bullet_text("Color schemes");
                            ui.bullet_text("Font settings");
                            ui.bullet_text("Window styling");
                            ui.bullet_text("Custom themes");
                        });
                } else {
                    // Use edge-resizable window when unlocked
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Theme Editor", &mut self.theme_editor_resize)
                        .size([398.0, 615.0], imgui::Condition::FirstUseEver)
                        .position([994.0, 421.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([250.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            ui.text("Theme Editor");
                            ui.separator();
                            ui.text("Customize UI appearance");
                            ui.spacing();
                            ui.text("Options:");
                            ui.bullet_text("Color schemes");
                            ui.bullet_text("Font settings");
                            ui.bullet_text("Window styling");
                            ui.bullet_text("Custom themes");
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Lighting Settings
            if self.global_ui_state.show_lighting_settings {
                if self.global_ui_state.windows_locked {
                    // Use regular window when locked
                    ui.window("Lighting Settings")
                        .position([983.0, 588.0], imgui::Condition::FirstUseEver)
                        .size([730.0, 556.0], imgui::Condition::FirstUseEver)
                        .flags(imgui::WindowFlags::NO_MOVE | imgui::WindowFlags::NO_RESIZE)
                        .build(|| {
                            ui.text("Lighting Settings");
                            ui.separator();
                            ui.text("Scene lighting configuration");
                            ui.spacing();
                            ui.text("Controls:");
                            ui.bullet_text("Ambient lighting");
                            ui.bullet_text("Directional lights");
                            ui.bullet_text("Point lights");
                            ui.bullet_text("Shadow settings");
                        });
                } else {
                    // Use edge-resizable window when unlocked
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Lighting Settings", &mut self.lighting_settings_resize)
                        .size([730.0, 556.0], imgui::Condition::FirstUseEver)
                        .position([983.0, 588.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            ui.text("Lighting Settings");
                            ui.separator();
                            ui.text("Scene lighting configuration");
                            ui.spacing();
                            ui.text("Controls:");
                            ui.bullet_text("Ambient lighting");
                            ui.bullet_text("Directional lights");
                            ui.bullet_text("Point lights");
                            ui.bullet_text("Shadow settings");
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            cursor_requests
        };
        
        // Process cursor requests with priority
        // Reset cursor state each frame to start fresh
        let _prev_cursor = self.pending_cursor;
        self.pending_cursor = None;
        self.cursor_priority = 0;
        
        // Process all cursor requests
        for (cursor, priority) in cursor_requests {
            self.set_cursor_with_priority(cursor, priority);
        }
        
        // Debug: Uncomment to log cursor changes
        // if _prev_cursor != self.pending_cursor {
        //     println!("Cursor changed: {:?} -> {:?}", _prev_cursor, self.pending_cursor);
        // }
        
        // Render ImGui to the surface
        if let Err(e) = self.imgui_manager.render(&self.device, &self.queue, &mut encoder, &view) {
            eprintln!("ImGui render error: {:?}", e);
        }
        
        Ok((output, view, encoder))
    }

    
    /// Present the frame
    pub fn present(&self, output: wgpu::SurfaceTexture, encoder: wgpu::CommandEncoder) {
        // Submit the command buffer
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Present the frame
        output.present();
    }
    
    /// Handle input events for ImGui
    pub fn handle_input(&mut self, event: &WindowEvent) -> bool {
        self.imgui_manager.handle_event(event)
    }
    
    /// Set cursor with priority (higher priority wins)
    fn set_cursor_with_priority(&mut self, cursor: Option<imgui::MouseCursor>, priority: i32) {
        // Higher priority always wins
        if priority > self.cursor_priority {
            self.pending_cursor = cursor;
            self.cursor_priority = priority;
        }
        // Same priority: prefer non-None cursors
        else if priority == self.cursor_priority && cursor.is_some() {
            self.pending_cursor = cursor;
        }
    }
    
    /// Get the current desired cursor (for winit window)
    pub fn get_desired_cursor(&self) -> Option<winit::window::CursorIcon> {
        self.pending_cursor.map(|imgui_cursor| {
            match imgui_cursor {
                imgui::MouseCursor::ResizeEW => winit::window::CursorIcon::EwResize,
                imgui::MouseCursor::ResizeNS => winit::window::CursorIcon::NsResize,
                imgui::MouseCursor::ResizeNWSE => winit::window::CursorIcon::NwseResize,
                imgui::MouseCursor::ResizeNESW => winit::window::CursorIcon::NeswResize,
                imgui::MouseCursor::Hand => winit::window::CursorIcon::Pointer,
                imgui::MouseCursor::TextInput => winit::window::CursorIcon::Text,
                imgui::MouseCursor::ResizeAll => winit::window::CursorIcon::Move,
                imgui::MouseCursor::NotAllowed => winit::window::CursorIcon::NotAllowed,
                _ => winit::window::CursorIcon::Default,
            }
        })
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_color_setter() {
        // Test that we can create a BasicScene struct and set background color
        // Note: We can't actually create a full BasicScene in tests without a window,
        // but we can test the color type
        let color = wgpu::Color {
            r: 0.2,
            g: 0.3,
            b: 0.4,
            a: 1.0,
        };
        
        assert_eq!(color.r, 0.2);
        assert_eq!(color.g, 0.3);
        assert_eq!(color.b, 0.4);
        assert_eq!(color.a, 1.0);
    }
}
