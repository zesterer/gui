pub mod button;
pub mod label;
pub mod list;
pub mod padded;
pub mod toggle;

pub use self::{
    button::Button,
    label::Label,
    list::List,
    padded::Padded,
    toggle::Toggle,
};

use crate::{
    event::{EventCtx, Handler},
    element::Element,
    layout::{Bounds, LayoutReq},
    canvas::Canvas,
    Event, Response, State,
};

pub trait Widget<'a, D> {
    fn with_state<S>(state: impl Into<S>) -> Self
        where Self: Sized + StateWidget<'a, D, S>
    {
        Self::from_state(State::Inner(state.into()))
    }

    fn gen_state<S>(f: impl FnMut(&mut D) -> S + 'a) -> Self
        where
            Self: Sized + StateWidget<'a, D, S>,
            S: Default,
    {
        Self::from_state(State::Generate(S::default(), Box::new(f)))
    }

    fn default_state<S>() -> Self
        where
            Self: Sized + StateWidget<'a, D, S>,
            S: Default,
    {
        Self::from_state(State::Inner(S::default()))
    }

    fn bind_state<S>(f: impl for<'b> FnMut(&'b mut D) -> &'b mut S + 'a) -> Self
        where Self: Sized + StateWidget<'a, D, S>,
    {
        Self::from_state(State::Bind(Box::new(f)))
    }

    fn on<E>(mut self, event: E, f: impl FnMut(EventCtx<D, E, Self>) + 'a) -> Self
        where Self: Sized + Handler<'a, D, E>
    {
        self.attach(f);
        self
    }

    fn padded(self, padding: f32) -> Padded<'a, D, Self> where Self: Sized {
        Padded::new(self, padding)
    }

    fn finish(self) -> Element<'a, D> where Self: Sized + 'a {
        Element::from_widget(self)
    }

    fn get_layout_req(&mut self) -> LayoutReq;

    fn fit_bounds(&mut self, bounds: Bounds) {}

    fn children(&mut self) -> Box<dyn Iterator<Item=&mut Element<'a, D>> + '_> {
        Box::new(std::iter::empty())
    }

    fn handle(
        &mut self,
        data: &mut D,
        event: &Event,
        bounds: Bounds,
        resp: &mut Response,
    ) -> bool { false }

    fn draw(
        &mut self,
        data: &mut D,
        bounds: Bounds,
        canvas: &mut Canvas,
    ) {}
}

pub trait StateWidget<'a, D, S>: Widget<'a, D> {
    fn from_state(state: State<'a, D, S>) -> Self;
}
