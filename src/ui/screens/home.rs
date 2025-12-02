use fltk::{group::Group, prelude::*, frame::Frame, app};
use std::cell::RefCell;

pub struct HomeScreen {
    pub group: RefCell<Group>,
}

impl HomeScreen {
    pub fn new() -> Self {
        let mut group = Group::new(0, 0, 500, 300, None);
        let mut label = Frame::new(200, 120, 100, 40, "Home");
        group.end();
        group.hide(); // 처음에는 숨김
        Self { group : RefCell::new(group) }
    }

    pub fn show(&self) { self.group.borrow_mut().show(); }
    pub fn hide(&self) { self.group.borrow_mut().hide(); }
}