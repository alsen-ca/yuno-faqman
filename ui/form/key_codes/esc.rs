use crate::ui::form::{Form, FormResult};

pub fn handle(_form: &mut Form) -> Option<FormResult> {
    Some(FormResult::Exit)
}
