/// The possible kinds of tokens that the lexer can generate.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Whitespace,
    EOF,
    Bad,
}

/// Represents a span of text in the input string, including its start and end positions.
#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    /// Creates a new TextSpan with the provided start and end positions and the literal text.
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    /// Calculates the length of the text span (number of characters).
    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

/// Represents a token generated by the lexer, containing its kind and the associated text span.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    /// Creates a new Token with the given kind and text span.
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

/// The lexer struct responsible for tokenizing the input string.
pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance with the provided input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    /// Gets the next token from the input string.
    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }

        let c = self.current_char();
        return c.map(|c| {
            let start: usize = self.current_pos;
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                kind = TokenKind::Whitespace;
            } else {
                kind = self.consume_punctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }

    /// Consumes a punctuation character and returns its corresponding token kind.
    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad,
        }
    }

    /// Checks if the provided character is the start of a number.
    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    /// Checks if the provided character is a whitespace character.
    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    /// Returns the current character at the lexer's current position.
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    /// Returns the next character after the lexer's current position.
    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    /// Consumes the current character and moves to the next position.
    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    /// Consumes a sequence of digits and returns the parsed integer value.
    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }

        number
    }
}