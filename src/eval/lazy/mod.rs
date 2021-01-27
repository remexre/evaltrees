mod apply;
mod expr;
mod reduce;
#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem::replace;

use failure::{err_msg, Error};

use crate::ast::{Decl, Literal, Op, PrintStyle};
use crate::eval::{
    lazy::{
        apply::try_apply,
        expr::LazyExpr,
        reduce::{beta_number, reducible},
    },
    util::transitive_closure,
    Evaluator,
};

/// Lazy evaluation.
#[derive(Debug)]
pub struct LazyEvaluation {
    decls: Vec<Decl<()>>,
    expr: Option<LazyExpr>,
    print_style: PrintStyle,
    wherevars: Vec<LazyExpr>,
}

impl LazyEvaluation {
    /// Creates a lazy-evalauation interpreter from a list of declarations. The nameless
    /// declaration will be the expression.
    pub fn new(decls: Vec<Decl<()>>) -> LazyEvaluation {
        LazyEvaluation {
            decls,
            expr: Some(LazyExpr::Variable("".into())),
            print_style: PrintStyle::AST,
            wherevars: Vec::new(),
        }
    }
}

impl Display for LazyEvaluation {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "{}",
            self.expr.as_ref().unwrap().display_as(self.print_style)
        )?;

        let iter = transitive_closure(self.expr.as_ref().unwrap().where_vars_indices(), |&num| {
            self.wherevars[num].where_vars_indices()
        })
        .into_iter()
        .map(|num| (num, &self.wherevars[num]));

        let mut first = true;
        for (n, expr) in iter {
            if first {
                first = false;
                write!(fmt, "\n  where")?;
            } else {
                write!(fmt, "\n       ")?;
            }
            write!(fmt, " ${} = {}", n, expr.display_as(self.print_style))?;
        }
        Ok(())
    }
}

impl Evaluator for LazyEvaluation {
    fn normal_form(&self) -> bool {
        !reducible(self.expr.as_ref().unwrap(), &self.decls)
    }

    fn set_print_style(&mut self, print_style: PrintStyle) {
        self.print_style = print_style;
    }

    fn step(&mut self) -> Result<(), Error> {
        let (mut expr, replacement) =
            step(self.expr.take().unwrap(), &self.decls, &mut self.wherevars)?;
        if let Some((n, e)) = replacement {
            expr.apply_replacement(n, &e);
            for expr in &mut self.wherevars {
                expr.apply_replacement(n, &e);
            }
        }
        self.expr = Some(expr);
        Ok(())
    }
}

// This is broadly similar to CBN, but a replacement can be returned in addition to a new
// expression.
fn step(
    expr: LazyExpr,
    decls: &[Decl<()>],
    wherevars: &mut Vec<LazyExpr>,
) -> Result<(LazyExpr, Option<(usize, LazyExpr)>), Error> {
    let beta = beta_number(&expr, decls);
    let expr = match expr {
        LazyExpr::If(c, t, e) => match *c {
            LazyExpr::Literal(Literal::True) => (*t, None),
            LazyExpr::Literal(Literal::False) => (*e, None),
            c => {
                let (c, replacement) = step(c, decls, wherevars)?;
                (LazyExpr::If(Box::new(c), t, e), replacement)
            }
        },
        LazyExpr::Literal(l) => (LazyExpr::Literal(l), None),
        LazyExpr::Op(Op::App, l, r) => match beta {
            Some(n) if n > 0 => (LazyExpr::Op(Op::App, l, r), None),
            Some(0) => {
                let mut args = vec![*r];
                let mut func = *l;
                while let LazyExpr::Op(Op::App, f, a) = func {
                    args.push(*a);
                    func = *f;
                }
                args.reverse();
                let func = match func {
                    LazyExpr::Variable(var) => var,
                    func => panic!("Invalid callable expression: {}", func),
                };
                try_apply(func, args, decls, wherevars)?
            }
            _ => {
                let (l, replacement) = step(*l, decls, wherevars)?;
                (LazyExpr::Op(Op::App, Box::new(l), r), replacement)
            }
        },
        LazyExpr::Op(Op::Cons, l, r) => {
            if reducible(&l, decls) {
                let (l, replacement) = step(*l, decls, wherevars)?;
                (LazyExpr::Op(Op::Cons, Box::new(l), r), replacement)
            } else {
                let (r, replacement) = step(*r, decls, wherevars)?;
                (LazyExpr::Op(Op::Cons, l, Box::new(r)), replacement)
            }
        }
        LazyExpr::Op(Op::Add, l, r) => math_op(Op::Add, l, r, decls, |l, r| Ok(l + r), wherevars)?,
        LazyExpr::Op(Op::Sub, l, r) => math_op(Op::Sub, l, r, decls, |l, r| Ok(l - r), wherevars)?,
        LazyExpr::Op(Op::Mul, l, r) => math_op(Op::Mul, l, r, decls, |l, r| Ok(l * r), wherevars)?,
        LazyExpr::Op(Op::Div, l, r) => math_op(
            Op::Div,
            l,
            r,
            decls,
            |l, r| {
                if r == 0 {
                    Err(err_msg("division by zero"))
                } else {
                    Ok(l / r)
                }
            },
            wherevars,
        )?,
        LazyExpr::Op(Op::Mod, l, r) => math_op(
            Op::Mod,
            l,
            r,
            decls,
            |l, r| {
                if r == 0 {
                    Err(err_msg("mod by zero"))
                } else {
                    Ok(l / r)
                }
            },
            wherevars,
        )?,
        LazyExpr::Variable(var) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                (decl.body.clone().into(), None)
            } else {
                (LazyExpr::Variable(var), None)
            }
        }
        LazyExpr::WhereVar(num) => {
            if reducible(&wherevars[num], decls) {
                let expr = LazyExpr::WhereVar(num);
                let expr = replace(&mut wherevars[num], expr);
                let (expr, replacement) = step(expr, decls, wherevars)?;
                (replace(&mut wherevars[num], expr), replacement)
            } else {
                let expr = wherevars[num].clone();
                (LazyExpr::WhereVar(num), Some((num, expr)))
            }
        }
    };
    Ok(expr)
}

fn math_op<F: Fn(usize, usize) -> Result<usize, Error>>(
    op: Op,
    l: Box<LazyExpr>,
    r: Box<LazyExpr>,
    decls: &[Decl<()>],
    f: F,
    wherevars: &mut Vec<LazyExpr>,
) -> Result<(LazyExpr, Option<(usize, LazyExpr)>), Error> {
    if let LazyExpr::Literal(Literal::Int(ln)) = *l {
        if let LazyExpr::Literal(Literal::Int(rn)) = *r {
            f(ln, rn).map(|n| (LazyExpr::Literal(Literal::Int(n)), None))
        } else {
            let (r, replacement) = step(*r, decls, wherevars)?;
            Ok((LazyExpr::Op(op, l, Box::new(r)), replacement))
        }
    } else {
        let (l, replacement) = step(*l, decls, wherevars)?;
        Ok((LazyExpr::Op(op, Box::new(l), r), replacement))
    }
}
