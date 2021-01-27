mod pattern;
#[cfg(test)]
mod tests;
mod type_;

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ast::Decl;

impl<Aux> Display for Decl<Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "Decl({}, [", self.name)?;
        let mut first = true;
        for arg in &self.args {
            if first {
                first = false;
            } else {
                write!(fmt, ", ")?;
            }
            write!(fmt, "{}", arg)?;
        }
        write!(fmt, "], {})", self.body)
    }
}
