KeyCode::Backspace => {
    if let FieldKind::Text { value } =
        &mut self.fields[self.cursor].kind
    {
        value.pop();
    }
}
