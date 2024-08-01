use std::io::{BufRead, BufReader};
use crate::grep::file_read_gateway::FileReadGateway;

pub struct InMemoryFileReader {
    text: String,
}

impl InMemoryFileReader {
    pub fn new(text: String) -> Self {
        InMemoryFileReader { text }
    }
}

impl FileReadGateway for InMemoryFileReader {
    fn read_file(&self, file_path: &str) -> Vec<String> {
        let reader = BufReader::new(self.text.as_bytes());
        let mut lines = reader.lines();

        let mut result = Vec::new();

        while let Some(line) = lines.next() {
            result.push(line.unwrap());
        }

        result
    }
}