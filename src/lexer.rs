use std::iter::Peekable;
use std::str::Lines;

pub struct Lexer<'a> {
    lines: Peekable<Lines<'a>>,
    level: Level,
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

#[cfg(test)]
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
        assert!(!"test str".is_ordered_list());
    }
}



#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone)]
enum Level {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    language: ProgLang,
    src: String,
    level: Level,
}

#[derive(Debug, Clone)]
pub struct OrgData {
    level: Level,
    data: String,
}

#[derive(Debug, Clone)]
pub struct OrgList {
    level: Level,
    data: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(OrgData),
    Date(OrgData),
    UnOrderedList(OrgList),
    OrderedList(OrgList),
    CodeBlock(CodeBlock),
    Headline(OrgData),
    Text(OrgData),
}

impl<'a> Lexer<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            lines: contents.lines().peekable(),
            level: Level::One,
        }
    }

    fn lex_title(&mut self) -> OrgEle {
        let title =
            self.lines.next().unwrap().strip_prefix("#+TITLE:").unwrap();
        OrgEle::Title(OrgData {
            data: title.trim().to_string(),
            level: self.level,
        })
    }

    fn lex_date(&mut self) -> OrgEle {
        let date =
            self.lines.next().unwrap().strip_prefix("#+DATE:").unwrap();
        OrgEle::Date(OrgData {
            data: date.trim().to_string(),
            level: self.level,
        })
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
        OrgEle::UnOrderedList(OrgList {
            data: list.clone(),
            level: self.level,
        })
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
        OrgEle::OrderedList(OrgList {
            data: list.clone(),
            level: self.level,
        })
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
            level: self.level,
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
                self.level = Level::Three;
                elements.push(OrgEle::Headline(OrgData {
                    data: self.lex_heading(),
                    level: self.level
                }));
            } else if line.starts_with("** ") {
                self.level = Level::Two;
                elements.push(OrgEle::Headline(OrgData {
                    data: self.lex_heading(),
                    level: self.level
                }));
            } else if line.starts_with("* ") {
                self.level = Level::One;
                elements.push(OrgEle::Headline(OrgData {
                    data: self.lex_heading(),
                    level: self.level
                }));
            } else {
                let text = line.trim();
                if !text.is_empty() {
                    elements.push(OrgEle::Text(OrgData {
                        data: text.to_string(),
                        level: self.level
                    }));
                }
                self.lines.next();
            }
        }
        elements.clone()
    }
}
