use token::Token;

pub mod ast_printer;
pub mod expression;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod value;

pub struct ParsingError {
    message: String,
    token: Token,
}

trait WithToken<T> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, ParsingError>;
}

impl<T> WithToken<T> for Result<T, String> {
    fn with_token(self, token: impl AsRef<Token>) -> Result<T, ParsingError> {
        self.map_err(|msg| ParsingError {
            message: msg,
            token: token.as_ref().clone(),
        })
    }
}
