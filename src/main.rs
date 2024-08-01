use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::grep::file_read_gateway::FileReaderGateway;

mod grep;

fn test(str: String) {
    println!("{}", str);
}

fn main() {
    let x = "Hello, world!".to_string();
    test(x);
    
    println!("{}", x);

    // let file_reader_gateway = FileReaderGateway::new();
    // 
    // let file = file_reader_gateway.read_file("src/main.rs");
    // 
    // for line in file {
    //     println!("{}", line);
    // }
    
}
