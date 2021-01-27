use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ast::Pattern;

impl<Aux> Display for Pattern<Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", PatternCSTDisplay(self, true))
    }
}

struct PatternCSTDisplay<'a, Aux: 'a>(&'a Pattern<Aux>, bool);

impl<'a, Aux: 'a> Display for PatternCSTDisplay<'a, Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self.0 {
            Pattern::Binding(n, _) => write!(fmt, "{}", n),
            Pattern::Cons(ref l, ref r, _) => {
                if self.1 {
                    write!(fmt, "(")?;
                }
                write!(
                    fmt,
                    "{} :: {}",
                    PatternCSTDisplay(l, true),
                    PatternCSTDisplay(r, false)
                )?;
                if self.1 {
                    write!(fmt, ")")?;
                }
                Ok(())
            }
            Pattern::Literal(l, _) => write!(fmt, "{}", l),
        }
    }
}
