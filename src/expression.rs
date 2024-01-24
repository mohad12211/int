#![allow(non_snake_case)]
use crate::{token::Token, value::Value};

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
        Grouping {
            expression: Expr,
        },
        Literal {
            value: Value,
        },
    }
}
