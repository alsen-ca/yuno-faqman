use std::io::{stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommand
};


#[derive(Debug)]
pub enum FieldKind {
    Text {
        value: String,
    },
    Enum {
        options: Vec<String>,
        selected: usize,
    },
}

pub struct FormField {
    pub label: String,
    pub kind: FieldKind,
}

pub struct Form {
    pub fields: Vec<FormField>,
    pub cursor: usize,
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
}


impl Form {
    pub fn new(fields: Vec<FormField>) -> Self {
        Self {
            fields,
            cursor: 0,
        }
    }

    pub fn run(&mut self) -> FormResult {
        terminal::enable_raw_mode().unwrap();

        loop {
            self.render();

            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor + 1 < self.fields.len() {
                            self.cursor += 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        if let FieldKind::Text { value } =
                            &mut self.fields[self.cursor].kind
                        {
                            value.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if let FieldKind::Text { value } =
                            &mut self.fields[self.cursor].kind
                        {
                            value.pop();
                        }
                    }
                    KeyCode::Enter => {
                        let field = &self.fields[self.cursor];

                        if field.label == "command" {
                            if let FieldKind::Text { value } = &field.kind {
                                match value.as_str() {
                                    "save" => {
                                        terminal::disable_raw_mode().unwrap();
                                        return FormResult::Save;
                                    }
                                    "exit" => {
                                        terminal::disable_raw_mode().unwrap();
                                        return FormResult::Exit;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    KeyCode::Left => {
                        if let FieldKind::Enum { selected, .. } = &mut self.fields[self.cursor].kind {
                            if *selected > 0 {
                                *selected -= 1;
                            }
                        }
                    }

                    KeyCode::Right => {
                        if let FieldKind::Enum { options, selected } = &mut self.fields[self.cursor].kind {
                            if *selected + 1 < options.len() {
                                *selected += 1;
                            }
                        }
                    }

                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        return FormResult::Exit;
                    }
                    _ => {}
                }
            }
        }
    }

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

    fn render(&self) {
        let mut out = stdout();
        // Wipes the screen
        out.execute(Clear(ClearType::All)).unwrap();
        // Forces cursor to top-left corner
        out.execute(MoveTo(0, 0)).unwrap();

        for (i, field) in self.fields.iter().enumerate() {
            let value = match &field.kind {
                FieldKind::Text { value } => value.as_str(),
                FieldKind::Enum { options, selected } => &options[*selected],
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

pub enum FormResult {
    Save,
    Exit,
}