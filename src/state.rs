pub enum State<'a, D, S> {
    Bind(Box<dyn for<'b> FnMut(&'b mut D) -> &'b mut S + 'a>),
    Generate(S, Box<dyn FnMut(&mut D) -> S + 'a>),
    Inner(S),
}

impl<'a, D, S> State<'a, D, S> {
    pub fn get_mut<'b, 'c: 'b>(&'c mut self, data: &'b mut D) -> &'b mut S {
        match self {
            State::Bind(f) => f(data),
            State::Generate(s, f) => {
                *s = f(data);
                s
            },
            State::Inner(s) => s,
        }
    }
}
