use super::field::FormField;

pub struct WordWeight {
    pub word: String,
    pub value: String
}

fn generate_weights(text: &str) -> Vec<WordWeight> {
    text.split_whitespace()
        .map(|w| WordWeight {
            word: w.to_lowercase(),
            value: "1.0".to_string(),
        })
        .collect()
}

fn update_weights(&mut self, label: &str, text: &str) {
    let items = Self::generate_weights(text);

    for field in &mut self.fields {
        if field.label == label {
            if let FieldKind::Weights { items: w_items, selected } = &mut field.kind {
                *w_items = items;
                *selected = 0;
            }
        }
    }
}
