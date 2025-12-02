mod yt_dlp;
use yt_dlp::ensure_and_download;
use fltk::{app, button::Button, frame::Frame, input::Input, prelude::*, window::Window};
use fltk_theme::{WidgetScheme, SchemeType};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut wind = Window::new(100, 100, 500, 250, "yt-dlp GUI");
    
    let mut input = Input::new(50, 50, 400, 40, "Search:");
    let mut frame = Frame::new(50, 150, 400, 40, "Ready");
    let mut btn = Button::new(200, 110, 100, 30, "Download");

    btn.set_callback(move |_| {
        let keyword = input.value();
        match ensure_and_download(&keyword) {
            Ok(_) => frame.set_label("Download OK"),
            Err(e) => frame.set_label(&format!("Error: {}", e)),
        }
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
