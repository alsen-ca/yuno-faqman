use crossterm::event::KeyCode;
use crate::ui::form::{Form, FieldKind, FormResult};
use crate::ui::form::weights::update_weights;

pub fn handle(form: &mut Form, code: KeyCode) -> Option<FormResult> {
    let KeyCode::Char(c) = code else { return None };

    let cursor = form.cursor;
    let is_question = form.fields[cursor].label == "question";

    let mut updated_text: Option<String> = None;

    match &mut form.fields[cursor].kind {
        FieldKind::Text { value } => {
            value.push(c);

            if is_question {
                updated_text = Some(value.clone());
            }
        }

        FieldKind::Weights { items, selected } => {
            if c.is_ascii_digit() || c == '.' {
                if let Some(item) = items.get_mut(*selected) {
                    item.value.push(c);
                }
            }
        }

        FieldKind::Enum { .. } => {}
    }

    if let Some(text) = updated_text {
        update_weights(&mut form.fields, "question_weights", &text);
    }

    None
}
