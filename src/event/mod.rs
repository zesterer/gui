pub mod click;

pub use self::{
    click::Click,
};

pub trait Handler<'a, D, E> {
    fn attach(&mut self, f: impl FnMut(EventCtx<D, E, Self>) + 'a);
}

pub struct EventCtx<'a, D, E, W> {
    pub widget: std::marker::PhantomData<(E, W)>,//&'a mut W,
    pub data: &'a mut D,
}

impl<'a, D, E, W> EventCtx<'a, D, E, W> {
    pub fn map<U>(self) -> EventCtx<'a, D, E, U> {
        EventCtx {
            widget: std::marker::PhantomData,
            data: self.data,
        }
    }
}

pub type Handle<'a, D, E, W> = Option<Box<dyn FnMut(EventCtx<D, E, W>) + 'a>>;
