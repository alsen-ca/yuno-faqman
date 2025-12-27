use super::field::{FormField, FieldKind};

#[derive(Debug)]
pub struct WordWeight {
    pub word: String,
    pub value: String,
}

pub fn generate_weights(text: &str) -> Vec<WordWeight> {
    text.split_whitespace()
        .map(|w| WordWeight {
            word: w.to_lowercase(),
            value: "1.0".to_string(),
        })
        .collect()
}

pub fn update_weights(fields: &mut [FormField], label: &str, text: &str) {
    let items = generate_weights(text);

    for field in fields {
        if field.label == label {
            if let FieldKind::Weights { items: w_items, selected } = &mut field.kind {
                *w_items = items;
                *selected = 0;
            }
            break;
        }
    }
}
