use winit::{
    application::ApplicationHandler, 
    event::{WindowEvent}, 
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{Window, WindowId}
};

use glium::{
    glutin::surface::WindowSurface,
    Surface,
    Display
};


// #[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub display: Option<Display<WindowSurface>>,
    pub event_loop: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            display: None,
            event_loop: Some(EventLoop::new().unwrap()),
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Exited with close button.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let mut frame = self.display.as_ref().expect("Display Inexisting !").draw();
                frame.clear_color(0.0, 0.0, 1.0, 1.0);
                frame.finish().unwrap();
                self.window.as_ref().unwrap().request_redraw();
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