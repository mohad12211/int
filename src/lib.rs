use token::Token;

pub mod environment;
pub mod expression;
pub mod functions;
pub mod interpreter;
pub mod native_functions;
pub mod parser;
pub mod raylib;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod value;

macro_rules! generate_enum_and_functions {
    ($enum_name:ident {
        $( $variant:ident { $( $field:ident : $field_type:ty $(,)? )* } ),* $(,)? }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name {
            // TODO: remove this box vvv
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

pub(crate) use generate_enum_and_functions;
use value::Value;

pub enum IntError {
    Error {
        message: String,
        token: Option<Token>,
    },
    ReturnValue(Value, Token),
    Break(Token),
    Continue(Token),
}

trait WithToken<T> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, IntError>;
}

impl<T> WithToken<T> for Result<T, String> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, IntError> {
        self.map_err(|msg| IntError::Error {
            message: msg,
            token: Some(token.as_ref().clone()),
        })
    }
}

impl From<String> for IntError {
    fn from(value: String) -> Self {
        IntError::Error {
            message: value,
            token: None,
        }
    }
}
