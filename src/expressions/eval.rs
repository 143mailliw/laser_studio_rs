use super::parser::Expr;
use super::parser::BinaryOperation;
use super::parser::UnaryOperation;
use super::parser::Spanned;
use super::parser::Span;
use super::parser::Assignment;
use super::errors::*;
use ahash::AHashMap;
use rand::Rng;

pub struct RawEvalError {
    error: String,
    span: Span
}

#[derive(Clone, Copy)]
pub struct EvalContext {
    pub x: f64,
    pub y: f64,
    pub index: f64,
    pub count: f64,
    pub fraction: f64,
    pub pi: f64,
    pub tau: f64,
    pub time: f64,
    pub projection_time: f64,
    pub projection_start_time: f64
}

fn eval(spanned_expr: &Spanned<Box<Expr>>, variables: &mut AHashMap<String, f64>, ctx: EvalContext) -> Result<f64, RawEvalError> {
    let expr = (&*spanned_expr.0).clone();
    let span = &spanned_expr.1;

    match expr {
        Expr::Call(func, args) => {
            match func.as_str() {                
                "if" => function_complex("if", 3, |args| if eval(&args[0], variables, ctx)? >= 1.0 {eval(&args[1], variables, ctx)} else {eval(&args[2], variables, ctx)}, span, args),
                _ => {
                    let arguments_results = args
                        .iter()
                        .map(|a| eval(&a, variables, ctx))
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
                        // "if" => function("if", 3, |args| if args[0] >= 1.0 {args[1]} else {args[2]}, span, arguments),
                        // formula for this is from the steam guide
                        "lerp" => function("lerp", 3, |args| (args[1] * args[0] + args[2] * (1.0 - args[0])), span, arguments),
                        _ => return Err(RawEvalError {
                            error: format!("No such function '{func}'."), 
                            span: span.clone()
                        })
                    }
                } 
            }
        },
        Expr::Number(x) => Ok(x),
        Expr::Group(x) => Ok(eval(&x, variables, ctx)?),
        Expr::UnaryExpression(op, a) => {
            match op {
                UnaryOperation::Negate => Ok(-eval(&a, variables, ctx)?),
                UnaryOperation::Not => Ok(if eval(&a, variables, ctx)? >= 1.0 {0.0} else {1.0})
            }
        },
        Expr::BinaryExpression(a, op, b) => {
            match op {
                // Mathematical Operations
                BinaryOperation::Add => Ok(eval(&a, variables, ctx)? + eval(&b, variables, ctx)?),
                BinaryOperation::Subtract => Ok(eval(&a, variables, ctx)? - eval(&b, variables, ctx)?),
                BinaryOperation::Multiply => Ok(eval(&a, variables, ctx)? * eval(&b, variables, ctx)?),
                BinaryOperation::Divide => Ok(eval(&a, variables, ctx)? / eval(&b, variables, ctx)?),
                BinaryOperation::Modulo => Ok(eval(&a, variables, ctx)? % eval(&b, variables, ctx)?),
                BinaryOperation::Exponent => Ok(f64::powf(eval(&a, variables, ctx)?, eval(&b, variables, ctx)?)),

                // Logical Operations
                BinaryOperation::LessThan => Ok((eval(&a, variables, ctx)? < eval(&b, variables, ctx)?) as u64 as f64),
                BinaryOperation::GreaterThan => Ok((eval(&a, variables, ctx)? > eval(&b, variables, ctx)?) as u64 as f64),
                BinaryOperation::LessThanOrEqual => Ok((eval(&a, variables, ctx)? <= eval(&b, variables, ctx)?) as u64 as f64),
                BinaryOperation::GreaterThanOrEqual => Ok((eval(&a, variables, ctx)? >= eval(&b, variables, ctx)?) as u64 as f64),
                BinaryOperation::Equal => Ok((eval(&a, variables, ctx)? == eval(&b, variables, ctx)?) as u64 as f64),
                BinaryOperation::And => Ok((eval(&a, variables, ctx)? >= 1.0 && eval(&b, variables, ctx)? >= 1.0) as u64 as f64),
                BinaryOperation::Or => Ok((eval(&a, variables, ctx)? >= 1.0 || eval(&b, variables, ctx)? >= 1.0) as u64 as f64),
            }
        },
        Expr::Variable(name) => {
            let result = match name.as_str() {
                "x" => Some(ctx.x),
                "y" => Some(ctx.y),
                "index" => Some(ctx.index),
                "count" => Some(ctx.count),
                "fraction" => Some(ctx.fraction),
                "pi" => Some(ctx.pi),
                "tau" => Some(ctx.tau),
                "time" => Some(ctx.time),
                "projectionTime" => Some(ctx.projection_time),
                "projectionStartTime" => Some(ctx.projection_start_time),
                _ => match variables.get(&name) {
                    Some(value) => Some(*value),
                    None => None
                }
            };
            match result {
                Some(value) => Ok(value),
                None => Err(RawEvalError { 
                    error: format!("Cannot find variable '{name}'. Are you using it too early?"),
                    span: span.clone()
                })

            }
        }
    }
}

fn function(name: &str, exp_count: u8, c: impl Fn(Vec<f64>) -> f64, span: &Span, arguments: Vec<f64>) -> Result<f64, RawEvalError> {
    if arguments.len() == exp_count.into() {
        Ok(c(arguments))
    } else {
        let actual_count = arguments.len();
        let exp_count_text = if exp_count == 1 {"argument"} else {"arguments"};
        let actual_count_text = if actual_count == 1 {"argument"} else {"arguments"};

        Err(RawEvalError {
            error: format!("Function '{name}' expected {exp_count} {exp_count_text}, but only got {actual_count} {actual_count_text}."),
            span: span.clone()
        })
    }
}

fn function_complex<T>(name: &str, exp_count: u8, mut c: impl FnMut(Vec<T>) -> Result<f64, RawEvalError>, span: &Span, arguments: Vec<T>) -> Result<f64, RawEvalError> {
    if arguments.len() == exp_count.into() {
        c(arguments)
    } else {
        let actual_count = arguments.len();
        let exp_count_text = if exp_count == 1 {"argument"} else {"arguments"};
        let actual_count_text = if actual_count == 1 {"argument"} else {"arguments"};

        Err(RawEvalError {
            error: format!("Function '{name}' expected {exp_count} {exp_count_text}, but only got {actual_count} {actual_count_text}."),
            span: span.clone()
        })
    }
}

pub fn run(assignments: Vec<Assignment>, text: String, variables: &mut AHashMap<String, f64>, ctx: EvalContext) -> (&mut AHashMap<String, f64>, Vec<Error>) {
    let mut errors: Vec<Error> = vec!();

    for assignment in assignments {
        let eval_result = eval(&assignment.expression, variables, ctx);

        match eval_result {
            Ok(value) => { variables.insert(assignment.name, value); },
            Err(error) => {
                let loc = get_position_from_span(error.span, text.clone());

                errors.push(Error {
                    line_number: loc.0,
                    col_number: loc.1,
                    reason: error.error,
                    error_type: ErrorType::EvaluationError
                });
            }
        }
    };
    
    (variables, errors)
}
