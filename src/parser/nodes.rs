enum Node {
    Expression(Expression),
    Literal(Literal),
    VariableAssignment(VariableAssignment),
    Variable(Variable),
    BooleanExpression(BooleanExpression),
    FunctionCall(FunctionCall)
}

enum Operator {
    MathematicalOperator(MathematicalOperator),
    LogicalOperator(LogicalOperator)
}

impl Operator {
    fn match_operator(operator: &str) -> Self {
        match operator {
            // mathematical operators
            "+" => return Operator::MathematicalOperator(MathematicalOperator::Add),
            "-" => return Operator::MathematicalOperator(MathematicalOperator::Subtract),
            "*" => return Operator::MathematicalOperator(MathematicalOperator::Multiply),
            "/" => return Operator::MathematicalOperator(MathematicalOperator::Divide),
            "^" => return Operator::MathematicalOperator(MathematicalOperator::Exponent),
            "%" => return Operator::MathematicalOperator(MathematicalOperator::Modulo),

            // logical operators
            "<" => return Operator::LogicalOperator(LogicalOperator::LessThan),
            ">" => return Operator::LogicalOperator(LogicalOperator::GreaterThan),
            "<=" => return Operator::LogicalOperator(LogicalOperator::LessThanOrEqualTo),
            ">=" => return Operator::LogicalOperator(LogicalOperator::GreaterThanOrEqualTo),
            "==" => return Operator::LogicalOperator(LogicalOperator::EqualTo),
            "&" => return Operator::LogicalOperator(LogicalOperator::And),
            "|" => return Operator::LogicalOperator(LogicalOperator::Or),
            "!" => return Operator::LogicalOperator(LogicalOperator::Not),

            // fallback
            _ => return Operator::MathematicalOperator(MathematicalOperator::InvalidOperation)
        }
    }
}

enum MathematicalOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
    InvalidOperation
}

enum LogicalOperator {
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    EqualTo,
    And,
    Or,
    Not
}

struct Expression {
    start: u64,
    end: u64,
    children: Box<Vec<Node>>
}

struct Literal {
    start: u64,
    end: u64,
    value: f64
}

struct VariableAssignment {
    start: u64,
    end: u64,
    name: String,
    value: Box<Node>
}

struct Variable {
    start: u64,
    end: u64,
    name: String
}

struct BooleanExpression {
    start: u64,
    end: u64,
    operation: Operator,
    left: Box<Node>,
    right: Box<Node>
}

struct FunctionCall {
    start: u64,
    end: u64,
    function: String,
    value: Box<Vec<Node>>
}
