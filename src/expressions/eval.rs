use super::parser::Expr;
use super::parser::Assignment;
use super::errors::Error;
use super::errors::ErrorType;
use std::collections::HashMap;

fn eval(expr: &Expr, variables: &mut HashMap<String, f64>) -> Result<f64, String> {
    match expr {
        Expr::Number(x) => Ok(*x),
        Expr::Negate(a) => Ok(-eval(a, variables)?),
        Expr::Add(a, b) => Ok(eval(a, variables)? + eval(b, variables)?),
        Expr::Subtract(a, b) => Ok(eval(a, variables)? - eval(b, variables)?),
        Expr::Multiply(a, b) => Ok(eval(a, variables)? * eval(b, variables)?),
        Expr::Divide(a, b) => Ok(eval(a, variables)? / eval(b, variables)?),
        Expr::Modulo(a, b) => Ok(eval(a, variables)? / eval(b, variables)?),
        Expr::Exponent(a, b) => Ok(f64::powf(eval(a, variables)?, eval(b, variables)?)),
        Expr::Variable(name) => if let val = variables.get(name) {
            Ok(*val.unwrap())
        } else {
            Err(format!("Cannot find variable `{}`. Are you using it too early?", name))
        },
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
