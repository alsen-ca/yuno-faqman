use crossterm::event::KeyCode;
use crate::ui::form::{Form, FieldKind};
use crate::ui::form::weights::update_weights;

pub fn handle(form: &mut Form, code: KeyCode) {
    let KeyCode::Char(c) = code else { return };

    match &mut form.fields[form.cursor].kind {
        FieldKind::Text { value } => {
            value.push(c);

            if form.fields[form.cursor].label == "question" {
                update_weights(&mut form.fields, "question_weights", value);
            }
        }

        FieldKind::Weights { items, selected } => {
            if c.is_ascii_digit() || c == '.' {
                if let Some(item) = items.get_mut(*selected) {
                    item.value.push(c);
                }
            }
        }

        _ => {}
    }
}
