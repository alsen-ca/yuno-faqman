use crossterm::event::KeyCode;
use crate::ui::form::{Form, FormResult};

pub fn handle(form: &mut Form, code: KeyCode) -> Option<FormResult> {
    match code {
        KeyCode::Up => {
            if form.cursor > 0 {
                form.cursor -= 1;
            }
        }
        KeyCode::Down => {
            if form.cursor + 1 < form.fields.len() {
                form.cursor += 1;
            }
        }
    }
}
