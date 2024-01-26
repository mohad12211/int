#![allow(non_snake_case)]
use crate::{generate_enum_and_functions, token::Token, value::Value};

generate_enum_and_functions! {
    Expr {
        Unary {
            operator: Token,
            right: Expr,
        },
        Binary {
            left: Expr,
            operator: Token,
            right: Expr,
        },
        Call {
            callee: Expr,
            paren: Token,
            arguments: Vec<Expr>,
        },
        Grouping {
            expression: Expr,
        },
        Literal {
            value: Value,
        },
        Ternary {
            condition: Expr,
            then_branch: Expr,
            else_branch: Expr,
        },
        Logical {
            left: Expr,
            operator: Token,
            right: Expr,
        },
        Variable {
            name: Token,
        },
        Assign {
            name: Token,
            expression: Expr,
        },
    }
}
