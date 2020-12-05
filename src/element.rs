use crate::{
    canvas::Canvas,
    layout::{Bounds, LayoutReq},
    Widget, Event, Response,
};
use vek::*;
use std::ops::{Deref, DerefMut};

pub struct Element<'a, D> {
    widget: Box<dyn Widget<'a, D> + 'a>,
    bounds: Bounds,
    last_layout: LayoutReq,
}

impl<'a, D> Element<'a, D> {
    pub(crate) fn from_widget(widget: impl Widget<'a, D> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
            bounds: Bounds::global([0.0; 2]),
            last_layout: LayoutReq::any(),
        }
    }

    pub fn last_layout_req(&self) -> LayoutReq {
        self.last_layout
    }

    pub(crate) fn get_layout_req(&mut self) -> LayoutReq {
        let layout = self.widget.get_layout_req();
        self.last_layout = layout;
        layout
    }

    pub(crate) fn fit_bounds(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        self.widget.fit_bounds(bounds);
    }

    pub(crate) fn handle(
        &mut self,
        data: &mut D,
        event: &Event,
        resp: &mut Response,
    ) -> bool {
        self.widget.handle(data, event, self.bounds, resp)
    }

    pub(crate) fn draw(
        &mut self,
        data: &mut D,
        canvas: &mut Canvas,
    ) {
        self.widget.draw(data, self.bounds, canvas)
    }
}
