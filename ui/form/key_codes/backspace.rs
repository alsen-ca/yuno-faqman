use crate::ui::form::{Form, FieldKind};

pub fn handle(form: &mut Form) {
    if let FieldKind::Text { value } = &mut form.fields[form.cursor].kind {
        value.pop();
    }
}
