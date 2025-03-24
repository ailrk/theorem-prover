use std::iter::Peekable;

use crate::language::*;


const KEYWORDS: [&str; 6] = [
    "not",
    "=>",
    "and",
    "or",
    "forall",
    "exists"
];


#[derive(Debug)]
pub enum ParserError {
    EmptyTokens,
    WrongTerm,
    LexerError,
    Unexpected
}


struct Lexer<'a> {
    tokens: Vec<&'a str>,
    input: &'a str,
}


impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: vec![],
            input,
        }
    }

    fn push(&mut self, from: usize, to: usize) {
        self.tokens.push(&self.input[from..to]);
    }

    fn lex(&mut self) -> Result<Vec<&'a str>, String> {
        let mut it = self.input.chars().enumerate().peekable();
        while let Some((idx, c)) = it.next() {
            match c {
                ' ' => {},
                '.' | ',' | '(' | ')' => {
                    self.push(idx, idx+1);
                },
                _ if c.is_alphabetic() || ['=', '>'].contains(&c) => {
                    let mut end = idx + 1;
                    while let Some(&(e, c)) = it.peek() {
                        end = e;
                        if !(c.is_alphabetic() || ['=', '>'].contains(&c)) {
                            break;
                        } else {
                            it.next();
                        }
                    }
                    self.push(idx, end);
                }
                _ => {
                    return Err(format!("unexpected character {}", c));
                }
            }
        }
        Ok(self.tokens.clone())
    }
}


pub fn lex(input: &str) -> Result<TokenStream, String> {
    Ok(TokenStream {
       tokens: Lexer::new(input).lex()?,
        i: 0
    })
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenStream<'a> {
    tokens: Vec<&'a str>,
    i: usize,
}


impl TokenStream<'_> {
    fn new(tokens: Vec<&str>) -> TokenStream {
        TokenStream {
            tokens,
            i: 0,
        }
    }

    fn iter(&mut self) -> TokenStreamIterator<'_> {
        TokenStreamIterator {
            stream: self,
            i: 0
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenStreamIterator<'a> {
    stream: &'a TokenStream<'a>,
    i: usize,
}


impl<'a> Iterator for TokenStreamIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.stream.tokens.len() {
            None
        } else {
            let i = self.i;
            self.i += 1;
            Some(self.stream.tokens[i])
        }
    }
}


type TokenIter<'a> = Peekable<TokenStreamIterator<'a>>;



fn parse_identifier(it: &mut TokenIter) -> Result<String, ParserError> {
    if let Some(&token) = it.peek() {
        if token.chars().all(|c| c.is_ascii_alphabetic()) {
            if KEYWORDS.contains(&token) {
                return Err(ParserError::Unexpected)
            }

            it.next();
            Ok(token.to_string())
        } else {
            Err(ParserError::Unexpected)
        }
    } else {
        Err(ParserError::EmptyTokens)
    }
}


fn parse_var(it: &mut TokenIter) -> Result<Var, ParserError> {
    let id = parse_identifier(it)?;
    Ok(Var { name: id, time: 0 })
}


fn parse_func(it: &mut TokenIter) -> Result<Func, ParserError> {
    let id = parse_identifier(it)?;
    let terms = parse_list(it)?;
    Ok(Func { name: id, terms })
}



fn parse_term(it: &mut TokenIter) -> Result<Term, ParserError> {
    let it1 = it.clone();
    match parse_func(it) {
        Ok(s) => Ok(Term::Func(Box::new(s))),
        Err(_) => {
            *it = it1;
            parse_var(it).map(|var| Term::Var(Box::new(var)))
        }
    }
}


fn parse_list(it: &mut TokenIter) -> Result<Vec<Term>, ParserError> {
    if let Some(&"(") = it.peek() {
        it.next();
    } else {
        return Err(ParserError::Unexpected);
    }

    let mut c = 0;
    let mut terms: Vec<Term> = vec![];

    while let Some(&token) = it.peek() {
        if token == ")" {
            it.next();
            return Ok(terms);
        }

        if c % 2 == 0 { // term
            let term = parse_term(it)?;
            terms.push(term);
        } else { // ,
            if token == "," {
                it.next();
            } else {
                return Err(ParserError::Unexpected);
            }
        }
        c += 1;
    }
    Err(ParserError::Unexpected)
}


fn parse_keyword(it: &mut TokenIter, keyword: &str) -> Result<(), ParserError> {
    if let Some(&token) = it.peek() {
        if token == keyword {
            it.next();
            Ok(())
        } else {
            Err(ParserError::Unexpected)
        }
    } else {
        Err(ParserError::Unexpected)
    }
}


fn parse_forall(it: &mut TokenIter) -> Result<Formula, ParserError> {
    let _ = parse_keyword(it, "forall");

    let var = parse_term(it)?;
    match var {
        Term::Var(_) => {},
        _ => return Err(ParserError::WrongTerm)
    }
    let _ = parse_keyword(it, ".");
    let formula = parse_formula(it)?;
    Ok(Formula::forall(var, formula))
}


fn parse_exists(it: &mut TokenIter) -> Result<Formula, ParserError> {
    let _ = parse_keyword(it, "exists");

    let var = parse_term(it)?;
    match var {
        Term::Var(_) => {},
        _ => return Err(ParserError::WrongTerm)
    }
    let _ = parse_keyword(it, ".");
    let formula = parse_formula(it)?;
    Ok(Formula::exists(var, formula))
}


fn parse_not(it: &mut TokenIter) -> Result<Formula, ParserError> {
    let _ = parse_keyword(it, "not");
    let formula = parse_formula(it)?;
    Ok(Formula::not(formula))
}


fn parse_pred(it: &mut TokenIter) -> Result<Formula, ParserError> {
    let id = parse_identifier(it)?;
    let terms = parse_list(it)?;
    Ok(Formula::pred(&id, terms))
}


fn parse_simple_formula(it: &mut TokenIter) -> Result<Formula, ParserError> {
    if let Some(&token) = it.peek() {
        match token {
            "forall" => parse_forall(it),
            "exists" => parse_exists(it),
            "not" => parse_not(it),
            _ => parse_pred(it)
        }
    } else {
        Err(ParserError::EmptyTokens)
    }
}


type ToFormula = Box<dyn FnOnce (Formula) -> Formula>;

fn parse_formula_tail(it: &mut TokenIter) -> Result<Option<ToFormula>, ParserError> {
    type Op = fn(Formula, Formula) -> Formula;

    let parse_tail = |it: &mut TokenIter, op: Op| -> Result<Option<ToFormula>, ParserError> {
        it.next();
        let formula1 = parse_formula(it)?;
        Ok(Some(Box::new(move |formula2| {
            op(formula1, formula2)
        })))
    };

    if let Some(&token) = it.peek() {
        match token {
            "and" => parse_tail(it, Formula::and),
            "or" => parse_tail(it, Formula::or),
            "=>" => parse_tail(it, Formula::implies),
            _ => Ok(None)
        }

    } else {
        Ok(None)
    }
}


fn parse_formula(it: &mut TokenIter) -> Result<Formula, ParserError> {
    if let Some(&"(") = it.peek() {
        let _ = parse_keyword(it, "(")?;
        parse_formula(it)
    } else {
        let simple = parse_simple_formula(it)?;
        let tail = parse_formula_tail(it)?;
        match tail {
            Some(f) => Ok(f(simple)),
            None => Ok(simple)
        }
    }
}


pub fn parse(input: &str) -> Result<Formula, ParserError> {
    let mut s = match lex(input) {
        Ok(s) => s,
        Err(_) => return Err(ParserError::LexerError)
    };

    parse_formula(&mut s.iter().peekable())
}
