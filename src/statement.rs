#![allow(non_snake_case)]
use crate::{expression::Expr, functions::Function, generate_enum_and_functions, token::Token};

generate_enum_and_functions! {
    Stmt {
        Block {
            statements: Vec<Stmt>
        },
        Expression {
            expression: Expr,
        },
        Function {
            fun: Function,
        },
        If {
            condition: Expr,
            then_branch: Stmt,
            else_branch: Option<Stmt>,
        },
        Print {
            expression: Expr,
        },
        Return {
            keyword: Token,
            value: Expr,
        },
        Var {
            initializer: Expr,
            name: Token,
        },
        While {
            condition: Expr,
            body: Stmt,
        }
    }
}
