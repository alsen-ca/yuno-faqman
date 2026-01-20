use std::io::{stdout, Write};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use crate::domain::thema::THEMEN;

use super::{Form, FieldKind};

impl Form {
    pub fn render(&self) {
        let mut out = stdout();
        // Wipes the screen
        out.execute(Clear(ClearType::All)).unwrap();
        // Forces cursor to top-left corner
        out.execute(MoveTo(0, 0)).unwrap();

        for (i, field) in self.fields.iter().enumerate() {
            let value: String = match &field.kind {
                FieldKind::Text { value } => value.clone(),
                FieldKind::Enum { options, selected } => options[*selected].clone(),
                FieldKind::Weights { items, selected } => {
                    items.iter().enumerate().map(|(i, w)| {
                        if i == *selected {
                            format!("[{}:{}]", w.word, w.value)
                        } else {
                            format!("{}:{}", w.word, w.value)
                        }
                    })
                    .collect::<Vec<_>>().join(" ")
                },
                FieldKind::UuidSelector { title, .. } => {
                    let themen = THEMEN.lock().unwrap();
                    if themen.iter().any(|t| t.title == *title) {
                        format!(
                            "{}{}{}",
                            crossterm::style::SetForegroundColor(crossterm::style::Color::Green),
                            title,
                            crossterm::style::ResetColor
                        )
                    } else {
                        format!(
                            "{}{}{}",
                            crossterm::style::SetForegroundColor(crossterm::style::Color::Red),
                            format!("{} (No such Thema)", title),
                            crossterm::style::ResetColor
                        )
                    }
                }
                FieldKind::MultiUuidSelector { tags, selected } => {
                    tags.iter().enumerate().map(|(i, tag)| {
                        if i == *selected {
                            format!("[{}]", tag.tag_title) // Selected tag is wrapped in brackets
                        } else {
                            tag.tag_title.clone() // Unselected tags are plain
                        }
                    }).collect::<Vec<_>>().join(" - ")
                }
            };

            if i == self.cursor {
                writeln!(out, "\r> {}: {}", field.label, value).unwrap();
            } else {
                writeln!(out, "\r  {}: {}", field.label, value).unwrap();
            }
        }

        writeln!(out, "\rArrows up and down to fill the next field | Arrows left and right to change the enums").unwrap();
        writeln!(out, "\rType on field | Enter save / exit on the last 'command' field").unwrap();
        writeln!(out, "\r").unwrap();
        out.flush().unwrap();
    }
}