use crate::{
    draw::Color,
    canvas::Canvas,
    layout::{Bounds, Direction, LayoutReq, Span},
    event::{Handle, Handler, Click, EventCtx},
    Widget, State, Event, Element, Response,
};

pub struct List<'a, D> {
    dir: Direction,
    children: Vec<Element<'a, D>>,
}

impl<'a, D> List<'a, D> {
    pub fn horizontal() -> Self { Self::new(Direction::Horizontal) }
    pub fn vertical() -> Self { Self::new(Direction::Vertical) }

    pub fn new(dir: Direction) -> Self {
        Self {
            dir,
            children: Vec::new()
        }
    }

    pub fn push(mut self, child: impl Widget<'a, D> + 'a) -> Self {
        self.children.push(child.finish());
        self
    }

    fn child_bounds(&self, bounds: Bounds) -> impl Fn(usize) -> Bounds {
        let child_count = self.children.len();
        let dir = self.dir;
        move |i| if dir == Direction::Horizontal {
            bounds.window(
                [i as f32 * bounds.size()[0] / child_count as f32, 0.0],
                [bounds.size()[0] / child_count as f32, bounds.size()[1]]
            )
        } else {
            bounds.window(
                [0.0, i as f32 * bounds.size()[1] / child_count as f32],
                [bounds.size()[0], bounds.size()[1] / child_count as f32]
            )
        }
    }
}

impl<'a, D> Widget<'a, D> for List<'a, D> {
    fn children(&mut self) -> Box<dyn Iterator<Item=&mut Element<'a, D>> + '_> {
        Box::new(self.children.iter_mut())
    }

    fn handle(
        &mut self,
        data: &mut D,
        event: &Event,
        bounds: Bounds,
        resp: &mut Response,
    ) -> bool {
        self.children
            .iter_mut()
            .enumerate()
            .any(|(i, child)| {
                child.handle(data, event, resp)
            })
    }

    fn get_layout_req(&mut self) -> LayoutReq {
        let dir = self.dir;
        self.children.iter_mut().fold(
            LayoutReq::new([Span::zero(); 2]),
            |l, c| {
                let c_l = c.get_layout_req();
                LayoutReq::new(if dir == Direction::Horizontal {[
                    l.width() + c_l.width(),
                    l.height().max(c_l.height()),
                ]} else {[
                    l.width().max(c_l.width()),
                    l.height() + c_l.height(),
                ]})
            },
        )
    }

    fn fit_bounds(&mut self, bounds: Bounds) {
        let dir = self.dir;

        let req_space = self.children.iter().map(|child| child.last_layout_req()[dir as usize].min).sum::<f32>();
        let spare_space = (bounds.size()[dir as usize] - req_space).max(0.0);
        let min_factor = (bounds.size()[dir as usize] / req_space).min(1.0);

        let child_count = self.children.len();

        let mut offset = 0.0;
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(i, child)| {
                let sz = child.last_layout_req()[dir as usize].min * min_factor + spare_space / child_count as f32;
                child.fit_bounds(bounds.window(
                    if dir == Direction::Horizontal { [offset, 0.0] } else { [0.0, offset] },
                    if dir == Direction::Horizontal {
                        [sz, bounds.size()[1]]
                    } else {
                        [bounds.size()[0], sz]
                    },
                ));
                offset += sz;
            });
    }

    fn draw(
        &mut self,
        data: &mut D,
        bounds: Bounds,
        canvas: &mut Canvas,
    ) {
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(i, child)| {
                child.draw(data, canvas)
            });
    }
}
