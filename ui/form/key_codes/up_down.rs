KeyCode::Up => {
    if self.cursor > 0 {
        self.cursor -= 1;
    }
}
KeyCode::Down => {
    if self.cursor + 1 < self.fields.len() {
        self.cursor += 1;
    }
}
