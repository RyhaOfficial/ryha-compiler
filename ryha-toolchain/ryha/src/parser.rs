// ryha-toolchain/ryha/src/parser.rs

use crate::ast::{Expression, Program, Statement};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token != Token::Eof {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => {}
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::Ident("".to_string())) {
            return None;
        }

        let name = match &self.cur_token {
            Token::Ident(s) => s.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(name, value))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expr = self.parse_prefix()?;

        while !self.peek_token_is(&Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expr = self.parse_infix(left_expr.clone())?;
        }

        Some(left_expr)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        match &self.cur_token {
            Token::Ident(s) => Some(Expression::Ident(s.clone())),
            Token::Int(i) => Some(Expression::IntLiteral(*i)),
            Token::String(s) => Some(Expression::StringLiteral(s.clone())),
            Token::True => Some(Expression::BoolLiteral(true)),
            Token::False => Some(Expression::BoolLiteral(false)),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix(token, Box::new(right)))
    }

    fn parse_infix(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix(token, Box::new(left), Box::new(right)))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        if !self.expect_peek(Token::LBrace) {
            return None;
        }
        let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token_is(&Token::Else) {
            self.next_token();
            if !self.expect_peek(Token::LBrace) {
                return None;
            }
            alternative = Some(self.parse_block_statement());
        }
        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(&Token::RBrace) && !self.cur_token_is(&Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        statements
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        let params = self.parse_function_parameters()?;
        if !self.expect_peek(Token::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Expression::Function { params, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return Some(identifiers);
        }
        self.next_token();
        if let Token::Ident(ident) = &self.cur_token {
            identifiers.push(ident.clone());
        }
        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            if let Token::Ident(ident) = &self.cur_token {
                identifiers.push(ident.clone());
            }
        }
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_expression_list(Token::RParen)?;
        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_expression_list(&mut self, end: Token) -> Option<Vec<Expression>> {
        let mut list = Vec::new();
        if self.peek_token_is(&end) {
            self.next_token();
            return Some(list);
        }
        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(end) {
            return None;
        }
        Some(list)
    }


    fn cur_token_is(&self, t: &Token) -> bool {
        match (&self.cur_token, t) {
            (Token::Ident(_), Token::Ident(_)) => true,
            _ => &self.cur_token == t,
        }
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        match (&self.peek_token, t) {
            (Token::Ident(_), Token::Ident(_)) => true,
            _ => &self.peek_token == t,
        }
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token
        );
        self.errors.push(msg);
    }

    fn cur_precedence(&self) -> Precedence {
        token_precedence(&self.cur_token)
    }

    fn peek_precedence(&self) -> Precedence {
        token_precedence(&self.peek_token)
    }
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

fn token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Eq | Token::NotEq => Precedence::Equals,
        Token::Lt | Token::Gt => Precedence::LessGreater,
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Slash | Token::Asterisk => Precedence::Product,
        Token::LParen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}
