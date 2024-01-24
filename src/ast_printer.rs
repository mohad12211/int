use crate::expression::Expr;

pub fn ast_print(expr: &Expr) -> String {
    match expr {
        Expr::Unary { operator, right } => paranthesize(&operator.lexeme, &[right]),
        Expr::Binary {
            left,
            operator,
            right,
        } => paranthesize(&operator.lexeme, &[left, right]),
        Expr::Grouping { expression } => paranthesize("group", &[expression]),
        Expr::Literal { value } => value.to_string(),
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
