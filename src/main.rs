use biospheres::scene::BasicScene;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Arc<Window>>,
    scene: Option<BasicScene>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("BioSpheres")
                            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
                    )
                    .unwrap()
            );
            
            // Create basic scene
            let scene = pollster::block_on(BasicScene::new(window.clone()));
            
            println!("Scene initialized successfully");
            
            self.window = Some(window);
            self.scene = Some(scene);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(scene) = &mut self.scene {
                    scene.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(scene)) = (&self.window, &mut self.scene) {
                    // Render the scene
                    match scene.render() {
                        Ok((output, _view, encoder)) => {
                            scene.present(output, encoder);
                        }
                        Err(wgpu::SurfaceError::Lost) => {
                            scene.resize(window.inner_size());
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            eprintln!("Out of memory!");
                            event_loop.exit();
                        }
                        Err(e) => {
                            eprintln!("Surface error: {:?}", e);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    println!("BioSpheres starting...");
    
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        scene: None,
    };
    
    event_loop.run_app(&mut app).unwrap();
}
