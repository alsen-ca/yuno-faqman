KeyCode::Left => {
    if let FieldKind::Enum || let FieldKind::Weights { selected, .. } = &mut self.fields[self.cursor].kind {
        if *selected > 0 {
            *selected -= 1;
        }
    }
}

KeyCode::Right => {
    if let FieldKind::Enum { options, selected } || let FieldKind::Weights { selected, .. } = &mut self.fields[self.cursor].kind {
        if *selected + 1 < options.len() {
            *selected += 1;
        }
    }
}
