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

    pub fn handle_char(&mut self, c: char) -> bool {
        if c == 'q' {
            return true;
        }
        self.strokes += 1;
        return false;
    }
    pub fn handle_backspace(&mut self) -> bool {
        return false;
    }
}
