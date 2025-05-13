use std::env;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use gfx;

mod config_engine;
mod log_setup;
mod pane;

struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<gfx::GFXState<'a>>,
    config: config_engine::Config,

    panes: Vec<pane::pane::Pane>,
    active_pane: usize,
}

impl Default for App<'_> {
    fn default() -> Self {
        App {
            window: None,
            state: None,
            config: config_engine::Config::default(),
            panes: Vec::new(),
            active_pane: 0,
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        // attributes.title = "coral".to_string();
        // attributes.inner_size = Some(LogicalSize::new(800, 600).into());

        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.window = Some(window.clone());
        self.state = Some(pollster::block_on(gfx::GFXState::new(window.clone())));

        // TODO: Make this more generic
        self.panes.push(pane::pane::Pane::new_from_file(""));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if match self.state.as_mut() {
            Some(s) => s.input(&event),
            None => false,
        } {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();

                if let Some(state) = self.state.as_mut() {
                    if let Some(active) = self.panes.get_mut(self.active_pane) {
                        active.generate_render_cmds(state);
                        state.set_inner_render_cmds(active.render_cmds.clone());
                    }

                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            log::error!("WGPU out of memeory");
                            event_loop.exit();
                        }

                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timed out");
                        }
                    }
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(s) = self.state.as_mut() {
                    s.resize(physical_size);
                }
            }
            _ => (),
        }
    }
}

fn main() {
    log_setup::setup();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    app.config = config_engine::Config::new();
    event_loop.run_app(&mut app);
}
