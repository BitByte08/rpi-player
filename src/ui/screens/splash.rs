use fltk::{group::Group, prelude::*, frame::Frame};
use std::cell::RefCell;

pub struct SplashScreen {
    pub group: RefCell<Group>,
    pub label: Frame,
}

impl SplashScreen {
    pub fn new() -> Self {
        let mut group = Group::new(0, 0, 500, 300, None);
        let label = Frame::new(150, 120, 200, 60, "Loading...");
        group.end();
        group.show();
        Self { 
            group : RefCell::new(group), 
            label 
        }
    }

    pub fn show(&self) { self.group.borrow_mut().show(); }
    pub fn hide(&self) { self.group.borrow_mut().hide(); }
}