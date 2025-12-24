KeyCode::Char(c) => {
    if let FieldKind::Text { value } = &mut self.fields[self.cursor].kind {
        value.push(c);

        if self.fields[self.cursor].label == "question" {
            self.update_weights("question_weights", value);
        }
    }
    KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
        if let FieldKind::Weights { items, selected } = &mut self.fields[self.cursor].kind {
            items[*selected].value.push(c);
        }
    }
}
