use crate::ui::form::{Form, FormResult};
use crate::controller::tag::Tag;
use crate::ui::form::FormField;

pub fn new_tag_flow() -> Option<Tag> {
    let mut form = Form::new(vec![
        FormField::text("en_og"),
        FormField::text("de_trans"),
        FormField::text("es_trans"),
        FormField::text("command"),
    ]);

    match form.run() {
        FormResult::Save => {
            let en_og = form.get_text("en_og")?;
            let de_trans = form.get_text("de_trans")?;
            let es_trans = form.get_text("es_trans")?;

            Some(Tag {
                en_og,
                de_trans,
                es_trans
            })
        }
        FormResult::Exit => None,
    }
}
