mod parser;

use std::fs;

fn read_file(name: &str) -> String {
    fs::read_to_string(name)
        .expect("Unable to read file.")
}
    
fn main() {
    let test_file = "/home/veera/Projects/orgp/test/example.org";
    let contents = read_file(test_file);
    let parser = parser::Parser::new(&contents);
}
