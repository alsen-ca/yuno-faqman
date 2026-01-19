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
            let Some(item) = items.get_mut(*selected) else { return None };
            let s = &mut item.value;
            let view = s.as_str();
            match c {
                
                '.' => {
                    // Only allow a single dot (.)
                    if !s.contains('.') { s.push('.')}
                }
                d if d.is_ascii_digit() => {
                    match view {
                        // Only allow 0..5
                        "" => {
                            if d <= '5' { s.push(d); }
                        }

                        // Accept if doesn't already has a dot
                        v if !v.contains('.') => {
                            // ignore
                        }

                        // Replace 0 with new number if has .0
                        v if v.ends_with(".0") => {
                            s.pop();
                            s.push(d);
                        }

                        // Only accept one decimal after dot
                        v if v.ends_with('.') => {
                            s.push(d);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        FieldKind::Enum { .. } => {}

        FieldKind::UuidSelector { title, .. } => {
            title.push(c);
        }
    }

    if let Some(text) = updated_text {
        update_weights(&mut form.fields, "question_weights", &text);
    }

    None
}
