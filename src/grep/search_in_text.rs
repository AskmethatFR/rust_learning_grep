use std::sync::{Arc, Mutex};

use crate::grep::file_read_gateway::FileReadGateway;

pub struct SearchInText<T: FileReadGateway> {
    file_reader_gateway: Arc<Mutex<T>>,
}
impl<T: FileReadGateway> SearchInText<T> {
    pub fn new(file_reader_gateway: Arc<Mutex<T>>) -> Self {
        SearchInText {
            file_reader_gateway
        }
    }

    pub fn search_line_with_pattern(&self, path: String, pattern: String) -> Vec<String> {
        let lines = self.file_reader_gateway.lock().unwrap().read_file(&path);

        lines.iter().filter(|line| line.contains(&pattern)).map(|line| line.clone()).collect()
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, Mutex};

    use crate::grep::in_memory_file_reader::InMemoryFileReader;
    use crate::grep::search_in_text::SearchInText;

    #[test]
    fn search_in_text_should_get_one_line() {
        let search_in_text = given_search_text("bonjour mes amis".to_string());

        let result = search_in_text.search_line_with_pattern("path".to_string(), "mes".to_string());

        let mut expected = Vec::new();
        expected.push("bonjour mes amis".to_string());
        should_be_equal_to_expected(result, expected);
    }

    fn should_be_equal_to_expected(result: Vec<String>, expected: Vec<String>) {
        assert_eq!(result, expected);
    }

    fn given_search_text(string: String) -> SearchInText<InMemoryFileReader> {
        let file_reader = Arc::new(Mutex::new(InMemoryFileReader::new(string)));
        let search_in_text = super::SearchInText::new(Arc::clone(&file_reader));
        search_in_text
    }

    #[test]
    fn search_in_text_should_get_two_lines() {
        let search_in_text = given_search_text("bonjour mes amis\nBonjour mes camarades".to_string());
        let string = "mes".to_string();
        let result = search_in_text.search_line_with_pattern("path".to_string(), string);

        let mut expected = Vec::new();
        expected.push("bonjour mes amis".to_string());
        expected.push("Bonjour mes camarades".to_string());
        
        should_be_equal_to_expected(result, expected);
    }
}