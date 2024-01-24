use token::Token;

pub mod ast_printer;
pub mod expression;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod value;

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
