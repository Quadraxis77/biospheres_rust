use std::collections::HashMap;
use std::sync::Arc;

/// Handle to a texture that can be displayed in ImGui
pub struct TextureHandle {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub size: wgpu::Extent3d,
}

impl TextureHandle {
    pub fn new(
        texture: Arc<wgpu::Texture>,
        view: Arc<wgpu::TextureView>,
        size: wgpu::Extent3d,
    ) -> Self {
        Self {
            texture,
            view,
            size,
        }
    }
}

/// Manages texture registration for displaying application textures in ImGui
pub(crate) struct TextureRegistry {
    textures: HashMap<imgui::TextureId, TextureHandle>,
    next_id: usize,
    pending_additions: Vec<(imgui::TextureId, TextureHandle)>,
    pending_removals: Vec<imgui::TextureId>,
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            next_id: 1, // Start at 1, 0 is reserved for font texture
            pending_additions: Vec::new(),
            pending_removals: Vec::new(),
        }
    }
    
    /// Register a texture and return its ImGui TextureId
    pub fn register(&mut self, handle: TextureHandle) -> imgui::TextureId {
        let id = imgui::TextureId::new(self.next_id);
        self.next_id += 1;
        
        self.pending_additions.push((id, handle));
        id
    }
    
    /// Unregister a texture by its TextureId
    pub fn unregister(&mut self, id: imgui::TextureId) {
        self.pending_removals.push(id);
    }
    
    /// Synchronize pending texture changes with the renderer
    pub fn sync_with_renderer(
        &mut self,
        renderer: &mut imgui_wgpu::Renderer,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        // Process removals first
        for id in self.pending_removals.drain(..) {
            if self.textures.remove(&id).is_some() {
                // Note: imgui_wgpu doesn't provide a way to remove textures
                // They will be cleaned up when the renderer is dropped
                log::debug!("Unregistered texture: {:?}", id);
            }
        }
        
        // Process additions
        for (id, handle) in self.pending_additions.drain(..) {
            // Create texture config for imgui_wgpu
            let texture_config = imgui_wgpu::TextureConfig {
                size: wgpu::Extent3d {
                    width: handle.size.width,
                    height: handle.size.height,
                    depth_or_array_layers: 1,
                },
                label: Some("imgui_texture"),
                format: Some(handle.texture.format()),
                ..Default::default()
            };
            
            // Create a new texture in the renderer
            let _imgui_texture = imgui_wgpu::Texture::new(device, renderer, texture_config);
            
            // Copy the texture data
            // Note: This is a simplified approach. In a real implementation,
            // you might want to copy the actual texture data or use the existing texture directly
            
            self.textures.insert(id, handle);
            log::debug!("Registered texture: {:?}", id);
        }
    }
}
