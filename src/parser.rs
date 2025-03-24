use crate::language::*;


const KEYWORDS: [&str; 6] = [
    "not",
    "implies",
    "and",
    "or",
    "forall",
    "exists"
];


pub enum ParserError {
    EmptyTokens
}


struct Lexer<'a> {
    tokens: Vec<&'a str>,
    chars: &'a [u8],
    i: usize,
    j: usize
}


impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: vec![],
            chars: input.as_bytes(),
            i: 0,
            j: 0
        }
    }

    fn push(&mut self) {
        self.tokens.push(std::str::from_utf8(&self.chars[self.i..self.j]).unwrap());
        self.i = self.j;
        self.j += 1;
    }

    fn lex(&mut self) -> Vec<&'a str> {
        while self.j <= self.chars.len() {
            // skip white spaces
            if self.chars[self.i] as char == ' ' {
                self.i += 1;
                continue;
            }

            if self.i > self.j {
                self.j = self.i;
            }

            // symbols
            match self.chars[self.i] as char {
                '.' | '(' | ')' => {
                    self.j = self.i + 1;
                    self.push();
                    continue;
                },
                _ => {}
            }

            // identifier
            while self.j < self.chars.len() && self.chars[self.j].is_ascii_alphanumeric() {
                self.j += 1;
                continue;
            }

            if self.j > self.i {
                self.push();
                continue;
            }

            self.j += 1;
        }

        self.tokens.clone()
    }
}



pub fn lex(input: &str) -> Vec<&str> {
    Lexer::new(input).lex()
}


impl Var {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl UTerm {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Func {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Pred {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Not {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}

impl And {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Or {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Implies {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl ForAll {
    fn parse(tokens: &mut Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


impl Exists {
    fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
        todo!()
    }
}


pub fn parse(tokens: &Vec<&str>) -> Result<Term, ParserError> {
    // if tokens.len() == 0 {
    //     return Err(ParserError::EmptyTokens);
    // }

    // match tokens[0] {
    //     "forall" => {},
    //     "exists" => {},
    //     "exists" => {},
    //     _ => {}
    // }

    todo!()
}
