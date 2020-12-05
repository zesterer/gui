use crate::{
    draw::Color,
    canvas::Canvas,
    layout::{Bounds, LayoutReq},
    event::{Handle, Handler, Click, EventCtx},
    Widget, StateWidget, State, Event, Element, Response,
};

pub struct Toggle<'a, D> {
    is_hover: bool,
    is_pressed: bool,
    state: State<'a, D, bool>,
    inner: Option<Element<'a, D>>,
    on_click: Handle<'a, D, Click, Self>,
}

impl<'a, D> Toggle<'a, D> {
    pub fn containing(mut self, inner: impl Widget<'a, D> + 'a) -> Self {
        self.inner = Some(inner.finish());
        self
    }
}

impl<'a, D> StateWidget<'a, D, bool> for Toggle<'a, D> {
    fn from_state(state: State<'a, D, bool>) -> Self {
        Self {
            is_hover: false,
            is_pressed: false,
            inner: None,
            state,
            on_click: None,
        }
    }
}

impl<'a, D> Widget<'a, D> for Toggle<'a, D> {
    fn children(&mut self) -> Box<dyn Iterator<Item=&mut Element<'a, D>> + '_> {
        Box::new(self.inner.as_mut().into_iter())
    }

    fn handle(
        &mut self,
        data: &mut D,
        event: &Event,
        bounds: Bounds,
        resp: &mut Response,
    ) -> bool {
        if self.inner
            .as_mut()
            .map(|x| x.handle(data, event, resp))
            .unwrap_or(false)
        {
            true
        } else {
            if let Event::Click(pos, _) = event {
                if bounds.contains(*pos) {
                    let state = self.state.get_mut(data);
                    *state ^= true;
                    self.on_click.as_mut().map(|f| f(EventCtx {
                        widget: std::marker::PhantomData,//self,
                        data,
                    }));
                    resp.redraw();
                    true
                } else {
                    false
                }
            } else if let Event::CursorMove(pos) = event {
                let old_hover = self.is_hover;
                self.is_hover = bounds.contains(*pos);
                if old_hover != self.is_hover {
                    resp.redraw();
                }
                false
            } else {
                false
            }
        }
    }

    fn get_layout_req(&mut self) -> LayoutReq {
        self.inner
            .as_mut()
            .map(|i| i.get_layout_req())
            .unwrap_or(LayoutReq::any())
    }

    fn fit_bounds(&mut self, bounds: Bounds) {
        self.inner.as_mut().map(|i| i.fit_bounds(bounds));
    }

    fn draw(
        &mut self,
        data: &mut D,
        bounds: Bounds,
        canvas: &mut Canvas,
    ) {
        canvas.bounded(bounds).fill(if self.is_pressed {
            Color::RED
        } else if self.is_hover {
            Color::YELLOW
        } else if *self.state.get_mut(data) {
            Color::BLUE
        } else {
            Color::WHITE
        });
        self.inner
            .as_mut()
            .map(|x| x.draw(data, canvas));
    }
}

impl<'a, D> Handler<'a, D, Click> for Toggle<'a, D> {
    fn attach(&mut self, f: impl FnMut(EventCtx<D, Click, Self>) + 'a) {
        self.on_click = Some(Box::new(f));
    }
}
