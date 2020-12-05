use std::{
    collections::HashMap,
    any::{Any, TypeId},
};

pub trait Widget: Default {
    type State;

    fn new<D, S: Into<Self::State>>(state: S) -> Element<D, Self> {
        Element {
            state: State::Inner(state.into()),
            widget: Default::default(),
            handlers: HashMap::default(),
        }
    }

    fn bind<D, F: for<'a> FnMut(&'a mut D) -> &'a mut Self::State + 'static>(f: F) -> Element<D, Self> {
        Element {
            state: State::Map(Box::new(f)),
            widget: Default::default(),
            handlers: HashMap::default(),
        }
    }
}

enum State<D, S> {
    Map(Box<dyn for<'a> FnMut(&'a mut D) -> &'a mut S>),
    Inner(S),
}

impl<D, S> State<D, S> {
    fn get_mut<'a, 'b: 'a>(&'b mut self, data: &'a mut D) -> &'a mut S {
        match self {
            State::Map(f) => f(data),
            State::Inner(s) => s,
        }
    }
}

pub struct Element<D, W: Widget> {
    state: State<D, W::State>,
    widget: W,
    handlers: HashMap<TypeId, Box<dyn FnMut(&mut D)>>,
}

impl<D, W: Widget> Element<D, W> {
    pub fn on<E: Event<W>, F: for<'a> FnMut(&'a mut D) + 'static>(mut self, _: E, f: F) -> Self {
        self.handlers.insert(TypeId::of::<E>(), Box::new(f));
        self
    }
}

pub trait Event<W: Widget>: Any {}

// Switch

#[derive(Default)]
pub struct Switch;

impl Widget for Switch {
    type State = bool;
}

// Events

pub struct Click;

impl Event<Switch> for Click {}

// Window

pub struct Window<D, W: Widget> {
    root: Element<D, W>,
}

impl<D, W: Widget> Window<D, W> {
    pub fn new(root: Element<D, W>) -> Self {
        Self { root }
    }

    pub fn run(self, data: D) {
        // TODO
    }
}

fn main() {
    struct Data {
        enabled: bool,
    }

    let switch = Switch::bind::<Data, _>(|d| &mut d.enabled)
        .on(Click, |_| println!("Hello, world!"));

    let switch2 = Switch::new(false);

    let list = List::new([
        switch,
        switch2,
    ]);

    Window::new(switch)
        .run(Data {
            enabled: true,
        })
}
