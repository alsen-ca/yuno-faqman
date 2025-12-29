use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

const MAX_HISTORY: usize = 50;

pub struct History {
    entries: Vec<String>,
    cursor: usize,
    path: PathBuf,
}

impl History {
    pub fn load(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        let mut entries = Vec::new();

        if path.exists() {
            let file = fs::File::open(&path)?;
            let reader = BufReader::new(file);

            for line in reader.lines().flatten() {
                if !line.trim().is_empty() {
                    entries.push(line);
                }
            }
        }

        // keep last MAX_HISTORY
        if entries.len() > MAX_HISTORY {
            entries.drain(0..entries.len() - MAX_HISTORY);
        }

        let cursor = entries.len();

        Ok(Self {
            entries,
            cursor,
            path,
        })
    }

    pub fn push(&mut self, command: &str) -> io::Result<()> {
        let cmd = command.trim();
        if cmd.is_empty() {
            return Ok(());
        }

        // Avoid duplicate consecutive entries
        if self.entries.last().map(|s| s == cmd).unwrap_or(false) {
            self.cursor = self.entries.len();
            return Ok(());
        }

        self.entries.push(cmd.to_string());

        if self.entries.len() > MAX_HISTORY {
            self.entries.remove(0);
        }

        self.cursor = self.entries.len();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", cmd)?;

        Ok(())
    }

    pub fn previous(&mut self) -> Option<&str> {
        if self.cursor == 0 {
            return None;
        }
        self.cursor -= 1;
        self.entries.get(self.cursor).map(String::as_str)
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.cursor >= self.entries.len() {
            return None;
        }
        self.cursor += 1;

        if self.cursor == self.entries.len() {
            return Some("");
        }

        self.entries.get(self.cursor).map(String::as_str)
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = self.entries.len();
    }
}
