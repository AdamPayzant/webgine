use std::env;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

mod log_setup;
mod wgpu_handler;

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<wgpu_handler::State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        // attributes.title = "coral".to_string();
        // attributes.inner_size = Some(LogicalSize::new(800, 600).into());

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.window = Some(window.clone());
        self.state = Some(pollster::block_on(wgpu_handler::State::new(window.clone())));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    unsafe {
        env::set_var("WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER", "1");
    }
    log_setup::setup();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app);
}
