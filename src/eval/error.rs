use thiserror::Error;
use symbol::Symbol;

#[derive(Error, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EvalError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Modulo by zero")]
    ModuloByZero,

    #[error("Unknown variable: {0}")]
    UnknownVariable(Symbol),

    #[error("Invalid callable expression: {0}")]
    InvalidCallable(String),

    #[error("Type error during evaluation: {0}")]
    TypeError(String), // For general type mismatches if any are found
}
