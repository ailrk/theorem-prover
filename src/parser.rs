use std::iter::Peekable;
use std::fmt;

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
    EndOfTokens,
    WrongTerm,
    LexerError,
    Unexpected(String)
}


impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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
                _ if c.is_whitespace() => {},
                '.' | ',' | '(' | ')' => {
                    self.push(idx, idx+1);
                },
                _ if c.is_alphabetic() || ['=', '>'].contains(&c) => {
                    let mut end = idx + 1;
                    while let Some(&(e, c)) = it.peek() {
                        if c.is_alphabetic() || ['=', '>'].contains(&c) {
                            it.next();
                            end = e + 1;
                        }
                        else {
                            break;
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
    })
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenStream<'a> {
    tokens: Vec<&'a str>
}


impl TokenStream<'_> {
    fn new(tokens: Vec<&str>) -> TokenStream {
        TokenStream {
            tokens
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
                return Err(ParserError::Unexpected(token.to_string()))
            }

            it.next();
            Ok(token.to_string())
        } else {
            Err(ParserError::Unexpected(token.to_string()))
        }
    } else {
        Err(ParserError::EndOfTokens)
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
        match it.peek() {
            Some(&token) => return Err(ParserError::Unexpected(token.to_string())),
            None => return Err(ParserError::EndOfTokens)
        };
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
                return Err(ParserError::Unexpected(token.to_string()));
            }
        }
        c += 1;
    }
    Err(ParserError::EndOfTokens)
}


fn parse_keyword(it: &mut TokenIter, keyword: &str) -> Result<(), ParserError> {
    if let Some(&token) = it.peek() {
        if token == keyword {
            it.next();
            Ok(())
        } else {
            Err(ParserError::Unexpected(token.to_string()))
        }
    } else {
        Err(ParserError::EndOfTokens)
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
        Err(ParserError::EndOfTokens)
    }
}


type ToFormula = Box<dyn FnOnce (Formula) -> Formula>;

fn parse_formula_tail(it: &mut TokenIter) -> Result<Option<ToFormula>, ParserError> {
    type Op = fn(Formula, Formula) -> Formula;

    let parse_tail = |it: &mut TokenIter, op: Op| -> Result<Option<ToFormula>, ParserError> {
        it.next();
        let formula2 = parse_formula(it)?;
        Ok(Some(Box::new(move |formula1| {
            op(formula1, formula2)
        })))
    };

    if let Some(&token) = it.peek() {
        match token {
            "and" => parse_tail(it, Formula::and),
            "or" => parse_tail(it, Formula::or),
            "=>" => parse_tail(it, Formula::implies),
            ")" => Ok(None),
            _ => return Err(ParserError::Unexpected(token.to_string()))
        }

    } else {
        Ok(None)
    }
}


fn parse_formula(it: &mut TokenIter) -> Result<Formula, ParserError> {
    if let Some(&"(") = it.peek() {
        let _ = parse_keyword(it, "(")?;
        let simple = parse_formula(it)?;
        let _ = parse_keyword(it, ")")?;
        let tail = parse_formula_tail(it)?;
        match tail {
            Some(f) => Ok(f(simple)),
            None => Ok(simple)
        }
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
