mod app_handler;
use app_handler::App;
// use std::sync::Arc;

// pub static mut H: Arc<u32> = Arc::new(1920);
fn main() {
    let mut app = App::default();
    app.init_display();
    app.run();
}