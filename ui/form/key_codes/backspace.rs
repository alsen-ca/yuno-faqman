use crate::ui::form::{Form, FieldKind, FormResult};

pub fn handle(form: &mut Form) -> Option<FormResult> {
    match &mut form.fields[form.cursor].kind {
        FieldKind::Text { value } => {
            value.pop();
        }

        FieldKind::Weights { items, selected } => {
            if let Some(item) = items.get_mut(*selected) {
                item.value.pop();
            }
        }

        FieldKind::UuidSelector { title, .. } => {
            title.pop();
        }


        FieldKind::MultiUuidSelector { tags, selected } => {
            let Some(selected_tag) = tags.get_mut(*selected) else { return None };
            selected_tag.tag_title.pop();
        }

        _ => {}
    }
    None
}
