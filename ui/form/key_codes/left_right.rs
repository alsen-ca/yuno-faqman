use crossterm::event::KeyCode;
use crate::ui::form::{Form, FieldKind, FormResult};
use crate::ui::form::field::TagSelector;

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

        (KeyCode::Left, FieldKind::MultiUuidSelector { selected, .. }) => {
            if *selected > 0 {
                *selected -= 1;
            }
            None
        }
        
        (KeyCode::Right, FieldKind::MultiUuidSelector { tags, selected }) => {
            if *selected + 1 < tags.len() {
                *selected += 1;
            } else {
                // Check if the current tag's title is not empty
                if let Some(current_tag) = tags.get(*selected) {
                    if !current_tag.tag_title.is_empty() {
                        // Add a new tag
                        tags.push(TagSelector { tag_title: String::new(), uuid: None });
                        *selected += 1;
                    }
                }
            }
            None
        }
        _ => None
    }
}
