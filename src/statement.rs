#![allow(non_snake_case)]
use crate::{expression::Expr, generate_enum_and_functions, token::Token};

generate_enum_and_functions! {
    Stmt {
        Block {
            statements: Vec<Stmt>
        },
        Expression {
            expression: Expr,
        },
        If {
            condition: Expr,
            then_branch: Stmt,
            else_branch: Option<Stmt>,
        },
        Print {
            expression: Expr,
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
