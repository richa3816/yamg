use std::fmt;

pub enum Mode {
    Normal,
    Insert,
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "normal"),
            Mode::Insert => write!(f, "insert"),
        }
    }
}

pub struct App {
    pub mode: Mode,
    pub input_box: String,
    pub submission: String
}

impl Default for App {
    fn default() -> App {
        App {
            mode: Mode::Normal,
            input_box: String::new(),
            submission: String::new(),
        }
    }
}

impl App {
    pub fn delete_word(&mut self) {
        let new_len = self.input_box.trim().len() - self.input_box.trim().split(' ').last().unwrap().len();
        self.input_box.truncate(new_len);
        self.input_box = String::from(self.input_box.trim());
    }
}
