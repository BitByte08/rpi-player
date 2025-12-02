use crate::ui::screens::{SplashScreen, HomeScreen};

#[derive(Clone, Copy)]
pub enum Route {
    Splash,
    Home,
}

pub struct Router {
    pub current: Route,
    pub splash: SplashScreen,
    pub home: HomeScreen,
}

impl Router {
    pub fn new() -> Self {
        Self {
            current: Route::Splash,
            splash: SplashScreen::new(),
            home: HomeScreen::new(),
        }
    }

    pub fn navigate(&mut self, route: Route) {
        // 모든 화면 숨기기
        self.splash.hide();
        self.home.hide();

        // 이동할 화면 보여주기
        match route {
            Route::Splash => self.splash.show(),
            Route::Home => self.home.show(),
        }

        self.current = route;
    }
}
