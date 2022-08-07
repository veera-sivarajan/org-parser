#[derive(Default)]
pub struct Parser {
    file: String,
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
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            ..Default::default()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.file.len()
    }

    fn peek(&self) -> Option<char> {
        if !self.is_at_end() {
            Some(self.file.chars().nth(self.current).unwrap())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.current += 1;
        c
    }

    fn next_eq(&mut self, expected: char) -> bool {
        self.peek().map_or(false, |c| {
            if c == expected {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    fn eat_meta(&mut self) {
        todo!()
    }

    pub fn parse(&mut self) -> Org {
        self.start = self.current;
        if let Some(c) = self.advance() {
            match c {
                '#' => {
                    if self.next_eq('+') {
                        self.eat_meta()
                    } else {
                        todo!()
                    }
                }
                _ => todo!()
            }
        }
        self.output.clone()
    }
}
