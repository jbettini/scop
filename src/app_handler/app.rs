use winit::{
    application::ApplicationHandler, 
    event::{WindowEvent,ElementState, KeyEvent}, 
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{Window, WindowId},
    keyboard::{KeyCode, PhysicalKey}
};

use glium::{
    glutin::surface::WindowSurface,
    Display
};

use super::shape::Triangle;

pub struct App {
    pub window: Option<Window>,
    pub display: Option<Display<WindowSurface>>,
    pub event_loop: Option<EventLoop<()>>,
    pub form: Triangle
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            display: None,
            event_loop: Some(EventLoop::new().unwrap()),
            form: Triangle::default()
        }
    }
    pub fn init_display(&mut self) {
        let event_loop = self.event_loop.as_ref().expect("EventLoop should be initialized");
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);
        let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_inner_size(1920, 1200)
            .with_title("Super Scop :O")
            .build(event_loop);
        self.display = Some(display);
        self.window = Some(_window);
    }
    pub fn run(mut self) {
        if let Some(event_loop) = self.event_loop.take() {
            let _ = event_loop.run_app(&mut self);
        } else {
            panic!("EventLoop not initialized");
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if self.display.is_none() {
            self.init_display();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().expect("Error: Window is not initialized !").request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested by the user.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.form.draw_triangle(self.display.as_ref().expect("Error: Display Inexisting!"));
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::KeyboardInput { device_id: _device_id, event, is_synthetic } => {
                match event {
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    } if !is_synthetic => {
                        println!("Escape key pressed - closing the application.");
                        event_loop.exit();
                    },
                    _ => {
                        // TODO: Autre input clavier
                    }
                }
            },
            WindowEvent::Resized(window_size) => {
                self.display.as_ref().expect("Error: Display not initialized.").resize(window_size.into());
            }
            _ => (),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}