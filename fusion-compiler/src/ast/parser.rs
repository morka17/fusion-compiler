// Import necessary modules and types
use crate::ast::ASTStatement;
use crate::ast::lexer::{Lexer, Token};

use super::{ASTExpression, ASTBinaryOperator, ASTBinaryOperatorKind};
use super::lexer::TokenKind;

// Define the Parser struct to process tokens
pub struct Parser {
    tokens: Vec<super::lexer::Token>,
    current: usize,
}

impl Parser {
    // Create a new Parser instance from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        // Remove whitespace tokens and store non-whitespace tokens in 'tokens' field
        Self {
            tokens: tokens
                .iter()
                .filter(|token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            current: 0,
        }
    }

    // A convenience function to create a new Parser instance from a vector of tokens
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self::new(tokens)
    }

    // Parse the next statement in the token stream
    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;
        // If the current token is EOF, return None to signal the end of parsing
        if token.kind == TokenKind::EOF {
            return None;
        }
        // Otherwise, parse the statement and return the result
        return self.parse_statement();
    }

    // Parse a statement, which is essentially an expression in this simplified example
    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;
        let expr = self.parse_expression()?; // Parse the expression part of the statement
        return Some(ASTStatement::expression(expr));
    }

    // Parse an expression, which may include binary operations
    fn parse_expression(&mut self) -> Option<ASTExpression> {
        self.parse_binary_expression(0) // Start with the lowest precedence (0) for binary operators
    }

    // Parse a binary expression with a given precedence level
    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?; // Parse the left-hand side of the binary expression

        // Keep parsing binary operators and their right-hand operands until the precedence is lower
        while let Some(operator) = self.parse_binary_operator() {
            self.consume(); // Consume the operator token
            let operator_precedence = operator.precedence(); // Get the precedence of the operator
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?; // Parse the right-hand side
            left = ASTExpression::binary(operator, left, right); // Create a binary expression node
        }

        return Some(left);
    }

    // Parse a binary operator
    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current()?;
        let kind = match token.kind {
            // Map token kinds to corresponding binary operator kinds
            TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
            TokenKind::Asterisk => Some(ASTBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
            _ => None,
        };

        // If the token is a valid binary operator, create and return the corresponding operator node
        return kind.map(|kind| ASTBinaryOperator::new(kind, token.clone()));
    }

    // Parse a primary expression, which can be a number or a parenthesized expression
    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?; // Consume the current token
        match token.kind {
            TokenKind::Number(number) => {
                return Some(ASTExpression::number(number)); // Create a number node
            },
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?; // Parse the expression inside the parentheses
                let token = self.consume()?;
                if token.kind != TokenKind::RightParen {
                    panic!("Expected right paren");
                }
                Some(ASTExpression::paranthesized(expr)) // Create a parentheses expression node
            },
            _  => {
                None // Return None for unsupported primary expressions
            }
        }
    }

    // Peek at a token with a given offset from the current position
    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    // Get the current token
    fn current(&self) -> Option<&super::lexer::Token> {
        self.peek(0)
    }

    // Consume the current token and move to the next one
    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        return Some(token);
    }
}
