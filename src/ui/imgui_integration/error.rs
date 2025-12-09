use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImguiError {
    #[error("Renderer error: {0}")]
    RendererError(#[from] imgui_wgpu::RendererError),
    
    #[error("Texture not loaded")]
    TextureNotLoaded,
    
    #[error("Invalid texture format")]
    InvalidTextureFormat,
    
    #[error("Window not available")]
    WindowNotAvailable,
}
