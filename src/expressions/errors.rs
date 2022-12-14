use super::parser::Span;

#[derive(Debug, Clone)]
pub enum ErrorType {
    ParseError,
    EvaluationError,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub line_number: u64,
    pub col_number: u64,
    pub reason: String,
    pub error_type: ErrorType,
    pub id: u8, // we'll use this to show documentation later
}

pub fn get_position_from_span(span: Span, string: String) -> (u64, u64) {
    // we don't need the whole span info for the error message,
    // but we keep it the whole time for things like error highlighting
    let mut remaining_chars: u64 = span
        .start
        .try_into()
        .expect("text file is longer than 18446744073709551615 characters");
    let mut line = 0;

    for cur_line in string.lines() {
        line += 1;

        let len: u64 = cur_line.len().try_into().expect(
            "if you encounter this error then something is *extremely* wrong with your computer",
        );

        if remaining_chars > len {
            remaining_chars -= len + 1;
        } else {
            break;
        }
    }

    return (line, remaining_chars);
}
