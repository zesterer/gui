use crate::{
    draw::{Fill, Stroke, Color},
    layout::Bounds,
};
use vek::*;

pub enum Primitive {
    Rect { rect: Rect<f32, f32>, fill: Fill },
    Line { line: LineSegment2<f32>, stroke: Stroke },
    Text { pos: Vec2<f32>, text: String, height: f32, col: Color },
}

#[derive(Default)]
pub struct Canvas {
    pub(crate) primitives: Vec<Primitive>,
}

impl Canvas {
    pub fn bounded<'a>(&'a mut self, bounds: Bounds) -> BoundedCanvas<'a> {
        BoundedCanvas {
            aabr: Aabr {
                min: Vec2::from(bounds.pos()),
                max: Vec2::from(bounds.pos()) + Vec2::from(bounds.size()),
            },
            canvas: self,
        }
    }
}

pub struct BoundedCanvas<'a> {
    aabr: Aabr<f32>,
    canvas: &'a mut Canvas,
}

impl<'a> BoundedCanvas<'a> {
    pub fn size(&self) -> [f32; 2] { self.aabr.size().into_array() }

    pub fn bounds(&self) -> Bounds {
        Bounds { rect: self.aabr.into() }
    }

    pub fn draw_text(&mut self, pos: [f32; 2], text: impl Into<String>, height: f32, col: impl Into<Color>) {
        self.canvas.primitives.push(Primitive::Text {
            pos: self.aabr.min + Vec2::from(pos),
            text: text.into(),
            height,
            col: col.into(),
        });
    }

    pub fn draw_line(&mut self, from: [f32; 2], to: [f32; 2], stroke: impl Into<Stroke>) {
        self.canvas.primitives.push(Primitive::Line {
            line: LineSegment2 {
                start: self.aabr.min + Vec2::from(from),
                end: self.aabr.min + Vec2::from(to),
            },
            stroke: stroke.into(),
        });
    }

    pub fn draw_rect(&mut self, pos: [f32; 2], size: [f32; 2], fill: impl Into<Fill>) {
        self.canvas.primitives.push(Primitive::Rect {
            rect: Rect::new(
                self.aabr.min.x + pos[0],
                self.aabr.min.y + pos[1],
                size[0],
                size[1],
            ),
            fill: fill.into(),
        });
    }

    pub fn fill(&mut self, fill: impl Into<Fill>) {
        self.draw_rect([0.0; 2], self.size(), fill)
    }
}
