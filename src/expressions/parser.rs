use chumsky::prelude::*;


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
    Group(Spanned<Box<Expr>>),

    // Expressions
    BinaryExpression(Spanned<Box<Expr>>, BinaryOperation, Spanned<Box<Expr>>),
    UnaryExpression(UnaryOperation, Spanned<Box<Expr>>),

    // Function Call
    Call(String, Vec<Spanned<Box<Expr>>>)
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expression: Spanned<Box<Expr>>,
    pub span: Span
}

pub fn parser() -> impl Parser<char, Vec<Assignment>, Error = Simple<char>> {
    let ident = text::ident()
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

        // delimiters weren't working here for some weird reason
        let group = just('(')
            .padded()
            .ignore_then(expr.clone())
            .padded()
            .then_ignore(just(')'))
            .padded()
            .map(|expr: Spanned<Expr>| Expr::Group((Box::new(expr.0), expr.1)))
            .map_with_span(|expr, span: Span| (expr, span));

        let call = ident
            .then_ignore(just('('))
            .padded()
            .then(expr.clone().separated_by(just(",")))
            .then_ignore(just(')'))
            .map(|(ident, vec)| Expr::Call(ident, vec.iter().map(|spanned| (Box::new(spanned.0.clone()), spanned.1.clone())).collect()))
            .map_with_span(|expr, span: Span| (expr, span));

        let atom = num
            .or(call)
            .or(group)
            .or(variable_reference);

        let op = |c| just(c).padded();
        let dc_op = |c, c2| just(c).then(just(c2)).padded();
 
        let unary = op('-').to(UnaryOperation::Negate)
            .or(op('!').to(UnaryOperation::Not))
            .map_with_span(|expr, span: Span| (expr, span))
            .repeated()
            .then(atom)
            .foldr(|op, right| (Expr::UnaryExpression(op.0, (Box::new(right.0), right.1.clone())), op.1.start..right.1.end));


        let binary_first = unary.clone()
            .then(op('^').to(BinaryOperation::Exponent)
                .then(unary)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Box::new(left.0), left.1.clone()), op, (Box::new(right.0), right.1)), left.1.start..span.end));

        let binary_second = binary_first.clone()
            .then(op('*').to(BinaryOperation::Multiply)
                .or(op('/').to(BinaryOperation::Divide))
                .or(op('%').to(BinaryOperation::Modulo))
                .then(binary_first) 
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Box::new(left.0), left.1.clone()), op, (Box::new(right.0), right.1)), left.1.start..span.end));

        let binary_third = binary_second.clone()
            .then(op('+').to(BinaryOperation::Add)
                .or(op('-').to(BinaryOperation::Subtract))
                .then(binary_second)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Box::new(left.0), left.1.clone()), op, (Box::new(right.0), right.1)), left.1.start..span.end));

        let logical_first = binary_third.clone()
            .then(op('<').to(BinaryOperation::LessThan)
                .or(op('>').to(BinaryOperation::GreaterThan))
                .or(dc_op('<', '=').to(BinaryOperation::LessThanOrEqual))
                .or(dc_op('>', '=').to(BinaryOperation::GreaterThanOrEqual))
                .or(dc_op('=', '=').to(BinaryOperation::Equal))
                .then(binary_third)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Box::new(left.0), left.1.clone()), op, (Box::new(right.0), right.1)), left.1.start..span.end));

        let logical_last = logical_first.clone()
            .then(op('&').to(BinaryOperation::And)
                .or(op('|').to(BinaryOperation::Or))
                .then(logical_first)
                .map_with_span(|expr, span: Span| (expr, span))
                .repeated())
            .foldl(|left, ((op, right), span)| (Expr::BinaryExpression((Box::new(left.0), left.1.clone()), op, (Box::new(right.0), right.1)), left.1.start..span.end));

        logical_last
    });

    let variable = text::ident()
        .chain::<char, _, _>(just('\'').or_not())
        .collect::<String>()
        .padded()
        .then_ignore(just('='))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map_with_span(|expr, span: Span| (expr, span))
        .map(|((name, right), span)| Assignment {
            name,
            expression: (Box::new(right.0), right.1),
            span
        });

    // TODO: fix this erroring on EOL
    let comment = just::<_, _, Simple<char>>('#')
        .then(take_until(just('\n')))
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
