use crate::ui::form::{Form, FormResult};
use crate::controller::tag::Tag;

pub fn new_tag_flow() -> Option<Tag> {
    let mut form = Form::new(&[
        "en_og",
        "de_trans",
        "es_trans",
        "command",
    ]);

    match form.run() {
        FormResult::Save => {
            Some(Tag {
                en_og: form.fields[0].1.clone(),
                de_trans: form.fields[1].1.clone(),
                es_trans: form.fields[2].1.clone(),
            })
        }
        FormResult::Exit => None,
    }
}
