use crate::ui::form::{Form, FormResult};
use crate::controller::qa::Qa;
use crate::ui::form::FormField;

pub fn new_qa_flow() -> Option<Qa> {
    let mut form = Form::new(vec![
        FormField::text("question"),
        FormFIeld::weights("question_weights"),
        FormField::text("answer"),
        FormField::enum_field("lang",
            &["en", "de", "es"],
            0,
        ),
        FormField::text("command"),
    ]);

    match form.run() {
        FormResult::Save => {
            let question = form.get_text("question")?;
            let question_weights = form.get_weights("question_weights");
            let answer = form.get_text("answer")?;
            let lang = form.get_enum("lang")?;

            Some(Qa {
                question,
                answer,
                lang: lang.parse().ok()?
            })
        }
        FormResult::Exit => None,
    }
}
