//! Utilities for implementing a REPL.

use ast::{Decl, PrintStyle, Type};
use cst::{Decl as CstDecl, Expr};
use eval::Evaluator;

/// A command entered at the REPL.
#[derive(Clone, Debug, PartialEq)]
pub enum ReplCommand {
    /// Adds a new declaration.
    Decl(CstDecl),

    /// Sets the function used to construct an evaluator.
    Evaluator(fn(Vec<Decl<Type>>) -> Box<Evaluator<Type>>),

    /// Evaluates an expression.
    Expr(Expr),

    /// Prints a help menu.
    Help,

    /// Lists all declarations.
    List,

    /// Changes the print style.
    PrintStyle(PrintStyle),

    /// Quits the REPL.
    Quit,

    /// Clears all declarations from the REPL.
    Reset,

    /// Gets the type of an expression.
    Typeof(Expr),
}

impl ReplCommand {
    /// Returns a help message for the commands.
    pub fn help() -> &'static str {
        r"<expr>              Evaluates an expression
:decl <decl>        Adds a declaration
:l                  Lists all declarations
:list               Lists all declarations
:t <expr>           Prints the type of an expression

:cbn                Switches to call-by-name evaluation
:cbv                Switches to call-by-value evaluation
:lazy               Switches to lazy evaluation

:ast                Switches to AST print style
:cst                Switches to CST print style

:help               Prints this help message
:q                  Quits the REPL
:quit               Quits the REPL
:reset              Removes all decls from the REPL"
    }
}
