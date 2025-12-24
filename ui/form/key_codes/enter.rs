KeyCode::Enter => {
    let field = &self.fields[self.cursor];

    if field.label == "command" {
        if let FieldKind::Text { value } = &field.kind {
            match value.as_str() {
                "save" => {
                    terminal::disable_raw_mode().unwrap();
                    return FormResult::Save;
                }
                "exit" => {
                    terminal::disable_raw_mode().unwrap();
                    return FormResult::Exit;
                }
                _ => {}
            }
        }
    }
}
