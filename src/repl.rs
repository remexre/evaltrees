//! Utilities for implementing a REPL.

use symbol::Symbol;

use ast::PrintStyle;
use cst::{Decl, Expr};

/// A command entered at the REPL.
#[derive(Clone, Debug, PartialEq)]
pub enum ReplCommand {
    /// Adds a new declaration.
    Decl(Decl),

    /// Evaluates an expression.
    Expr(Expr),

    /// Prints a help menu.
    Help,

    /// Changes the print style.
    PrintStyle(PrintStyle),

    /// Quits the REPL.
    Quit,

    /// Clears all declarations from the REPL.
    Reset,

    /// Gets the type of a symbol.
    Typeof(Symbol),
}

impl ReplCommand {
    /// Returns a help message for the commands.
    pub fn help() -> &'static str {
        "TODO"
    }
}
