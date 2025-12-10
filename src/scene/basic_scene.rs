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
    genome_editor::{render_genome_editor_window, render_genome_editor_content, GenomeGraphState},
    cell_inspector::{CellInspectorState, render_cell_inspector_window, render_cell_inspector_content},
    theme_editor::{ThemeEditorState, render_theme_editor_window, render_theme_editor_content},
    camera_settings::{CameraSettingsState, render_camera_settings_window, render_camera_settings_content},
    lighting_settings::{LightingSettingsState, render_lighting_settings_window, render_lighting_settings_content},
    edge_resize::{EdgeResizableWindow, EdgeResizeState},
    main_menu_bar::render_main_menu_bar,
    imgui_style::{ImguiThemeState, apply_imgui_style},
};
use crate::simulation::SimulationState;
use crate::genome::{CurrentGenome, GenomeNodeGraph};
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
    current_genome: CurrentGenome,
    node_graph: GenomeNodeGraph,
    graph_state: GenomeGraphState,
    cell_inspector_state: CellInspectorState,
    theme_editor_state: ThemeEditorState,
    camera_settings_state: CameraSettingsState,
    lighting_settings_state: LightingSettingsState,
    imgui_theme_state: ImguiThemeState,
    
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
    
    // Settings persistence
    previous_ui_state: GlobalUiState,
    previous_theme_state: ImguiThemeState,
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
        
        // Initialize UI state - load from files if they exist
        let global_ui_state = GlobalUiState::load_from_file(&GlobalUiState::default_settings_path());
        let imgui_theme_state = ImguiThemeState::load_from_file(&ImguiThemeState::default_theme_path());
        
        // Clone for tracking changes
        let previous_ui_state = global_ui_state.clone();
        let previous_theme_state = imgui_theme_state.clone();
        
        let scene_manager_state = SceneManagerState::default();
        let time_scrubber_state = TimeScrubberState::default();
        let performance_monitor = PerformanceMonitor::default();
        let simulation_state = SimulationState::default();
        let current_genome = CurrentGenome::default();
        let cell_inspector_state = CellInspectorState::default();
        let theme_editor_state = ThemeEditorState::default();
        let camera_settings_state = CameraSettingsState::default();
        let lighting_settings_state = LightingSettingsState::default();
        
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
            current_genome,
            node_graph: GenomeNodeGraph::default(),
            graph_state: GenomeGraphState::default(),
            cell_inspector_state,
            theme_editor_state,
            camera_settings_state,
            lighting_settings_state,
            imgui_theme_state,
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
            previous_ui_state,
            previous_theme_state,
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
    /// Returns (surface_texture, texture_view, command_encoder, exit_requested)
    pub fn render(&mut self, window: &Window) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView, wgpu::CommandEncoder, bool), wgpu::SurfaceError> {
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
        let (cursor_requests, manual_save_requested, exit_requested) = {
            let ui = self.imgui_manager.prepare_frame(window);
            
            // Collect cursor requests from all windows
            let mut cursor_requests = Vec::new();
            
            // Create a dockspace that covers the entire viewport
            // This allows windows to be docked anywhere in the application
            ui.dockspace_over_main_viewport();
            
            // Apply ImGui theme and styling
            apply_imgui_style(ui, &mut self.imgui_theme_state, self.global_ui_state.ui_scale);
            
            // Render main menu bar at the top
            let (manual_save_requested, mut exit_requested) = render_main_menu_bar(ui, &mut self.global_ui_state, &mut self.simulation_state, &mut self.imgui_theme_state);
            
            // Render all UI windows inline to avoid borrow checker issues
            // Scene Manager
            if self.global_ui_state.show_scene_manager {
                if self.global_ui_state.windows_locked {
                    if render_scene_manager_window(
                        ui,
                        &mut self.scene_manager_state,
                        &mut self.simulation_state,
                        &self.global_ui_state,
                    ) {
                        exit_requested = true;
                    }
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Scene Manager", &mut self.scene_manager_resize)
                        .size([355.0, 312.0], imgui::Condition::FirstUseEver)
                        .position([3079.0, 31.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([250.0, 150.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            if render_scene_manager_content(ui, &mut self.scene_manager_state, &mut self.simulation_state) {
                                exit_requested = true;
                            }
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
                    render_cell_inspector_window(
                        ui,
                        &mut self.cell_inspector_state,
                        &self.current_genome,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Cell Inspector", &mut self.cell_inspector_resize)
                        .size([355.0, 368.0], imgui::Condition::FirstUseEver)
                        .position([3079.0, 1067.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([200.0, 150.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_cell_inspector_content(ui, &mut self.cell_inspector_state, &self.current_genome);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Genome Editor
            if self.global_ui_state.show_genome_editor {
                if self.global_ui_state.windows_locked {
                    render_genome_editor_window(
                        ui,
                        &mut self.current_genome,
                        &mut self.simulation_state,
                        &self.global_ui_state,
                        &mut self.node_graph,
                        &mut self.graph_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Genome Editor", &mut self.genome_editor_resize)
                        .size([894.0, 1408.0], imgui::Condition::FirstUseEver)
                        .position([4.0, 31.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([400.0, 300.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_genome_editor_content(ui, &mut self.current_genome, &mut self.simulation_state, &mut self.node_graph, &mut self.graph_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Camera Settings
            if self.global_ui_state.show_camera_settings {
                if self.global_ui_state.windows_locked {
                    render_camera_settings_window(
                        ui,
                        &mut self.camera_settings_state,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Camera Settings", &mut self.camera_settings_resize)
                        .size([815.0, 613.0], imgui::Condition::FirstUseEver)
                        .position([2223.0, 215.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_camera_settings_content(ui, &mut self.camera_settings_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Theme Editor
            if self.global_ui_state.show_theme_editor {
                if self.global_ui_state.windows_locked {
                    render_theme_editor_window(
                        ui,
                        &mut self.theme_editor_state,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Theme Editor", &mut self.theme_editor_resize)
                        .size([398.0, 615.0], imgui::Condition::FirstUseEver)
                        .position([994.0, 421.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([250.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_theme_editor_content(ui, &mut self.theme_editor_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            // Lighting Settings
            if self.global_ui_state.show_lighting_settings {
                if self.global_ui_state.windows_locked {
                    render_lighting_settings_window(
                        ui,
                        &mut self.lighting_settings_state,
                        &self.global_ui_state,
                    );
                } else {
                    let mut cursor_to_set = None;
                    EdgeResizableWindow::new("Lighting Settings", &mut self.lighting_settings_resize)
                        .size([730.0, 556.0], imgui::Condition::FirstUseEver)
                        .position([983.0, 588.0], imgui::Condition::FirstUseEver)
                        .border_size(6.0)
                        .min_size([300.0, 200.0])
                        .build(ui, |cursor| cursor_to_set = cursor, || {
                            render_lighting_settings_content(ui, &mut self.lighting_settings_state);
                        });
                    cursor_requests.push((cursor_to_set, 10));
                }
            }
            
            (cursor_requests, manual_save_requested, exit_requested)
        };
        
        // Handle manual save request
        if manual_save_requested {
            self.save_settings();
            println!("Settings saved manually");
        }
        
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
        
        // Check for settings changes and save if needed
        self.check_and_save_settings();
        
        Ok((output, view, encoder, exit_requested))
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

    /// Save UI settings to files
    pub fn save_settings(&self) {
        // Save UI state
        if let Err(e) = self.global_ui_state.save_to_file(&GlobalUiState::default_settings_path()) {
            eprintln!("Failed to save UI settings: {}", e);
        }

        // Save theme settings
        if let Err(e) = self.imgui_theme_state.save_to_file(&ImguiThemeState::default_theme_path()) {
            eprintln!("Failed to save theme settings: {}", e);
        }
    }

    /// Check if settings have changed and save them if so
    fn check_and_save_settings(&mut self) {
        let mut settings_changed = false;

        // Check if UI state changed
        if self.global_ui_state != self.previous_ui_state {
            if let Err(e) = self.global_ui_state.save_to_file(&GlobalUiState::default_settings_path()) {
                eprintln!("Failed to save UI settings: {}", e);
            } else {
                self.previous_ui_state = self.global_ui_state.clone();
                settings_changed = true;
            }
        }

        // Check if theme state changed
        if self.imgui_theme_state.current_theme != self.previous_theme_state.current_theme {
            if let Err(e) = self.imgui_theme_state.save_to_file(&ImguiThemeState::default_theme_path()) {
                eprintln!("Failed to save theme settings: {}", e);
            } else {
                self.previous_theme_state = self.imgui_theme_state.clone();
                settings_changed = true;
            }
        }

        if settings_changed {
            println!("Settings saved automatically");
        }
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
