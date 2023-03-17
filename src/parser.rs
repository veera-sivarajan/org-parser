use std::iter::Peekable;
use std::str::Lines;

pub struct Parser<'a> {
    lines: Peekable<Lines<'a>>,
}

trait OrgParser {
    fn is_ordered_list(&self) -> bool;
}

impl OrgParser for &str {
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

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(String),
    Date(String),
    UnOrderedList(Vec<String>),
    OrderedList(Vec<String>),
    CodeBlock(String),
}

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self {
            lines: contents.lines().peekable(),
        }
    }

    fn parse_title(&mut self) -> OrgEle {
        let title =
            self.lines.next().unwrap().strip_prefix("#+TITLE:").unwrap();
        OrgEle::Title(title.trim().to_string())
    }

    fn parse_date(&mut self) -> OrgEle {
        let date =
            self.lines.next().unwrap().strip_prefix("#+DATE:").unwrap();
        OrgEle::Date(date.trim().to_string())
    }

    fn parse_unordered_list(&mut self) -> OrgEle {
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

    fn parse_ordered_list(&mut self) -> OrgEle {
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

    fn parse_code_block(&mut self) -> OrgEle {
        let mut code = String::new();
        self.lines.next();
        for line in self.lines.by_ref() {
            if line.starts_with("#+END_SRC") {
                break;
            } else {
                code.push_str(line.trim());
            }
        }
        OrgEle::CodeBlock(code)
    }

    pub fn parse(&mut self) -> Vec<OrgEle> {
        let mut elements = vec![];
        while let Some(line) = self.lines.peek() {
            if line.starts_with("#+TITLE:") {
                elements.push(self.parse_title());
            } else if line.starts_with("#+DATE:") {
                elements.push(self.parse_date());
            } else if line.starts_with("- ") {
                elements.push(self.parse_unordered_list());
            } else if line.is_ordered_list() {
                elements.push(self.parse_ordered_list());
            } else if line.starts_with("#+BEGIN_SRC") {
                elements.push(self.parse_code_block());
            } else {
                self.lines.next();
                continue;
            }
        }
        elements.clone()
    }
}
