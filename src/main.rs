mod lexer;

use std::fs;

fn read_file(name: &str) -> String {
    fs::read_to_string(name).expect("Unable to read file.")
}

fn main() {
    let test_file = "/home/veera/Projects/orgp/test/example.org";
    let contents = read_file(test_file);
    let mut lexer = lexer::Lexer::new(&contents);
    let org = lexer.lex();
    for ele in org {
        println!("{ele:?}");
    }
}
