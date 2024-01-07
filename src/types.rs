#[derive(Default)]
pub(crate) enum Align {
    #[default]
    Default,
    Center,
    Left,
    Right,
}

pub(crate) struct Bar {
    pub col: usize,
    pub pos: usize
}
