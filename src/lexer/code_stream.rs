pub struct CodeStream {
    pub index: usize,
    expr: String,
}

impl CodeStream {
    pub fn new(expr: String) -> Self {
        Self { expr, index: 0 }
    }

    pub fn current(&self) -> char {
        self.expr[self.index..].chars().next().unwrap()
    }

    pub fn accept(&mut self) -> char {
        let ch: char = self.current();
        self.index += 1;

        ch
    }

    pub fn check(&self, char: char) -> bool {
        !self.is_eof() && self.current() == char
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.expr.len()
    }
}
