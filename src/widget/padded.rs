use crate::{
    draw::Color,
    canvas::Canvas,
    layout::{Bounds, LayoutReq},
    event::{Handle, Handler, Click, EventCtx},
    Widget, State, Event, Element, Response,
};
use std::marker::PhantomData;

pub struct Padded<'a, D, W: Widget<'a, D>> {
    padding: f32,
    inner: W,
    phantom: PhantomData<&'a D>,
}

impl<'a, D, W: Widget<'a, D>> Padded<'a, D, W> {
    pub fn new(inner: W, padding: f32) -> Self {
        Self {
            padding,
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, D, W: Widget<'a, D>> Widget<'a, D> for Padded<'a, D, W> {
    fn handle(
        &mut self,
        data: &mut D,
        event: &Event,
        bounds: Bounds,
        resp: &mut Response,
    ) -> bool {
        self.inner.handle(data, event, bounds.padded_window(self.padding), resp)
    }

    fn get_layout_req(&mut self) -> LayoutReq {
        self.inner
            .get_layout_req()
            .padded(self.padding)
    }

    fn fit_bounds(&mut self, bounds: Bounds) {
        self.inner.fit_bounds(bounds.padded_window(self.padding))
    }

    fn draw(
        &mut self,
        data: &mut D,
        bounds: Bounds,
        canvas: &mut Canvas,
    ) {
        self.inner.draw(data, bounds.padded_window(self.padding), canvas)
    }
}

impl<'a, D, E, W: Widget<'a, D> + Handler<'a, D, E>> Handler<'a, D, E> for Padded<'a, D, W> {
    fn attach(&mut self, mut f: impl FnMut(EventCtx<D, E, Self>) + 'a) {
        self.inner.attach(move |ctx| f(ctx.map()));
    }
}
