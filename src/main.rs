use winit::event_loop::{ControlFlow, EventLoop};

mod app_handler;
use app_handler::App;

 fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}