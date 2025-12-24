KeyCode::Esc => {
    terminal::disable_raw_mode().unwrap();
    return FormResult::Exit;
}
