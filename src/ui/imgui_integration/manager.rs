use super::{ImguiConfig, ImguiError, TextureHandle};
use super::texture_registry::TextureRegistry;
use std::time::Instant;

/// Manages the ImGui context and rendering lifecycle
pub struct ImguiManager {
    context: imgui::Context,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    current_cursor: Option<imgui::MouseCursor>,
    config: ImguiConfig,
    display_scale: f32,
    texture_registry: TextureRegistry,
}

impl ImguiManager {
    /// Create a new ImguiManager with the given configuration
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_format: wgpu::TextureFormat,
        config: ImguiConfig,
    ) -> Self {
        // Create ImGui context
        let mut context = imgui::Context::create();
        
        // Configure ini filename
        context.set_ini_filename(config.ini_filename.clone());
        
        // Enable docking support
        let io = context.io_mut();
        
        // Enable docking with the docking feature enabled
        io.config_flags |= imgui::ConfigFlags::DOCKING_ENABLE;
        
        // Enable keyboard navigation
        io.config_flags |= imgui::ConfigFlags::NAV_ENABLE_KEYBOARD;
        
        // Enable window resizing from edges and corners
        io.config_windows_resize_from_edges = true;
        
        // Enable additional window interaction options
        io.config_windows_move_from_title_bar_only = false; // Allow moving from anywhere in window
        
        // Ensure mouse capture is working properly
        io.mouse_draw_cursor = false; // Let the OS handle cursor drawing
        
        // Configure window interaction for better edge detection
        io.config_drag_click_to_input_text = false;
        io.config_input_trickle_event_queue = true;
        
        // Additional configuration to improve edge resize detection
        io.config_input_text_cursor_blink = true;
        io.config_input_text_enter_keep_active = true;
        
        println!("ImGui docking and window resizing enabled successfully");
        println!("Edge resizing enabled: {}", io.config_windows_resize_from_edges);
        println!("Move from title bar only: {}", io.config_windows_move_from_title_bar_only);
        
        // Configure style for better resize visibility and edge detection
        {
            let style = context.style_mut();
            
            // Critical settings for edge resizing
            style.window_border_size = 1.0; // Normal border size
            style.window_rounding = 0.0; // No rounding to ensure clean edges
            style.window_padding = [8.0, 8.0]; // Padding inside windows
            
            // Ensure resize areas are accessible
            style.window_min_size = [100.0, 100.0]; // Larger minimum size
            
            // Make resize grips and borders highly visible
            style[imgui::StyleColor::ResizeGrip] = [0.26, 0.59, 0.98, 0.40]; // More visible resize grip
            style[imgui::StyleColor::ResizeGripHovered] = [0.26, 0.59, 0.98, 0.80]; // Bright when hovered
            style[imgui::StyleColor::ResizeGripActive] = [0.26, 0.59, 0.98, 1.00]; // Fully opaque when active
            
            // Make window borders very visible for edge detection
            style[imgui::StyleColor::Border] = [0.70, 0.70, 0.80, 1.00]; // Bright, fully opaque border
            style[imgui::StyleColor::BorderShadow] = [0.00, 0.00, 0.00, 0.00]; // No shadow
            
            // Improve window contrast
            style[imgui::StyleColor::WindowBg] = [0.06, 0.06, 0.07, 0.94]; // Dark background
            style[imgui::StyleColor::TitleBg] = [0.10, 0.10, 0.11, 1.00]; // Title background
            style[imgui::StyleColor::TitleBgActive] = [0.16, 0.29, 0.48, 1.00]; // Active title background
            
            // Additional style tweaks for better interaction
            style.frame_border_size = 1.0; // Frame borders
            style.popup_border_size = 1.0; // Popup borders
        }
        
        // Get display scale (default to 1.0 for now, will be updated later)
        let display_scale = 1.0;
        
        // Configure font atlas
        Self::configure_fonts(&mut context, &config, display_scale);
        
        // Create renderer configuration
        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: surface_format,
            ..Default::default()
        };
        
        // Initialize imgui_wgpu renderer
        let renderer = imgui_wgpu::Renderer::new(
            &mut context,
            device,
            queue,
            renderer_config,
        );
        
        log::info!("ImGui integration initialized with format: {:?}", surface_format);
        
        Self {
            context,
            renderer,
            last_frame: Instant::now(),
            current_cursor: None,
            config,
            display_scale,
            texture_registry: TextureRegistry::new(),
        }
    }
    
    /// Configure fonts with the given display scale
    fn configure_fonts(
        context: &mut imgui::Context,
        config: &ImguiConfig,
        display_scale: f32,
    ) {
        let fonts = context.fonts();
        
        // Calculate scaled font size
        let font_size = if config.apply_display_scale_to_font_size {
            config.font_size * display_scale
        } else {
            config.font_size
        };
        
        // Calculate scaled oversampling
        let (oversample_h, oversample_v) = if config.apply_display_scale_to_font_oversample {
            (
                (config.font_oversample_h as f32 * display_scale).max(1.0) as i32,
                (config.font_oversample_v as f32 * display_scale).max(1.0) as i32,
            )
        } else {
            (config.font_oversample_h, config.font_oversample_v)
        };
        
        // Add default font with configuration
        fonts.add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                size_pixels: font_size,
                oversample_h,
                oversample_v,
                ..Default::default()
            }),
        }]);
        
        // Set display framebuffer scale
        context.io_mut().display_framebuffer_scale = [display_scale, display_scale];
    }
    
    /// Prepare a new frame for rendering
    /// Returns a mutable reference to the UI context for building the UI
    pub fn prepare_frame(&mut self, window: &winit::window::Window) -> &mut imgui::Ui {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.last_frame = now;
        
        let io = self.context.io_mut();
        
        // Update delta time
        io.update_delta_time(delta);
        
        // Update display size
        let window_size = window.inner_size();
        io.display_size = [window_size.width as f32, window_size.height as f32];
        
        // Configure cursor handling
        io.mouse_draw_cursor = false; // Always let OS handle cursor drawing
        
        // Start new frame
        let ui = self.context.new_frame();
        
        // Don't reset cursor here - let the application manage cursor state
        // The cursor will be managed by the scene's cursor priority system
        
        ui
    }
    
    /// Render the ImGui UI to the given texture view
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> Result<(), ImguiError> {
        // Synchronize texture registry with renderer
        self.texture_registry.sync_with_renderer(&mut self.renderer, device, queue);
        
        // Generate draw data
        let draw_data = self.context.render();
        
        // Handle empty draw lists gracefully
        if draw_data.draw_lists_count() == 0 {
            return Ok(());
        }
        
        // Create render pass with LoadOp::Load to preserve existing content
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("imgui_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        // Render ImGui
        self.renderer
            .render(draw_data, queue, device, &mut render_pass)
            .map_err(ImguiError::from)?;
        
        Ok(())
    }
    
    /// Update the display scale and regenerate fonts
    pub fn update_display_scale(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scale: f32,
    ) {
        if (self.display_scale - scale).abs() < 0.001 {
            return; // No significant change
        }
        
        self.display_scale = scale;
        
        // Reconfigure fonts with new scale
        Self::configure_fonts(&mut self.context, &self.config, scale);
        
        // Reload font texture in renderer
        self.renderer
            .reload_font_texture(&mut self.context, device, queue);
        
        // Scale UI style proportionally
        let style = self.context.style_mut();
        style.scale_all_sizes(scale);
        
        log::info!("Display scale updated to: {}", scale);
    }
    
    /// Register a texture for use in ImGui
    pub fn register_texture(&mut self, handle: TextureHandle) -> imgui::TextureId {
        self.texture_registry.register(handle)
    }
    
    /// Unregister a texture
    pub fn unregister_texture(&mut self, id: imgui::TextureId) {
        self.texture_registry.unregister(id);
    }
    
    /// Get a mutable reference to the ImGui context
    pub fn context_mut(&mut self) -> &mut imgui::Context {
        &mut self.context
    }
    
    /// Get a reference to the ImGui context
    pub fn context(&self) -> &imgui::Context {
        &self.context
    }
    
    /// Set the mouse cursor for the current frame
    pub fn set_cursor(&mut self, cursor: Option<imgui::MouseCursor>) {
        self.current_cursor = cursor;
    }
    
    /// Handle winit events with custom edge resizing
    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        let io = self.context.io_mut();
        
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                io.add_mouse_pos_event([position.x as f32, position.y as f32]);
                
                // Custom edge resize detection
                self.handle_edge_resize_detection([position.x as f32, position.y as f32]);
                
                true
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == winit::event::ElementState::Pressed;
                match button {
                    winit::event::MouseButton::Left => {
                        io.add_mouse_button_event(imgui::MouseButton::Left, pressed);
                        
                        // Handle edge resize mouse input
                        if pressed {
                            self.start_edge_resize();
                        } else {
                            self.end_edge_resize();
                        }
                    },
                    winit::event::MouseButton::Right => io.add_mouse_button_event(imgui::MouseButton::Right, pressed),
                    winit::event::MouseButton::Middle => io.add_mouse_button_event(imgui::MouseButton::Middle, pressed),
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }
    
    /// Custom edge resize detection
    fn handle_edge_resize_detection(&mut self, _mouse_pos: [f32; 2]) {
        // This will be called during UI rendering when we have access to window positions
        // For now, we'll implement the logic in the UI rendering phase
    }
    
    /// Start edge resize operation
    fn start_edge_resize(&mut self) {
        // Will be implemented with window-specific logic
    }
    
    /// End edge resize operation  
    fn end_edge_resize(&mut self) {
        // Will be implemented with window-specific logic
    }
}
