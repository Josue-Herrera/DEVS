
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop, ActiveEventLoop},
    window::{Window, WindowId, WindowAttributes},
    event::{WindowEvent, Event}
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    // Proceed with next loop iteration right after prior finishes
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Error: {}", e);
    }

}

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Ok(window) = event_loop.create_window(WindowAttributes::default()) {
            let window_handle = Arc::new(window);
            self.window = Some(window_handle.clone());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            },
            WindowEvent::Resized(_) => {
                println!("Resize requested");
            },
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");

            },
            _ => {},
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }

}

