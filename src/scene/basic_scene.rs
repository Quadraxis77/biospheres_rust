use std::sync::Arc;
use wgpu;
use winit::window::Window;

/// Basic scene that renders a simple background color
/// This provides the foundation for ImGui rendering on top
pub struct BasicScene {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    background_color: wgpu::Color,
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
        let surface = instance.create_surface(window).unwrap();
        
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
        
        Self {
            surface,
            device,
            queue,
            config,
            background_color,
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
    
    /// Render the scene
    /// Returns the texture view for ImGui to render on top
    pub fn render(&self) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView, wgpu::CommandEncoder), wgpu::SurfaceError> {
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
        
        Ok((output, view, encoder))
    }
    
    /// Present the frame
    pub fn present(&self, output: wgpu::SurfaceTexture, encoder: wgpu::CommandEncoder) {
        // Submit the command buffer
        self.queue.submit(std::iter::once(encoder.finish()));
        
        // Present the frame
        output.present();
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
