pub struct Parser {
    file: String,
    current: usize,
    next: usize,
}

impl Parser {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            current: 0,
            next: 0,
        }
    }

    // pub fn parse() -> Json 
}
