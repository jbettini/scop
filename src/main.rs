mod app_handler;
use app_handler::App;


fn main() {
    let mut app = App::default();
    app.init_display();
    if let Some(window) = &app.window {
        window.request_redraw();
    }
    app.run();
}