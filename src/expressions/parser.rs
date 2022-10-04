use chumsky::prelude::*;
use std::sync::Arc;
use super::errors;
pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,

    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    And,
    Or
}

#[derive(Debug, Clone)]
pub enum UnaryOperation {
    Negate,
    Not
}

#[derive(Debug, Clone)]
pub enum Expr {
    // Data Types
    Number(f64),
    Variable(String),
    Group(Spanned<Arc<Expr>>),

    // Expressions
    BinaryExpression(Spanned<Arc<Expr>>, BinaryOperation, Spanned<Arc<Expr>>),
    UnaryExpression(UnaryOperation, Spanned<Arc<Expr>>),

    // Function Call
    Call(String, Vec<Spanned<Arc<Expr>>>)
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expression: Spanned<Arc<Expr>>,
    pub span: Span
}

pub fn parser() -> impl Parser<char, Vec<Assignment>, Error = Simple<char>> {
    let ident = text::ident()
        .chain::<char, _, _>(just('\'').or_not())
        .collect::<String>()
        .padded();

    let expr = recursive(|expr| {
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(|s: String| Expr::Number(s.parse().unwrap()))
            .padded()
            .map_with_span(|expr, span: Span| (expr, span));

        let variable_reference = ident
            .map(Expr::Variable)
            .map_with_span(|expr, span: Span| (expr, span));

        let group = expr.clone()
            .delimited_by(just('('), just(')'))
            .padded()
            .map(|expr: Spanned<Expr>| Expr::Group((Arc::new(expr.0), expr.1)))
            .map_with_span(|expr, span: Span| (expr, span));

        let call = ident
            .then(expr.clone().separated_by(just(",")).delimited_by(just('('), just(')')))
            .padded()
            .map(|(ident, vec)| Expr::Call(ident, vec.iter().map(|spanned| (Arc::new(spanned.0.clone()), spanned.1.clone())).collect()))
            .map_with_span(|expr, span: Span| (expr, span));

        // TODO: this sucks, find a better way to do this that doesn't suck
        let op = |c| just(c).padded();
        let dc_op = |c, c2| just(c).then(just(c2)).padded();

        let short_mul_rhs = call.clone()
            .or(group.clone())
            .or(variable_reference.clone());

        let short_mul_neg = op('-')
            .map_with_span(|expr, span: Span| (expr, span))
            .repeated()
            .then(short_mul_rhs.clone().or(num.clone()))
            .foldr(|op, right| (Expr::UnaryExpression(UnaryOperation::Negate, (Arc::new(right.0), right.1.clone())), op.1.start..right.1.end))
            .labelled("expression");

        let short_mul_exp = short_mul_rhs.clone()
            .then(op('^')
                .to(BinaryOperation::Exponent)
                .then(short_mul_neg)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((_op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), BinaryOperation::Exponent, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        let short_mul = num.clone()
            .then(short_mul_exp)
            .map(|(lhs, rhs)| (Expr::BinaryExpression((Arc::new(lhs.0), lhs.1.clone()), BinaryOperation::Multiply, (Arc::new(rhs.0), rhs.1.clone())), lhs.1.start..rhs.1.end));

        let atom = short_mul
            .or(num)
            .or(call)
            .or(group)
            .or(variable_reference);

        let unary = op('-').to(UnaryOperation::Negate)
            .or(op('!').to(UnaryOperation::Not))
            .map_with_span(|expr, span: Span| (expr, span))
            .repeated()
            .then(atom)
            .foldr(|op, right| (Expr::UnaryExpression(op.0, (Arc::new(right.0), right.1.clone())), op.1.start..right.1.end))
            .labelled("expression");

        let binary_first = unary.clone()
            .then(op('^').to(BinaryOperation::Exponent)
                .then(unary)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), op, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        let binary_second = binary_first.clone()
            .then(op('*').to(BinaryOperation::Multiply)
                .or(op('/').to(BinaryOperation::Divide))
                .or(op('%').to(BinaryOperation::Modulo))
                .then(binary_first) 
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), op, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        let binary_third = binary_second.clone()
            .then(op('+').to(BinaryOperation::Add)
                .or(op('-').to(BinaryOperation::Subtract))
                .then(binary_second)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), op, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        let logical_first = binary_third.clone()
            .then(dc_op('<', '=').to(BinaryOperation::LessThanOrEqual)
                .or(dc_op('>', '=').to(BinaryOperation::GreaterThanOrEqual))
                .or(dc_op('=', '=').to(BinaryOperation::Equal))
                .or(op('<').to(BinaryOperation::LessThan))
                .or(op('>').to(BinaryOperation::GreaterThan))
                .then(binary_third)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), op, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        let logical_last = logical_first.clone()
            .then(op('&').to(BinaryOperation::And)
                .or(op('|').to(BinaryOperation::Or))
                .then(logical_first)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Arc::new(left.0), left.1.clone()), op, (Arc::new(right.0), right.1)), left.1.start..span.end))
            .labelled("expression");

        logical_last
    });

    let variable = ident
        .padded()
        .then_ignore(just('='))
        .then(expr.clone())
        .padded()
        .then_ignore(just(';'))
        .map_with_span(|expr, span: Span| (expr, span))
        .map(|((name, right), span)| Assignment {
            name,
            expression: (Arc::new(right.0), right.1),
            span
        })
        .labelled("variable assignment");

    // TODO: fix this erroring on EOL
    let comment = just::<_, _, Simple<char>>('#')
        .then(take_until(just('\n').labelled("new line")))
        .padded()
        .ignored();

    variable
        // .or(output)
        .recover_with(skip_then_retry_until([]))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .then_ignore(end())
}

pub fn process_parser_error(error: Simple<char>, text: String) -> errors::Error {
    let loc = errors::get_position_from_span(error.span(), text.clone());

    let mut processed_error = errors::Error {
        line_number: loc.0,
        col_number: loc.1,
        error_type: errors::ErrorType::ParseError,
        reason: "P255: Unexpected parser error.".to_string(),
        id: 255
    };

    let found_character = error.found();

    let mut wants_semi = false;
    let mut wants_close_parens = false;
    let mut wants_new_line = false;
    let mut wants_num = false;

    let got_period = found_character == Some(&'.');
    let got_none = found_character == None;

    error
        .expected()
        .for_each(|want| {
            match want {
                Some(';') => wants_semi = true,
                Some(')') => wants_close_parens = true,
                Some('\n') => wants_new_line = true,
                Some('0') => wants_num = true,
                None => panic!("parser gave an error for no reason"),
                _ => ()
            }
        });

    if got_none && wants_new_line {
        processed_error.reason = "P1: Expressions cannot end in a comment.".into();
        processed_error.id = 1;
    } else if got_none {
        processed_error.reason = "P2: Unexpected end of file.".into();
        processed_error.id = 2;
    } else if got_period && wants_num {
        processed_error.reason = "P3: Laser Studio doesn't support shorthand float literals (eg. .1). Please use full literals instead (eg. 0.1).".into();
        processed_error.id = 3;
    } else if wants_semi {
        processed_error.reason = format!("P4: Unexpected character '{}'. Perhaps you forgot a semi-colon?", found_character.unwrap_or(&'_'));
        processed_error.id = 4;
    } else if wants_close_parens {
        processed_error.reason = format!("P5: Unexpected character '{}'. Perhaps you forgot to close your parenthesis?", found_character.unwrap_or(&'_'));
        processed_error.id = 5;
    } else {
        processed_error.reason = format!("P0: Unexpected character '{}'.", found_character.unwrap_or(&'_'));
        processed_error.id = 0;
    }

    processed_error
}
