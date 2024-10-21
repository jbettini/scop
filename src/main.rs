mod app;
use app::app::App;

fn main() {
    let mut app = App::default();
    app.init_display();
    app.run();

}