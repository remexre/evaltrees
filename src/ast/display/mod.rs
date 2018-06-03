mod expr;
mod pattern;
#[cfg(test)]
mod tests;
mod type_;

use std::fmt::{Display, Formatter, Result as FmtResult};

use ast::Decl;

impl<Aux> Display for Decl<Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.name)?;
        for arg in &self.args {
            write!(fmt, " {}", arg)?;
        }
        write!(fmt, " = {}", self.body)
    }
}
