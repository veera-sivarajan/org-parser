#[derive(Default)]
pub struct Parser {
    contents: String,
    start: usize,
    current: usize,
    output: Org,
}

#[derive(Default, Clone, Debug)]
pub struct Org {
    title: String,
    date: String,
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

    fn eat_meta(&mut self) {
        let tag = self.eat_until(':');
        match tag {
            "#+TITLE" => {
                self.advance();
                self.skip();
                let data = self.eat_until('\n').trim();
                self.output.title = data.to_owned();
            }
            "#+DATE" => {
                self.advance();
                self.skip();
                let data = self.eat_until('\n').trim();
                self.output.date = data.to_owned();
            }
            _ => todo!()
        }
    }

    pub fn parse(&mut self) -> Option<Org> {
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(next_char) = self.advance() {
                match next_char {
                    '#' => if self.next_eq('+') {
                        self.eat_meta();
                    } else {
                        todo!()
                    }
                    '\n' => continue,
                    _ => break, 
                }
            } else {
                todo!()
            }
        }
        Some(self.output.clone())
    }
}
