use biospheres::scene::BasicScene;
use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

struct App {
    window: Arc<Window>,
    scene: BasicScene,
}

fn main() {
    println!("BioSpheres starting...");
    
    let event_loop = EventLoop::new().unwrap();
    
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("BioSpheres")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)
            .unwrap()
    );
    
    // Create basic scene
    let scene = pollster::block_on(BasicScene::new(window.clone()));
    println!("Scene initialized successfully");
    
    let mut app = App { window, scene };
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => {
                // Let the scene handle input first
                if app.scene.handle_input(&event) {
                    // ImGui consumed the event, request redraw
                    app.window.request_redraw();
                }
                
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Close requested, exiting...");
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        app.scene.resize(physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        // Render the scene with UI
                        match app.scene.render(&app.window) {
                            Ok((output, _view, encoder)) => {
                                app.scene.present(output, encoder);
                                
                                // Update cursor based on scene's desired cursor
                                let desired_cursor = app.scene.get_desired_cursor()
                                    .unwrap_or(winit::window::CursorIcon::Default);
                                app.window.set_cursor_icon(desired_cursor);
                            }
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                app.scene.resize(app.window.inner_size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                eprintln!("Out of memory!");
                                elwt.exit();
                            }
                            Err(e) => {
                                eprintln!("Surface error: {:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                app.window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
