use super::data::CalcuatedPoint;
use crate::expressions::errors;
use ahash::AHashMap;
use std::result::Result;

pub enum RetrievalError {
    MissingName,
    MissingValue,
}

pub trait ExecutionContext {
    /// Loads an expression, generating the data required (AST, shader, etc.) for the current
    /// ExecutionContext ahead of execution.
    fn load(&mut self, expression: String) -> Result<(), errors::Error>;
    /// Executes an expression, returning a vector of calculated points and boolean indicating if
    /// an error that did not stop execution was detected.
    fn execute(&mut self) -> Result<(Vec<CalcuatedPoint>, bool), errors::Error>;

    /// Retrieves any errors that may have occured at the specified index, i.
    fn retrieve_errors(&mut self, i: u16) -> Result<Vec<Vec<errors::Error>>, RetrievalError>;
    /// Retrieves any declared variables tbat were generated at the specified index, i.
    fn retrieve_variables(&mut self, i: u16) -> Result<Vec<AHashMap<String, f64>>, RetrievalError>;
}
