#[derive(Default)]
pub struct Parser {
    contents: String,
    start: usize,
    current: usize,
    output: Org,
}

#[derive(Default, Clone)]
struct Org {
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

    pub fn parse(&mut self) -> Org {
        while !self.is_at_end() {
            self.start = self.current;
            if let Some(next_char) = self.advance() {
                match next_char {
                    '#' => if self.next_eq('+') {
                        self.eat_meta()
                    } else {
                        todo!()
                    }
                    _ => todo!()
                }
            } else {
                todo!()
            }
        }
    }
}
