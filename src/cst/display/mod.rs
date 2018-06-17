#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter, Result as FmtResult};

use ast::Op;
use cst::{Decl, Expr};

impl Display for Decl {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.name)?;
        for arg in &self.args {
            write!(fmt, " {}", arg)?;
        }
        write!(fmt, " = {}", self.body)
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt_expr(self, 0, fmt)
    }
}

fn fmt_expr(expr: &Expr, prec: usize, fmt: &mut Formatter) -> FmtResult {
    match *expr {
        Expr::If(ref c, ref t, ref e) => {
            if prec > 0 {
                write!(fmt, "(")?;
            }
            write!(fmt, "if ")?;
            fmt_expr(c, 0, fmt)?;
            write!(fmt, " then ")?;
            fmt_expr(t, 0, fmt)?;
            write!(fmt, " else ")?;
            fmt_expr(e, 0, fmt)?;
            if prec > 0 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::List(ref es) => {
            write!(fmt, "[")?;
            let mut first = true;
            for e in es {
                if first {
                    first = false;
                } else {
                    write!(fmt, "; ")?;
                }
                fmt_expr(e, 0, fmt)?;
            }
            write!(fmt, "]")
        }
        Expr::Literal(lit) => write!(fmt, "{}", lit),
        Expr::Op(Op::Add, ref l, ref r) => {
            if prec > 1 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 1, fmt)?;
            write!(fmt, " + ")?;
            fmt_expr(r, 2, fmt)?;
            if prec > 1 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::App, ref l, ref r) => {
            if prec > 3 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 3, fmt)?;
            write!(fmt, " ")?;
            fmt_expr(r, 4, fmt)?;
            if prec > 3 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::Cons, ref l, ref r) => {
            if prec > 0 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 1, fmt)?;
            write!(fmt, " :: ")?;
            fmt_expr(r, 0, fmt)?;
            if prec > 0 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::Div, ref l, ref r) => {
            if prec > 2 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 2, fmt)?;
            write!(fmt, " / ")?;
            fmt_expr(r, 3, fmt)?;
            if prec > 2 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::Mod, ref l, ref r) => {
            if prec > 2 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 2, fmt)?;
            write!(fmt, " mod ")?;
            fmt_expr(r, 3, fmt)?;
            if prec > 2 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::Mul, ref l, ref r) => {
            if prec > 2 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 2, fmt)?;
            write!(fmt, " * ")?;
            fmt_expr(r, 3, fmt)?;
            if prec > 2 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Op(Op::Sub, ref l, ref r) => {
            if prec > 1 {
                write!(fmt, "(")?;
            }
            fmt_expr(l, 1, fmt)?;
            write!(fmt, " - ")?;
            fmt_expr(r, 2, fmt)?;
            if prec > 1 {
                write!(fmt, ")")?;
            }
            Ok(())
        }
        Expr::Variable(sym) => write!(fmt, "{}", sym),
    }
}
