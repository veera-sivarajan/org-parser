#[derive(Default)]
pub struct Parser {
    contents: String,
    start: usize,
    current: usize,
}

#[derive(Debug, Clone)]
pub enum OrgEle {
    Title(String),
    Date(String),
}

impl Parser {
    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_owned(),
            ..Default::default()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.contents.len()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.contents.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.contents.chars().nth(self.current)
    }

    fn next_eq(&mut self, expected: char) -> bool {
        self.peek()
            .map_or(false, |c| {
                if c == expected {
                    self.advance();
                    true
                } else {
                    false
                }
            })
    }

    fn eat_until(&mut self, end: char) -> &str {
        while let Some(c) = self.peek() {
            if c != end {
                let _ = self.advance();
            } else {
                break;
            }
        }
        &self.contents[self.start..self.current]
    }

    fn skip(&mut self) {
        self.start = self.current;
    }

    fn eat_meta(&mut self) -> OrgEle {
        let tag = self.eat_until(':');
        match tag {
            "+TITLE" => {
                self.advance();
                self.skip();
                let data = self.eat_until('\n').trim();
                return OrgEle::Title(data.to_owned());
            }
            "+DATE" => {
                self.advance();
                self.skip();
                let data = self.eat_until('\n').trim();
                return OrgEle::Date(data.to_owned());
            }
            _ => {
                println!("Tag: {tag}");
                todo!()
            }
        }
    }

    pub fn parse(&mut self) -> Vec<OrgEle> {
        let mut result: Vec<OrgEle> = vec![];
        while let Some(next_char) = self.advance() { 
            self.start = self.current;
            match next_char {
                '#' => {
                    if self.next_eq('+') {
                        let ele = self.eat_meta();
                        result.push(ele);
                    } else {
                        todo!()
                    }
                }
                '\n' => continue,
                _ => break,
            }
        }
        result
    }
}
