use crate::{
    draw::Color,
    canvas::Canvas,
    layout::{Bounds, LayoutReq, Span},
    Widget, StateWidget, Event, Response, State,
};

pub struct Label<'a, D> {
    state: State<'a, D, String>,
}

impl<'a, D> StateWidget<'a, D, String> for Label<'a, D> {
    fn from_state(state: State<'a, D, String>) -> Self {
        Self { state }
    }
}

impl<'a, D> Widget<'a, D> for Label<'a, D> {
    fn get_layout_req(&mut self) -> LayoutReq {
        LayoutReq::new([
            Span::exactly(96.0),
            Span::exactly(32.0),
        ])
    }

    fn draw(
        &mut self,
        data: &mut D,
        bounds: Bounds,
        canvas: &mut Canvas,
    ) {
        canvas.bounded(bounds).draw_text([0.0; 2], self.state.get_mut(data).clone(), 20.0, Color::BLACK);
    }
}
