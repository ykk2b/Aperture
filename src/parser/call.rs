use super::Parser;
use crate::ast::CallType;
use crate::ast::LiteralType;
use crate::ast::TokenType::*;
use crate::interpreter::expr::Expression;
use crate::errors::ErrorCode::E0x201;

impl Parser {
    pub fn array_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];
        self.consume(RightBracket);
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Array,
        }
    }

    pub fn obj_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.consume(Ident);
        let args = vec![Expression::Value {
            id: self.id(),
            value: LiteralType::String(e.lexeme),
        }];
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Struct,
        }
    }

    pub fn enum_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let e = self.expr();
        let args = vec![e];

        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Enum,
        }
    }

    pub fn func_call(&mut self) -> Expression {
        let name = self.prev(2).clone();
        let mut args = vec![];
        while !self.is_token(RightParen) {
            let arg = self.expr();
            args.push(arg);
            if self.is_token(RightParen) {
                break;
            }
            if !self.if_token_consume(Comma) && !self.is_token(RightParen) {
                self.throw_error(E0x201, vec![self.peek().lexeme.clone()]);
            }
        }
        self.consume(RightParen);
        Expression::Call {
            id: self.id(),
            name: Box::new(Expression::Var {
                id: self.id(),
                name,
            }),
            args,
            call_type: CallType::Func,
        }
    }

    pub fn method(&mut self) -> Expression {
        let mut expr = self.primary();
        if self.if_token_consume(Dot) {
            self.advance();
            if self.peek().token == LeftParen {
                self.retreat();
                expr = self.method_body(expr);
            } else {
                self.retreat();
                expr = self.obj_call()
            }
        }
        expr
    }

    pub fn method_body(&mut self, expr: Expression) -> Expression {
        let name = self.consume(Ident).clone();
        self.consume(LeftParen);
        let mut args = vec![];
        while !self.if_token_consume(RightParen) {
            let e = self.expr();
            args.push(e);
            self.if_token_consume(Comma);
        }
        Expression::Method {
            id: self.id(),
            name,
            left: Box::new(expr),
            args,
        }
    }
}