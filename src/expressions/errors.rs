pub enum ErrorType {
    ParseError,
    EvaluationError
}

pub struct Error {
    pub line_number: u64,
    pub col_number: u64,
    pub reason: String,
    pub error_type: ErrorType
}
