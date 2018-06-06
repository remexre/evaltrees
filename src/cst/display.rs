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
        match *self {
            Expr::If(ref c, ref t, ref e) => write!(fmt, "(if {} then {} else {})", c, t, e),
            Expr::List(ref es) => {
                write!(fmt, "[")?;
                let mut first = true;
                for e in es {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, "; ")?;
                    }
                    write!(fmt, "{}", e)?;
                }
                write!(fmt, "]")
            }
            Expr::Literal(lit) => write!(fmt, "{}", lit),
            Expr::Op(op, ref l, ref r) => write!(
                fmt,
                "({}{}{})",
                l,
                match op {
                    Op::Add => " + ",
                    Op::App => " ",
                    Op::Cons => " :: ",
                    Op::Div => " / ",
                    Op::Mod => " mod ",
                    Op::Mul => " * ",
                    Op::Sub => " - ",
                },
                r
            ),
            Expr::Variable(sym) => write!(fmt, "{}", sym),
        }
    }
}
