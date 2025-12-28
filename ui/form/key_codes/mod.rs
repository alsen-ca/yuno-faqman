use crossterm::event::KeyCode;

use crate::ui::form::{Form, FormResult};

mod up_down;
mod left_right;
mod char;
mod enter;
mod backspace;
mod esc;

pub fn handle(form: &mut Form, code: KeyCode) -> Option<FormResult> {
    match code {
        KeyCode::Up | KeyCode::Down => up_down::handle(form, code),
        KeyCode::Left | KeyCode::Right => left_right::handle(form, code),
        KeyCode::Char(_) => char::handle(form, code),
        KeyCode::Enter => enter::handle(form),
        KeyCode::Backspace => backspace::handle(form),
        KeyCode::Esc => return esc::handle(form),
        _ => None
    }
}
