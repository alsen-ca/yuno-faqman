use super::weights::WordWeight;

#[derive(Debug)]
pub enum FieldKind {
    Text {
        value: String,
    },
    Enum {
        options: Vec<String>,
        selected: usize,
    },
    Weights {
        items: Vec<WordWeight>,
        selected: usize
    }
}

#[derive(Debug)]
pub struct FormField {
    pub label: String,
    pub kind: FieldKind,
}


impl FormField {
    pub fn text(label: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: FieldKind::Text {
                value: String::new(),
            },
        }
    }

    pub fn enum_field(label: &str, options: &[&str], default: usize) -> Self {
        Self {
            label: label.to_string(),
            kind: FieldKind::Enum {
                options: options.iter().map(|s| s.to_string()).collect(),
                selected: default,
            },
        }
    }

    pub fn weights(label: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: FieldKind::Weights {
                items: Vec::new(),
                selected: 0,
            },
        }
    }
}