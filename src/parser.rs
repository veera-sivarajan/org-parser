#[derive(Default)]
pub struct Parser {
    elements: Vec<OrgEle>,
}

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(String),
    Date(String),
}

impl Parser {
    pub fn new() -> Self {
        Self {
            elements: vec![],
        }
    }

    fn parse_title(&mut self, line: &str) {
        let title = line.strip_prefix("#+TITLE:").unwrap();
        self.elements.push(OrgEle::Title(title.trim().to_string()));
    }

    
    fn parse_date(&mut self, line: &str) {
        let date = line.strip_prefix("#+DATE:").unwrap();
        self.elements.push(OrgEle::Date(date.trim().to_string()));
    }

    pub fn parse(&mut self, contents: &str) -> Vec<OrgEle> { 
        for line in contents.lines() {
            if line.starts_with("#+TITLE:") {
                self.parse_title(line);
            } else if line.starts_with("#+DATE:") {
                self.parse_date(line);
            } else {
                continue
            }
        }
        self.elements.clone()
    }
}

