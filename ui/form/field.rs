use super::weights::WordWeight;
use uuid::Uuid;

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
    },
    UuidSelector {
        title: String, // Entered by form
        uuid: Option<Uuid> // Resolve uuid based on title, if found
    },
    MultiUuidSelector {
        tags: Vec<TagSelector>,
        selected: usize
    },
}

#[derive(Debug)]
pub struct TagSelector {
    pub tag_title: String,
    pub uuid: Option<Uuid>,
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

    pub fn convert_uuid(label: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: FieldKind::UuidSelector {
                title: String::new(),
                uuid: None
            }
        }
    }

    pub fn multi_uuid_selector(label: &str) -> Self {
        Self {
            label: label.to_string(),
            kind: FieldKind::MultiUuidSelector {
                tags: vec![TagSelector { tag_title: String::new(), uuid: None }],
                selected: 0
            }
        }
    }
}