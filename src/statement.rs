#![allow(non_snake_case)]
use crate::{expression::Expr, token::Token};

macro_rules! generate_enum_and_functions {
    ($enum_name:ident {
        $( $variant:ident { $( $field:ident : $field_type:ty $(,)? )* } ),* $(,)? }
    ) => {
        #[derive(Debug)]
        pub enum $enum_name {
            $( $variant { $( $field: Box<$field_type> ),* } ),*
        }
            $(
                pub fn $variant($( $field: $field_type ),*) -> $enum_name {
                    $enum_name::$variant {
                    $($field: $field.into()),*
                }
                }
            )*
    };
}

generate_enum_and_functions! {
    Stmt {
        Block {
            statements: Vec<Stmt>
        },
        Expression {
            expression: Expr,
        },
        Print {
            expression: Expr,
        },
        Var {
            name: Token,
            initializer: Expr,
        }
    }
}
