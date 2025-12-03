mod ui;
use ui::{Router, Route};

mod yt_dlp;
use yt_dlp::{init};

use fltk::app;
use fltk::prelude::*;
use fltk::window::DoubleWindow;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

fn main() {
    let app = app::App::default();
    let mut wind = DoubleWindow::new(100, 100, 500, 300, "yt-dlp GUI");

    // Router 생성
    let router = Rc::new(RefCell::new(ui::Router::new()));

    wind.end();
    wind.show();

    // 처음에는 Splash 화면
    router.borrow_mut().navigate(Route::Splash);
    match init::Init::ensure() {
        Ok(_) => println!("Initialization complete."),
        Err(e) => eprintln!("Error during init: {}", e),
    }
    // 2초 후 Home 화면으로 전환
    {
        let router_clone = router.clone();
        app::add_timeout3(2.0, move |_| {
            router_clone.borrow_mut().navigate(Route::Home);
        });
    }

    app.run().unwrap();
}
