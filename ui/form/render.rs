use std::io::{stdout, Write};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use super::{Form, FieldKind};

impl Form {
    pub fn render(&self) {
        let mut out = stdout();
        // Wipes the screen
        out.execute(Clear(ClearType::All)).unwrap();
        // Forces cursor to top-left corner
        out.execute(MoveTo(0, 0)).unwrap();

        for (i, field) in self.fields.iter().enumerate() {
            let value = match &field.kind {
                FieldKind::Text { value } => value.as_str(),
                FieldKind::Enum { options, selected } => &options[*selected],
                FieldKind::Weights { items, selected } => {
                    &items.iter().enumerate().map(|(i, w)| {
                        if i == *selected {
                            format!("[{}:{}]", w.word, w.value)
                        } else {
                            format!("{}:{}", w.word, w.value)
                        }
                    })
                    .collect::<Vec<_>>().join(" ")
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