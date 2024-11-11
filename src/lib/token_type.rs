use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    // Single-character tokens
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Dot,          // .
    Semicolon,    // ;
    Colon,        // :
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %

    // One or two character tokens
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=

    // Literals
    Identifier, // variable names, function names
    String,     // "string literals"
    Number,     // numeric literals

    // Keywords
    And,      // and
    Or,       // or
    Not,      // not
    If,       // if
    Else,     // else
    For,      // for
    While,    // while
    Return,   // return
    Function, // function
    Let,      // let (for variable declarations)
    Const,    // const (for constants)
    Class,    // class (for object-oriented programming)
    This,     // this (for referring to the current instance)
    Super,    // super (for calling methods in a superclass)
    Import,   // import (for modules or libraries)
    Export,   // export (for modules or libraries)
    True,     // true
    False,    // false
    Nil,      // null (null value)-> nil
    Print,    //Print
    Var,      //variable

    // Special types
    Eof, // End of file
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}
