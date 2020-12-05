use gui::{
    widget::{Toggle, Button, Label, List},
    layout::Direction,
    event::Click,
    Window, Widget,
};

fn main() {
    struct Data {
        counter: i32,
    }

    let ui = List::<Data>::vertical()
        .push(Button::<Data>::default_state()
            .containing(Label::<Data>::gen_state(|d| format!("{}", d.counter))
                .padded(16.0))
            .on(Click, |ctx| ctx.data.counter = 0)
            .padded(8.0))
        .push(List::<Data>::horizontal()
            .push(Button::<Data>::default_state()
                .containing(Label::<Data>::with_state("+").padded(16.0))
                .on(Click, |ctx| ctx.data.counter += 1)
                .padded(8.0))
            .push(Button::<Data>::default_state()
                .containing(Label::<Data>::with_state("-").padded(16.0))
                .on(Click, |ctx| ctx.data.counter -= 1)
                .padded(8.0)))
        .padded(8.0);

    Window::new(ui)
        .run(Data {
            counter: 0,
        })
}

/*
Label::new("Hello, world!")

Label::bind(|state| &mut state.my_text)

TextEntry::bind(|state| &mut state.my_text)

VBox::new()
    .push(Toggle::new(Label::new("Hello!")))
    .push(Label::new("World!"))

VBox::bind(|state| &mut state.entries)

Button::new(Label::new("Click me!"))
    .on::<Click>(|_| println!("Hello, world!"))
*/
