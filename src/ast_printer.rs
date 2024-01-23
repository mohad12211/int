use crate::expression::Expr;
use Expr::{Binary, Grouping, Literal, Unary};

pub fn ast_print(expr: &Expr) -> String {
    match expr {
        Unary { operator, right } => paranthesize(&operator.lexeme, &[right]),
        Binary {
            left,
            operator,
            right,
        } => paranthesize(&operator.lexeme, &[left, right]),
        Grouping { expression } => paranthesize("group", &[expression]),
        Literal { value } => value.to_string(),
    }
}

fn paranthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut output = String::new();
    output.push('(');
    output.push_str(name);
    for expr in exprs {
        output.push(' ');
        output.push_str(&ast_print(expr));
    }
    output.push(')');

    output
}
