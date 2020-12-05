use crate::{
    draw::Fill,
    layout::Bounds,
    canvas::{Primitive, Canvas},
    Widget, Event, Element, MouseButton, Response,
};
use std::{
    convert::{TryFrom, TryInto},
    ops::Deref,
};
use cairo::{Format, ImageSurface, Context};
use piet::{kurbo::{Rect, Line}, RenderContext, Text, TextLayoutBuilder, TextAttribute};
use piet_cairo::CairoRenderContext;

pub struct Window<'a, D> {
    win: minifb::Window,
    element: Element<'a, D>,
    font_data: Vec<u8>,
}

impl<'a, D> Window<'a, D> {
    pub fn new(root: impl Widget<'a, D> + 'a) -> Self {
        Self {
            win: minifb::Window::new(
                "Test",
                640,
                480,
                minifb::WindowOptions {
                    resize: true,
                    ..Default::default()
                },
            ).unwrap(),
            element: root.finish(),
            font_data: include_bytes!("../../data/OpenSans-Regular.ttf").to_vec(),
        }
    }

    pub fn run(mut self, mut data: D) {
        let mut last_mouse_pos = None;
        let mut last_size = (0, 0);
        let mut mouse_down = false;

        //let mut buf = Vec::new();
        let mut surf = ImageSurface::create(Format::ARgb32, self.win.get_size().0 as i32, self.win.get_size().1 as i32).unwrap();
        let mut maybe_font = None;
        let mut redraw = true;

        while self.win.is_open() {
            let (w, h) = self.win.get_size();

            // Collect events
            let mut events = Vec::new();
            let mouse_pos = self.win.get_mouse_pos(minifb::MouseMode::Pass);
            if mouse_pos != last_mouse_pos {
                mouse_pos.map(|mouse_pos| events.push(Event::CursorMove([mouse_pos.0, mouse_pos.1])));
                last_mouse_pos = mouse_pos;
            }
            mouse_down = if self.win.get_mouse_down(minifb::MouseButton::Left) {
                if !mouse_down {
                    if let Some(mouse_pos) = mouse_pos {
                        events.push(Event::Click([mouse_pos.0, mouse_pos.1], MouseButton::Left));
                    }
                }
                true
            } else {
                false
            };

            if last_size != self.win.get_size() {
                last_size = self.win.get_size();
                surf = ImageSurface::create(Format::ARgb32, self.win.get_size().0 as i32, self.win.get_size().1 as i32).unwrap();
                redraw = true;
                self.element.get_layout_req();
                self.element.fit_bounds(Bounds::global([w as f32, h as f32]));
            }

            for event in events {
                let mut resp = Response {
                    redraw: false,
                };
                self.element.handle(&mut data, &event, &mut resp);
                redraw |= resp.redraw;
            }

            if redraw {
                //buf.resize_with(w * h * 4, || 0);

                // Collect widget primitives
                let mut prim_canvas = Canvas::default();
                self.element.draw(&mut data, &mut prim_canvas);

                // Draw primitives
                //let mut canvas = andrew::Canvas::new(&mut buf, w, h, w * 4, andrew::Endian::native());
                //canvas.pixel_size = 4;

                {
                    let cx = Context::new(&surf);
                    let mut rcx = CairoRenderContext::new(&cx);
                    rcx.clear(piet::Color::grey(0.5));

                        for prim in prim_canvas.primitives {
                        match prim {
                            Primitive::Rect { rect, fill } => {
                                let brush = match fill {
                                    Fill::Color(col) => rcx.solid_brush(piet::Color::rgba8(col.r, col.g, col.b, col.a)),
                                };
                                rcx.fill(Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.w) as f64, (rect.y + rect.h) as f64), &brush);

                                // canvas
                                //     .draw(&andrew::shapes::rectangle::Rectangle::new(
                                //         rect.position().map(|e| e as usize).into_tuple(),
                                //         rect.extent().map(|e| e as usize).into_tuple(),
                                //         None,
                                //         Some(match fill {
                                //             Fill::Color(col) => [col.a, col.r, col.g, col.b],
                                //         }),
                                //     ))
                            },
                            Primitive::Line { line, stroke } => {
                                let brush = match stroke.fill {
                                    Fill::Color(col) => rcx.solid_brush(piet::Color::rgba8(col.r, col.g, col.b, col.a)),
                                };
                                rcx.stroke(Line::new(
                                    line.start.map(|e| e as f64).into_tuple(),
                                    line.end.map(|e| e as f64).into_tuple(),
                                ), &brush, stroke.width as f64);
                                // canvas
                                //     .draw(&andrew::line::Line::new(
                                //         line.start.map(|e| e as usize).into_tuple(),
                                //         line.end.map(|e| e as usize).into_tuple(),
                                //         match stroke.fill {
                                //             Fill::Color(col) => [col.a, col.r, col.g, col.b],
                                //         },
                                //         true,
                                //     ))
                            },
                            Primitive::Text { pos, text, height, col } => {
                                let mut text_state = rcx.text();

                                let font = maybe_font.take().unwrap_or_else(|| {
                                    text_state.font_family("Open Sans").unwrap()
                                });

                                let layout = text_state
                                    .new_text_layout(&text)
                                    .text_color(piet::Color::rgba8(col.r, col.g, col.b, col.a))
                                    .default_attribute(TextAttribute::FontSize(16.0))
                                    .build()
                                    .unwrap();

                                rcx.draw_text(&layout, pos.map(|e| e as f64).into_tuple());

                                maybe_font = Some(font);

                                // canvas
                                //     .draw(&andrew::text::Text::new(
                                //         pos.map(|e| e as usize).into_tuple(),
                                //         [col.a, col.r, col.g, col.b],
                                //         &self.font_data,
                                //         height,
                                //         1.0,
                                //         text,
                                //     ))
                            },
                        }
                    }
                }

                self.win.update_with_buffer(unsafe { std::mem::transmute(surf.get_data().unwrap().deref()) }, w, h).unwrap();

                redraw = false;
            } else {
                self.win.update();
            }
        }
    }
}
