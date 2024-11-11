use crate::lib::token::Token;
use crate::lib::token_type::TokenType;
use std::collections::HashMap;
use std::iter::Map;

use crate::util::string;
#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub tokens: Vec<Token>,
    pub keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut scanner = Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            keywords: HashMap::new(),
        };
        scanner.init_keywords();
        scanner
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("and".to_string(), TokenType::And);
        self.keywords.insert("class".to_string(), TokenType::Class);
        self.keywords.insert("else".to_string(), TokenType::Else);
        self.keywords.insert("false".to_string(), TokenType::False);
        self.keywords.insert("for".to_string(), TokenType::For);
        self.keywords.insert("fun".to_string(), TokenType::Function);
        self.keywords.insert("if".to_string(), TokenType::If);
        self.keywords.insert("nil".to_string(), TokenType::Nil);
        self.keywords.insert("or".to_string(), TokenType::Or);
        self.keywords.insert("print".to_string(), TokenType::Print);
        self.keywords
            .insert("return".to_string(), TokenType::Return);
        self.keywords.insert("super".to_string(), TokenType::Super);
        self.keywords.insert("this".to_string(), TokenType::This);
        self.keywords.insert("true".to_string(), TokenType::True);
        self.keywords.insert("var".to_string(), TokenType::Var);
        self.keywords.insert("while".to_string(), TokenType::While);
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        // println!("{}", self.source);
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        // println!("{:?}", self.tokens);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen, Some(c.to_string()));
            }
            ')' => {
                self.add_token(TokenType::RightParen, Some(c.to_string()));
            }
            '{' => {
                self.add_token(TokenType::LeftBrace, Some(c.to_string()));
            }
            '}' => {
                self.add_token(TokenType::RightBrace, Some(c.to_string()));
            }
            ',' => {
                self.add_token(TokenType::Comma, Some(c.to_string()));
            }
            '.' => {
                self.add_token(TokenType::Dot, Some(c.to_string()));
            }
            '-' => {
                self.add_token(TokenType::Minus, Some(c.to_string()));
            }
            '+' => {
                self.add_token(TokenType::Plus, Some(c.to_string()));
            }
            ';' => {
                self.add_token(TokenType::Semicolon, Some(c.to_string()));
            }
            '*' => {
                self.add_token(TokenType::Star, Some(c.to_string()));
            }
            '!' => {
                // TODO: Handle error
                let match_c = "=".chars().next().expect("String is empty");
                match self.match_token(match_c) {
                    true => self.add_token(TokenType::BangEqual, None),
                    false => self.add_token(TokenType::Bang, None),
                }
            }
            '/' => {
                // TODO: Handle error

                let match_c = "/".chars().next().expect("String is empty");
                if self.match_token(match_c) {
                    let ch = "=".chars().next().expect("String is empty");
                    while !self.peek().eq(&ch) && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            'o' => {
                if self.match_token('r') {
                    self.add_token(TokenType::Or, None);
                }
            }
            _ => {
                if self.is_digit(&c) {
                    // self.add_token(TokenType::Eof, Some(c.to_string()));
                    self.number();
                } else if self.is_alpha(&c) {
                    self.identifier();
                }else{
                    //error
                }
            }
        }
    }

    fn number(&mut self){
        while self.is_digit(&self.peek()) {
            self.advance();
        }
        if self.peek().eq(&'.') && self.is_digit(&self.peek_next()) {
            self.advance();
            while self.is_digit(&self.peek()) {

                self.advance();
            }
        }
        let value: f64 = *&self.source[self.start..self.current].parse().expect("Failed to parse string as f64");
        self.add_token(TokenType::Number,Some(value.to_string()));
    }
    fn peek_next(&self)->char{
        if self.current+1 <=self.source.len() {
            return '\0';
        }
    /// TODO - error handle
        let ch = self.source.chars().nth(self.current+1).expect("Not found");
        ch
    }
    fn identifier(&mut self) {
        while self.is_alpha_numeric(&self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match self.keywords.get(text) {
            Some(i) => self.add_token(i.clone(), None),
            None => self.add_token(TokenType::Identifier, None),
        };
        // Toke
    }
    fn is_alpha_numeric(&self, c: &char) -> bool {
        self.is_alpha(&c) | self.is_digit(&c)
    }
    fn is_alpha(&self, c: &char) -> bool {
        (c >= &'a' && c <= &'z') | (c >= &'A' && c <= &'Z') | (c == &'_')
    }

    fn is_digit(&self, c: &char) -> bool {
        c >= &'0' && c <= &'9'
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let ch = self.source.chars().nth(self.current).expect("Not found");
        ch
    }

    fn string(&mut self) {
        while !self.peek().eq(&'"') && !self.is_at_end() {
            if self.peek().eq(&'\n') {
                self.line += 1;
            }
            self.advance();
        }
    }
    fn is_at_end(&self) -> bool {
        // println!("current {}", self.current);
        // println!("source {}", self.source);
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        // TODO: Handle error
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .expect("Reached end of source unexpectedly.");
        self.current += 1;
        ch
    }
    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current_char = match self.source.chars().nth(self.current) {
            Some(t) => t,
            None => return false,
        };

        if !current_char.eq(&expected) {
            return false;
        };
        self.current += 1;
        true
    }
}
