use std::iter::Peekable;
use std::str::Lines;

pub struct Lexer<'a> {
    lines: Peekable<Lines<'a>>,
}

trait OrgLexer {
    fn is_ordered_list(&self) -> bool;
}

impl OrgLexer for &str {
    fn is_ordered_list(&self) -> bool {
        let chars = self.chars();
        let mut preceded_by_digit = false;
        let mut preceded_by_dot = false;
        for c in chars {
            if c.is_ascii_digit() {
                preceded_by_digit = true;
            } else if c == '.' && preceded_by_digit {
                preceded_by_digit = false;
                preceded_by_dot = true;
            } else {
                return c == ' ' && preceded_by_dot;
            }
        }
        false
    }
}

mod test {
    use super::*;

    #[test]
    fn test_is_ordered_list() {
        assert!("1. ".is_ordered_list());
        assert!("10. ".is_ordered_list());
        assert!("100. ".is_ordered_list());
        assert!(!"1.".is_ordered_list());
        assert!(!"10 ".is_ordered_list());
        assert!(!". ".is_ordered_list());
        assert!(!"1 ".is_ordered_list());
    }
}

#[derive(Debug, Clone)]
enum ProgLang {
    Rust,
    Python,
    Cpp,
    C,
    Java,
    Unknown,
}

impl From<&str> for ProgLang {
    fn from(text: &str) -> Self {
        match text.to_lowercase().as_str() {
            "rust" => ProgLang::Rust,
            "python" => ProgLang::Python,
            "cpp" => ProgLang::Cpp,
            "c" => ProgLang::C,
            "java" => ProgLang::Java,
            _ => ProgLang::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    language: ProgLang,
    src: String,
}

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(String),
    Date(String),
    UnOrderedList(Vec<String>),
    OrderedList(Vec<String>),
    CodeBlock(CodeBlock),
    H3(String),
    H2(String),
    H1(String),
    Text(String),
}

impl<'a> Lexer<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            lines: contents.lines().peekable(),
        }
    }

    fn lex_title(&mut self) -> OrgEle {
        let title =
            self.lines.next().unwrap().strip_prefix("#+TITLE:").unwrap();
        OrgEle::Title(title.trim().to_string())
    }

    fn lex_date(&mut self) -> OrgEle {
        let date =
            self.lines.next().unwrap().strip_prefix("#+DATE:").unwrap();
        OrgEle::Date(date.trim().to_string())
    }

    fn lex_unordered_list(&mut self) -> OrgEle {
        let mut list = vec![];
        while let Some(line) = self.lines.peek() {
            if line.starts_with("- ") {
                let sentence = line.strip_prefix("- ").unwrap();
                list.push(sentence.trim().to_string());
                self.lines.next();
            } else {
                break;
            }
        }
        OrgEle::UnOrderedList(list.clone())
    }

    fn lex_ordered_list(&mut self) -> OrgEle {
        let mut list = vec![];
        while let Some(line) = self.lines.peek() {
            if line.is_ordered_list() {
                let index = line.find(' ').unwrap() + 1;
                let text = &line[index..];
                list.push(text.trim().to_string());
                self.lines.next();
            } else {
                break;
            }
        }
        OrgEle::OrderedList(list.clone())
    }

    fn lex_code_block(&mut self) -> OrgEle {
        let mut code = String::new();
        let first_line = self.lines.next().unwrap();
        let index = first_line.find(' ').unwrap() + 1;
        let lang_text = &first_line[index..];
        for line in self.lines.by_ref() {
            if line.starts_with("#+END_SRC") {
                break;
            } else {
                code.push_str(line.trim());
            }
        }
        OrgEle::CodeBlock(CodeBlock {
            language: lang_text.trim().into(),
            src: code,
        })
    }

    fn lex_heading(&mut self) -> String {
        let heading = self.lines.next().unwrap();
        let index = heading.find(' ').unwrap() + 1;
        let title = &heading[index..];
        title.trim().to_string()
    }

    pub fn lex(&mut self) -> Vec<OrgEle> {
        let mut elements = vec![];
        while let Some(line) = self.lines.peek() {
            if line.starts_with("#+TITLE:") {
                elements.push(self.lex_title());
            } else if line.starts_with("#+DATE:") {
                elements.push(self.lex_date());
            } else if line.starts_with("- ") {
                elements.push(self.lex_unordered_list());
            } else if line.is_ordered_list() {
                elements.push(self.lex_ordered_list());
            } else if line.starts_with("#+BEGIN_SRC") {
                elements.push(self.lex_code_block());
            } else if line.starts_with("*** ") {
                elements.push(OrgEle::H3(self.lex_heading()));
            } else if line.starts_with("** ") {
                elements.push(OrgEle::H2(self.lex_heading()));
            } else if line.starts_with("* ") {
                elements.push(OrgEle::H1(self.lex_heading()));
            } else {
                let text = line.trim();
                if !text.is_empty() {
                    elements.push(OrgEle::Text(text.to_string()));
                }
                self.lines.next();
            }
        }
        elements.clone()
    }
}
