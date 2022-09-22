use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Expr {
    // Data Types
    Number(f64),
    Variable(String),

    // Operators (Math)
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),

    // Operators (Logical)
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThanOrEqual(Box<Expr>, Box<Expr>),
    GreaterThanOrEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>, Box<Expr>),

    // Function Call
    Call(String, Vec<Expr>)
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expression: Expr
}

pub fn parser() -> impl Parser<char, Vec<Assignment>, Error = Simple<char>> {
    let ident = text::ident()
        .padded();

    let expr = recursive(|expr| {
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(|s: String| Expr::Number(s.parse().unwrap()))
            .padded();

        let indirect_value = expr.delimited_by(just('('), just(')'))
            .or(ident.map(Expr::Variable));

        let atom = num
            .or(indirect_value);

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, right| Expr::Negate(Box::new(right)));

        let binary_first = unary.clone()
            .then(op('^').to(Expr::Exponent as fn(_, _) -> _)
                .then(unary)
                .repeated())
            .foldl(|left, (op, right)| op(Box::new(left), Box::new(right)));

        let binary_second = binary_first.clone()
            .then(op('*').to(Expr::Multiply as fn(_, _) -> _)
                .or(op('/').to(Expr::Divide as fn(_, _) -> _))
                .or(op('%').to(Expr::Multiply as fn(_, _) -> _))
                .then(binary_first)
                .repeated())
            .foldl(|left, (op, right)| op(Box::new(left), Box::new(right)));

        let binary_third = binary_second.clone()
            .then(op('+').to(Expr::Add as fn(_, _) -> _)
                .or(op('-').to(Expr::Subtract as fn(_, _) -> _))
                .then(binary_second)
                .repeated())
            .foldl(|left, (op, right)| op(Box::new(left), Box::new(right)));

        binary_third
    });

    let variable = text::ident()
        .chain::<char, _, _>(just('\'').or_not())
        .collect::<String>()
        .padded()
        .then_ignore(just('='))
        .then(expr.clone())
        .then_ignore(just(';'))
        .map(|(name, right)| Assignment {
            name,
            expression: right
        });

    // TODO: fix this erroring on EOL if and ONLY if Tower doesn't
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
