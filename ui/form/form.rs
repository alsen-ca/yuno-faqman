use crossterm::{
    event::{self, Event},
    terminal,
};

use super::{FormField, FormResult};
use super::key_codes;


pub struct Form {
    pub fields: Vec<FormField>,
    pub cursor: usize,
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
                if let Some(result) = key_codes::handle(self, key.code) {
                    terminal::disable_raw_mode().unwrap();
                    return result;
                }
            }
        }
    }
}