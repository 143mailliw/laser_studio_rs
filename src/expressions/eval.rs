use super::parser::Expr;
use super::parser::BinaryOperation;
use super::parser::UnaryOperation;
use super::parser::Spanned;
use super::parser::Span;
use super::parser::Assignment;
use super::errors::Error;
use super::errors::ErrorType;
use std::collections::HashMap;
use rand::Rng;

pub struct RawEvalError {
    error: String,
    span: Span
}

fn eval(spanned_expr: &Spanned<Box<Expr>>, variables: &mut HashMap<String, f64>) -> Result<f64, RawEvalError> {
    let expr = spanned_expr.0.clone();
    let span = spanned_expr.1.clone();

    match *expr {
        Expr::Number(x) => Ok(x),
        Expr::Group(x) => Ok(eval(&x, variables)?),
        Expr::UnaryExpression(op, a) => {
            match op {
                UnaryOperation::Negate => Ok(-eval(&a, variables)?),
                UnaryOperation::Not => Ok(if eval(&a, variables)? >= 1.0 {0.0} else {1.0})
            }
        },
        Expr::BinaryExpression(a, op, b) => {
            match op {
                // Mathematical Operations
                BinaryOperation::Add => Ok(eval(&a, variables)? + eval(&b, variables)?),
                BinaryOperation::Subtract => Ok(eval(&a, variables)? - eval(&b, variables)?),
                BinaryOperation::Multiply => Ok(eval(&a, variables)? * eval(&b, variables)?),
                BinaryOperation::Divide => Ok(eval(&a, variables)? / eval(&b, variables)?),
                BinaryOperation::Modulo => Ok(eval(&a, variables)? / eval(&b, variables)?),
                BinaryOperation::Exponent => Ok(f64::powf(eval(&a, variables)?, eval(&b, variables)?)),

                // Logical Operations
                BinaryOperation::LessThan => Ok((eval(&a, variables)? < eval(&b, variables)?) as u64 as f64),
                BinaryOperation::GreaterThan => Ok((eval(&a, variables)? > eval(&b, variables)?) as u64 as f64),
                BinaryOperation::LessThanOrEqual => Ok((eval(&a, variables)? <= eval(&b, variables)?) as u64 as f64),
                BinaryOperation::GreaterThanOrEqual => Ok((eval(&a, variables)? >= eval(&b, variables)?) as u64 as f64),
                BinaryOperation::Equal => Ok((eval(&a, variables)? == eval(&b, variables)?) as u64 as f64),
                BinaryOperation::And => Ok((eval(&a, variables)? >= 1.0 && eval(&a, variables)? >= 1.0) as u64 as f64),
                BinaryOperation::Or => Ok((eval(&a, variables)? >= 1.0 || eval(&a, variables)? >= 1.0) as u64 as f64),
            }
        },
        Expr::Variable(name) => {
            let result = variables.get(&name);

            match result {
                Some(value) => Ok(*value),
                None => Err(RawEvalError { 
                    error: format!("Cannot find variable '{name}'. Are you using it too early?"),
                    span
                })

            }
        },
        Expr::Call(func, args) => {
            let arguments_results = args
                .iter()
                .map(|a| eval(&a, variables))
                .collect::<Result<Vec<f64>, RawEvalError>>();

            let arguments = match arguments_results {
                Ok(args) => args,
                Err(error) => return Err(error)
            };

            match func.as_str() {
                "sin" => function("sin", 1, |args| f64::sin(args[0]), span, arguments),
                "cos" => function("cos", 1, |args| f64::cos(args[0]), span, arguments),
                "tan" => function("tan", 1, |args| f64::tan(args[0]), span, arguments),
                "asin" => function("asin", 1, |args| f64::asin(args[0]), span, arguments),
                "acos" => function("acos", 1, |args| f64::acos(args[0]), span, arguments),
                "atan" => function("atan", 1, |args| f64::atan(args[0]), span, arguments),
                "atan2" => function("atan2", 2, |args| f64::atan2(args[0], args[1]), span, arguments),
                "sqrt" => function("sqrt", 1, |args| f64::sqrt(args[0]), span, arguments),
                "min" => function("min", 2, |args| f64::min(args[0], args[1]), span, arguments),
                "max" => function("max", 2, |args| f64::max(args[0], args[1]), span, arguments),
                "floor" => function("floor", 1, |args| f64::floor(args[0]), span, arguments),
                "ceil" => function("ceil", 1, |args| f64::ceil(args[0]), span, arguments),
                "round" => function("round", 1, |args| f64::round(args[0]), span, arguments),
                "abs" => function("abs", 1, |args| f64::abs(args[0]), span, arguments),
                "rand" => function("rand", 0, |_args| {
                    let mut rng = rand::thread_rng();
                    
                    rng.gen::<f64>()
                }, span, arguments),
                "if" => function("if", 3, |args| if args[0] >= 1.0 {args[1]} else {args[2]}, span, arguments),
                // formula for this is from the steam guide
                "lerp" => function("lerp", 3, |args| (args[1] * args[0] + args[2] * (1.0 - args[0])), span, arguments),
                _ => return Err(RawEvalError {
                    error: format!("No such function '{func}'."), 
                    span
                })
            }
        }
    }
}

fn function(name: &str, exp_count: u8, c: impl Fn(Vec<f64>) -> f64, span: Span, arguments: Vec<f64>) -> Result<f64, RawEvalError> {
    if arguments.len() == exp_count.into() {
        Ok(c(arguments))
    } else {
        let actual_count = arguments.len();
        let exp_count_text = if exp_count == 1 {"argument"} else {"arguments"};
        let actual_count_text = if actual_count == 1 {"argument"} else {"arguments"};

        Err(RawEvalError {
            error: format!("Function '{name}' expected {exp_count} {exp_count_text}, but only got {actual_count} {actual_count_text}."),
            span
        })
    }
}

pub fn run(assignments: Vec<Assignment>) -> (HashMap<String, f64>, Vec<Error>) {
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut errors: Vec<Error> = vec!();

    for assignment in assignments {
        let eval_result = eval(&assignment.expression, &mut variables);
        
        match eval_result {
            Ok(value) => { variables.insert(assignment.name, value); },
            Err(error) => { errors.push(Error { line_number: 0, col_number: 0, reason: error.error, error_type: ErrorType::EvaluationError }); }
        }
    };
    
    (variables, errors)
}
