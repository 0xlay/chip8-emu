pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}
