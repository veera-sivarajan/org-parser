use std::iter::Peekable;
use std::str::Lines;

pub struct Parser<'a> {
    // elements: Vec<OrgEle>,
    lines: Peekable<Lines<'a>>,
}

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(String),
    Date(String),
    UnOrderedList(Vec<String>),
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

    pub fn parse(&mut self) -> Vec<OrgEle> {
        let mut elements = vec![];
        while let Some(line) = self.lines.peek() {
            if line.starts_with("#+TITLE:") {
                elements.push(self.parse_title());
            } else if line.starts_with("#+DATE:") {
                elements.push(self.parse_date());
            } else if line.starts_with("- ") {
                elements.push(self.parse_unordered_list());
            } else {
                self.lines.next();
                continue;
            }
        }
        elements.clone()
    }
}
