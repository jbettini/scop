// mod window {
//     pub mod manager;
// }

use winit::{
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    application::ApplicationHandler,
    window::{Window, WindowAttributes, WindowId},
    event::{Event, WindowEvent},
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

 fn main() {
    let event_loop = EventLoop::new().unwrap();
    // let window_attributes = Window::default_attributes()
    //             .with_title("Fantastic window number one!")
    //             .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
    // let window = event_loop.create_window(window_attributes).unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app);
}