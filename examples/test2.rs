pub trait Widget<D> {
    fn on<E>(mut self, f: impl FnMut(E, EventCtx<D, Self>) + 'static) -> Self
        where Self: Sized + Handler<D, E>
    {
        self.attach(f);
        self
    }

    fn finish<'a>(self) -> Element<'a, D> where Self: Sized + 'a {
        Element {
            widget: Box::new(self),
        }
    }
}

pub struct Element<'a, D> {
    widget: Box<dyn Widget<D> + 'a>,
}

// State

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

// Events

pub trait Handler<D, E> {
    fn attach(&mut self, f: impl FnMut(E, EventCtx<D, Self>) + 'static);
}

pub struct EventCtx<'a, D, W> {
    pub widget: &'a mut W,
    pub data: &'a mut D,
}

pub trait HandlerFn<D, E, W> = FnMut(E, EventCtx<D, W>);

pub type Handle<D, W, E> = Option<Box<dyn FnMut(E, EventCtx<D, W>)>>;

pub struct Click;

// Toggle

pub struct Toggle<D> {
    state: State<D, bool>,
    on_click: Handle<D, Self, Click>,
}

impl<D> Toggle<D> {
    pub fn new(state: impl Into<bool>) -> Self {
        Self {
            state: State::Inner(state.into()),
            on_click: None,
        }
    }

    pub fn bind(f: impl for<'a> FnMut(&'a mut D) -> &'a mut bool + 'static) -> Self {
        Self {
            state: State::Map(Box::new(f)),
            on_click: None,
        }
    }
}

impl<D> Widget<D> for Toggle<D> {}

impl<D> Handler<D, Click> for Toggle<D> {
    fn attach(&mut self, f: impl FnMut(Click, EventCtx<D, Self>) + 'static) {
        self.on_click = Some(Box::new(f));
    }
}

// Label

pub struct Label<D> {
    state: State<D, String>,
}

impl<D> Label<D> {
    pub fn new(state: impl Into<String>) -> Self {
        Self {
            state: State::Inner(state.into()),
        }
    }

    pub fn bind(f: impl for<'a> FnMut(&'a mut D) -> &'a mut String + 'static) -> Self {
        Self {
            state: State::Map(Box::new(f)),
        }
    }
}

impl<D> Widget<D> for Label<D> {}

fn main() {
    struct Data {
        enabled: bool,
    }

    Label::<Data>::new("Hello, world!")
        .finish();

    Toggle::<Data>::new(false)
        .finish();

    Toggle::<Data>::bind(|s| &mut s.enabled)
        .on(|Click, _| println!("Toggled!"))
        .finish();
}
