use super::{Form, FieldKind};

impl Form {
    pub fn get_text(&self, label: &str) -> Option<String> {
        self.fields.iter().find_map(|field| {
            if field.label == label {
                if let FieldKind::Text { value } = &field.kind {
                    return Some(value.clone());
                }
            }
            None
        })
    }

    pub fn get_enum(&self, label: &str) -> Option<String> {
        self.fields.iter().find_map(|field| {
            if field.label == label {
                if let FieldKind::Enum { options, selected } = &field.kind {
                    return Some(options[*selected].clone());
                }
            }
            None
        })
    }

    pub fn get_weights(&self, label: &str) -> Option<Vec<(String, f32)>> {
        self.fields.iter().find_map(|f| {
            if f.label != label {
                return None
            }
            match &f.kind {
                FieldKind::Weights { items, .. } => {
                    let map = items
                        .iter()
                        .filter_map(|ww| {
                            ww.value
                                .parse::<f32>()
                                .ok()
                                .map(|v| (ww.word.clone(), v))
                        })
                        .collect::<Vec<(_, _)>>();

                    Some(map)
                }
                _ => None,
            }
        })
    }
}