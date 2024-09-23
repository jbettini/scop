mod app_handler;
use app_handler::App;


fn main() {
    let mut app = App::default();
    app.init_display();
    app.run();
}