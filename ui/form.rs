use std::io::{stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommand
};

pub struct Form {
    pub fields: Vec<(String, String)>,
    pub cursor: usize,
}

impl Form {
    pub fn new(labels: &[&str]) -> Self {
        Self {
            fields: labels
                .iter()
                .map(|l| (l.to_string(), String::new()))
                .collect(),
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
                        self.fields[self.cursor].1.push(c);
                    }
                    KeyCode::Backspace => {
                        self.fields[self.cursor].1.pop();
                    }
                    KeyCode::Enter => {
                        if self.fields[self.cursor].0 == "command" {
                            match self.fields[self.cursor].1.as_str() {
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
                    KeyCode::Esc => {
                        terminal::disable_raw_mode().unwrap();
                        return FormResult::Exit;
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(&self) {
        let mut out = stdout();
        // Wipes the screen
        out.execute(Clear(ClearType::All)).unwrap();
        // Forces cursor to top-left corner
        out.execute(MoveTo(0, 0)).unwrap();

        for (i, (label, value)) in self.fields.iter().enumerate() {
            if i == self.cursor {
                writeln!(out, "\r> {}: {}", label, value).unwrap();
            } else {
                writeln!(out, "\r  {}: {}", label, value).unwrap();
            }
        }

        writeln!(out, "\r↑ ↓ to move | Type on field | Enter save / exit").unwrap();
        writeln!(out, "\r").unwrap();
        out.flush().unwrap();
    }
}

pub enum FormResult {
    Save,
    Exit,
}