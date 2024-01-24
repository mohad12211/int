use token::Token;

pub mod ast_printer;
pub mod expression;
pub mod interpreter;
pub mod parser;
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

pub struct Error {
    message: String,
    token: Option<Token>,
}

trait WithToken<T> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, Error>;
}

impl<T> WithToken<T> for Result<T, String> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, Error> {
        self.map_err(|msg| Error {
            message: msg,
            token: Some(token.as_ref().clone()),
        })
    }
}
