#[derive(Debug)]
pub struct State {
    pub target: String,
    pub input: String,
    pub strokes: u64,
    pub hits: u64,

    pub should_loop: bool,
}

impl State {
    pub fn new(text: String) -> Self {
        Self {
            target: text,
            input: "".to_string(),
            strokes: 0,
            hits: 0,
            should_loop: false,
        }
    }
}
