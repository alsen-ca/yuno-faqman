use crossterm::event::KeyCode;
use crate::ui::form::{Form, FieldKind, FormResult};

pub fn handle(form: &mut Form, code: KeyCode) -> Option<FormResult> {
    let field = &mut form.fields[form.cursor].kind;

    match (code, field) {
        (KeyCode::Left, FieldKind::Enum { selected, .. }) => {
            if *selected > 0 {
                *selected -= 1;
            }
            None
        }

        (KeyCode::Right, FieldKind::Enum { options, selected }) => {
            if *selected + 1 < options.len() {
                *selected += 1;
            }
            None
        }

        (KeyCode::Left, FieldKind::Weights { selected, .. }) => {
            if *selected > 0 {
                *selected -= 1;
            }
            None
        }

        (KeyCode::Right, FieldKind::Weights { items, selected }) => {
            if *selected + 1 < items.len() {
                *selected += 1;
            }
            None
        }
        _ => None
    }
}
