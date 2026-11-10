// read_char()
// next_token()
// skip_whitespace()

use crate::token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    //Return the current character, or None at the end.
    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn next_token(&mut self) -> token::TokenKind {
        token::TokenKind::EOF
    }
}
pub fn read_char() {
    println!("Reading File")
}
