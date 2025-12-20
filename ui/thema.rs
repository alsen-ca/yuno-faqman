use crate::ui::form::{Form, FormResult};
use crate::controller::thema::Thema;
use crate::ui::form::FormField;

pub fn new_thema_flow() -> Option<Thema> {
    let mut form = Form::new(vec![
        FormField::text("title"),
        FormField::text("command"),
    ]);

    match form.run() {
        FormResult::Save => {
            let title = form.get_text("title")?;
            Some(Thema {
                title: title
            })
        }
        FormResult::Exit => None,
    }
}
