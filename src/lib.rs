#![feature(option_zip)]

pub mod backend;
pub mod canvas;
pub mod draw;
pub mod element;
pub mod event;
pub mod layout;
pub mod widget;
pub mod state;

pub use self::{
    backend::Window,
    element::Element,
    widget::{Widget, StateWidget},
    state::State,
};

pub struct Response {
    redraw: bool,
}

impl Response {
    pub fn redraw(&mut self) {
        self.redraw = true;
    }
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum Event {
    CursorMove([f32; 2]),
    Click([f32; 2], MouseButton),
}
