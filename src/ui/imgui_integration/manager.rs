use super::{ImguiConfig, ImguiError, TextureHandle};
use super::texture_registry::TextureRegistry;
use std::time::Instant;

/// Manages the ImGui context and rendering lifecycle
pub struct ImguiManager {
    context: imgui::Context,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    _last_cursor: Option<imgui::MouseCursor>,
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
        
        // Note: Docking support requires imgui-rs with docking feature enabled
        // For now, we'll skip this as it may not be available in imgui 0.12.0
        
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
            _last_cursor: None,
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
        
        // Start new frame
        self.context.new_frame()
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
}
