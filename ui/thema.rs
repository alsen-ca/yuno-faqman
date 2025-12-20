use crate::ui::form::{Form, FormResult};
use crate::controller::thema::Thema;

pub fn new_thema_flow() -> Option<Thema> {
    let mut form = Form::new(&[
        "title",
        "command",
    ]);

    match form.run() {
        FormResult::Save => {
            Some(Thema {
                title: form.fields[0].1.clone()
            })
        }
        FormResult::Exit => None,
    }
}
