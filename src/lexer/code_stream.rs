pub struct CodeStream {
    pub index: usize,
    expr: String,
}

impl CodeStream {
    pub fn new(expr: String) -> Self {
        Self { expr, index: 0 }
    }

    pub fn current(&self) -> char {
        let ch = self.expr.chars().nth(self.index);

        match ch {
            None => panic!("panic at CodeStream::current"),
            Some(ch) => ch,
        }
    }

    pub fn accept(&mut self) -> char {
        let ch: char = self.current();
        self.index += 1;

        ch
    }

    pub fn check(&self, char: char) -> bool {
        return self.current() == char;
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.expr.len()
    }
}
