use vek::*;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Copy, Clone)]
pub struct Bounds {
    pub(crate) rect: Rect<f32, f32>,
}

impl Bounds {
    pub fn global(size: [f32; 2]) -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, size[0], size[1]),
        }
    }

    pub fn pos(&self) -> [f32; 2] {
        self.rect.position().into_array()
    }

    pub fn size(&self) -> [f32; 2] {
        self.rect.extent().into_array()
    }

    pub fn window(&self, pos: [f32; 2], size: [f32; 2]) -> Self {
        let pos = [pos[0].min(self.rect.w), pos[1].min(self.rect.h)];
        Self {
            rect: Rect::new(
                self.rect.x + pos[0],
                self.rect.y + pos[1],
                size[0].min(self.rect.w - pos[0]),
                size[1].min(self.rect.h - pos[1]),
            ),
        }
    }

    pub fn padded_window(&self, padding: f32) -> Self {
        Self {
            rect: Rect::new(
                self.rect.x + padding.min(self.rect.w / 2.0),
                self.rect.y + padding.min(self.rect.h / 2.0),
                self.rect.w - padding.min(self.rect.w / 2.0) * 2.0,
                self.rect.h - padding.min(self.rect.h / 2.0) * 2.0,
            ),
        }
    }

    pub fn contains(&self, point: impl Into<Vec2<f32>>) -> bool {
        self.rect.contains_point(point.into())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub(crate) min: f32,
    max: Option<f32>,
}

impl Span {
    pub fn fill() -> Self {
        Self::min(0.0)
    }

    pub fn min(min: f32) -> Self {
        Self {
            min,
            max: None,
        }
    }

    pub fn exactly(x: f32) -> Self {
        Self {
            min: x,
            max: Some(x),
        }
    }

    pub fn zero() -> Self {
        Self {
            min: 0.0,
            max: Some(0.0),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            min: self.min.max(other.min),
            max: match (self.max, other.max) {
                (Some(s), Some(o)) => Some(s.min(o)),
                (s, o) => s.or(o),
            },
        }
    }
}

impl std::ops::Add for Span {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            min: self.min + other.min,
            max: match (self.max, other.max) {
                (Some(s), Some(o)) => Some(s + o),
                (_, _) => None,
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LayoutReq {
    span: Extent2<Span>,
}

impl LayoutReq {
    pub fn new(span: [Span; 2]) -> Self {
        Self { span: span.into() }
    }

    pub fn any() -> Self {
        Self::new([Span::min(0.0), Span::min(0.0)])
    }

    pub fn padded(self, padding: f32) -> Self {
        Self {
            span: self.span.map(|e| Span {
                min: e.min + padding * 2.0,
                max: e.max.map(|max| max + padding * 2.0),
            }),
        }
    }

    pub fn width(&self) -> Span {
        self.span.w
    }

    pub fn height(&self) -> Span {
        self.span.h
    }
}

impl std::ops::Index<usize> for LayoutReq {
    type Output = Span;

    fn index(&self, index: usize) -> &Self::Output {
        &self.span[index]
    }
}
