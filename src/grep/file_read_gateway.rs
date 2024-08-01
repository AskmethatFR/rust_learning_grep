use std::fs::File;
use std::io::{BufRead, BufReader};


pub trait FileReadGateway {
    fn read_file(&self, file_path: &str) -> Vec<String>;
}

pub struct FileReaderGateway {}
impl FileReaderGateway {
    pub(crate) fn new() -> Self {
        FileReaderGateway {}
    }

    pub  fn read_file(&self, file_path: &str) -> Vec<String> {
        let file = File::open(file_path).expect("file not found");
        let reader = BufReader::new(file);
       
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        lines
    }
}

#[cfg(test)]
mod tests {
    use crate::grep;

    #[test]
    fn test_read_file() {
        let file_read_gateway = grep::file_read_gateway::FileReaderGateway::new();


        let file_path = "src/grep/data/hello.txt";
        let result = file_read_gateway.read_file(file_path);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Hello, World!");
        
    }
}