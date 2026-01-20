use super::{Form, FieldKind};
use uuid::Uuid;
use crate::domain::thema::THEMEN;

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

    pub fn get_convert_to_uuid(&self, label: &str) -> Option<Uuid> {
        self.fields.iter().find_map(|field| {
            if field.label != label {
                return None
            }
            if let FieldKind::UuidSelector { title, uuid } = &field.kind {
                // UUID is already resolved
                if let Some(uuid) = uuid {
                    return Some(*uuid);
                }
                // Resolve uuid from title
                let themen = THEMEN.lock().unwrap();
                if let Some(thema) = themen.iter().find(|t| t.title == *title) {
                    return Some(thema.id);
                } else {
                    eprintln!("No such thema: {}", title);
                }
            }
            None
        })
    }

    pub fn get_multi_uuids(&self, label: &str) -> Option<Vec<Uuid>> {
        self.fields.iter().find_map(|f| {
            if f.label != label {
                return None;
            }
            match &f.kind {
                FieldKind::MultiUuidSelector { tags, .. } => {
                    let uuids = tags
                        .iter()
                        .filter_map(|tag| tag.uuid)
                        .collect::<Vec<_>>();
                    Some(uuids)
                }
                _ => None,
            }
        })
    }
}