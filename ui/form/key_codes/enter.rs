use crate::ui::form::{Form, FieldKind, FormResult};

pub fn handle(form: &mut Form) -> Option<FormResult> {
    let field = &form.fields[form.cursor];

    if field.label != "command" {
        return None;
    }

    let FieldKind::Text { value } = &field.kind else {
        return None;
    };

    match value.as_str() {
        "save" => Some(FormResult::Save),
        "exit" => Some(FormResult::Exit),
        _ => None,
    }
}
