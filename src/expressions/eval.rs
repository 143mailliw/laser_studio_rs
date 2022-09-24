use super::parser::Expr;
use super::parser::BinaryOperation;
use super::parser::UnaryOperation;
use super::parser::Spanned;
use super::parser::Assignment;
use super::errors::Error;
use super::errors::ErrorType;
use std::collections::HashMap;

fn eval(spanned_expr: &Spanned<Box<Expr>>, variables: &mut HashMap<String, f64>) -> Result<f64, String> {
    let expr = spanned_expr.0.clone();
    let _span = spanned_expr.1.clone();

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
                None => Err(format!("Cannot find variable `{}`. Are you using it too early?", name))

            }
        }
        _ => todo!()
    }
}

pub fn run(assignments: Vec<Assignment>) -> (HashMap<String, f64>, Vec<Error>) {
    let mut variables: HashMap<String, f64> = HashMap::new();
    let mut errors: Vec<Error> = vec!();

    for assignment in assignments {
        let eval_result = eval(&assignment.expression, &mut variables);
        
        match eval_result {
            Ok(value) => { variables.insert(assignment.name, value); },
            Err(error) => { errors.push(Error { line_number: 0, col_number: 0, reason: error, error_type: ErrorType::EvaluationError }); }
        }
    };
    
    (variables, errors)
}
