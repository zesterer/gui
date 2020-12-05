pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Color::new(0x0, 0x0, 0x0, 0xFF);
    pub const WHITE: Self = Color::new(0xFF, 0xFF, 0xFF, 0xFF);
    pub const RED: Self = Color::new(0xFF, 0x0, 0x0, 0xFF);
    pub const GREEN: Self = Color::new(0x0, 0xFF, 0x0, 0xFF);
    pub const BLUE: Self = Color::new(0x0, 0x0, 0xFF, 0xFF);
    pub const YELLOW: Self = Color::new(0xFF, 0xFF, 0x0, 0xFF);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

pub enum Fill {
    Color(Color),
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Fill::Color(color)
    }
}

pub struct Stroke {
    pub width: f32,
    pub fill: Fill,
    // TODO: Join, Hip, etc.
}

impl<T: Into<Fill>> From<T> for Stroke {
    fn from(fill: T) -> Self {
        Self {
            width: 1.0,
            fill: fill.into(),
        }
    }
}
