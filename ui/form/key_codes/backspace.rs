use crate::ui::form::{Form, FieldKind, FormResult};

pub fn handle(form: &mut Form) -> Option<FormResult> {
    if let FieldKind::Text { value } = &mut form.fields[form.cursor].kind {
        value.pop();
    }
    None
}
