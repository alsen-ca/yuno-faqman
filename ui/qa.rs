use std::str::FromStr;
use crate::ui::form::{Form, FormResult};
use crate::controller::qa::{Qa, Language};

pub fn new_qa_flow() -> Option<Qa> {
    let mut form = Form::new(&[
        "question",
        "answer",
        "lang (en | de | es)",
        "command",
    ]);

    match form.run() {
        FormResult::Save => {
            let lang_input = &form.fields[2].1;
            let lang = match Language::from_str(lang_input) {
                Ok(d) => d,
                Err(_) => {
                    println!("\r\nInvalid language. Use en, de or es");
                    return None;
                }
            };

            Some(Qa {
                question: form.fields[0].1.clone(),
                answer: form.fields[1].1.clone(),
                lang
            })
        }
        FormResult::Exit => None,
    }
}
