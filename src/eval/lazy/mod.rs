mod expr;
mod reduce;
#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, PrintStyle};
use eval::{lazy::{expr::LazyExpr,
                  reduce::{beta_number, reducible}},
           util::{apply, arg_normal_enough},
           Evaluator};

/// Lazy evaluation.
#[derive(Debug)]
pub struct LazyEvaluation<Aux> {
    decls: Vec<Decl<Aux>>,
    expr: Option<LazyExpr<Aux>>,
    print_style: PrintStyle,
    wherevars: Vec<Option<LazyExpr<Aux>>>,
}

impl<Aux: Clone> LazyEvaluation<Aux> {
    /// Creates a lazy-evalauation interpreter from a list of declarations. The nameless
    /// declaration will be the expression.
    pub fn new(decls: Vec<Decl<Aux>>) -> LazyEvaluation<Aux> {
        let expr = {
            let nameless = decls
                .iter()
                .find(|decl| decl.name == "".into())
                .expect("Nameless declaration missing");
            LazyExpr::Variable("".into(), nameless.aux_ref().clone())
        };
        LazyEvaluation {
            decls,
            expr: Some(expr.into()),
            print_style: PrintStyle::AST,
            wherevars: Vec::new(),
        }
    }

    /// Returns the indices of all reachable where-bound variables.
    fn where_vars_indices(&self) -> HashSet<usize> {
        let mut set = self.expr.as_ref().unwrap().where_vars_indices();
        let mut added = true;
        while added {
            added = false;
            unimplemented!()
        }
        set
    }
}

impl<Aux: Clone> Display for LazyEvaluation<Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        writeln!(fmt, "{}", self.expr.as_ref().unwrap())?;
        let iter: Option<(usize, LazyExpr<Aux>)> = None;
        let mut first = true;
        for (n, expr) in iter {
            if first {
                first = false;
                write!(fmt, "  where")?;
            } else {
                write!(fmt, "\n       ")?;
            }
            write!(fmt, " ${} = {}", n, expr)?;
        }
        Ok(())
    }
}

impl<Aux: Clone> Evaluator<Aux> for LazyEvaluation<Aux> {
    fn normal_form(&self) -> bool {
        !reducible(self.expr.as_ref().unwrap(), &self.decls)
    }

    fn set_print_style(&mut self, print_style: PrintStyle) {
        self.print_style = print_style;
    }

    fn step(&mut self) -> Result<(), Error> {
        let expr = step(self.expr.take().unwrap(), &self.decls)?;
        self.expr = Some(expr);
        Ok(())
    }
}

fn step<Aux: Clone>(expr: LazyExpr<Aux>, decls: &[Decl<Aux>]) -> Result<LazyExpr<Aux>, Error> {
    let beta = beta_number(&expr, decls);
    let expr = match expr {
        LazyExpr::If(c, t, e, aux) => match *c {
            LazyExpr::Literal(Literal::True, _) => *t,
            LazyExpr::Literal(Literal::False, _) => *e,
            c => LazyExpr::If(Box::new(step(c, decls)?), t, e, aux),
        },
        LazyExpr::Literal(l, aux) => LazyExpr::Literal(l, aux),
        LazyExpr::Op(Op::App, l, r, aux) => match beta {
            Some(n) if n > 0 => LazyExpr::Op(Op::App, l, r, aux),
            Some(0) => {
                let mut args = vec![*r];
                let mut func = *l;
                let mut r_types = vec![aux];
                while let LazyExpr::Op(Op::App, f, a, ty) = func {
                    args.push(*a);
                    func = *f;
                    r_types.push(ty);
                }
                args.reverse();
                let func_name = match func {
                    LazyExpr::Variable(var, _) => var,
                    func => panic!("Invalid callable expression: {}", func),
                };
                unimplemented!()
                /*
                if let Some(n) = check_arg_normalization(func_name, &args, decls) {
                    normalize_arg(n, func, args, r_types, decls)?
                } else {
                    apply(func_name, args, decls)?
                }
                */
            }
            _ => LazyExpr::Op(Op::App, Box::new(step(*l, decls)?), r, aux),
        },
        LazyExpr::Op(Op::Cons, l, r, aux) => if reducible(&l, decls) {
            LazyExpr::Op(Op::Cons, Box::new(step(*l, decls)?), r, aux)
        } else {
            LazyExpr::Op(Op::Cons, l, Box::new(step(*r, decls)?), aux)
        },
        LazyExpr::Op(Op::Add, l, r, aux) => math_op(Op::Add, l, r, aux, decls, |l, r| Ok(l + r))?,
        LazyExpr::Op(Op::Sub, l, r, aux) => math_op(Op::Sub, l, r, aux, decls, |l, r| Ok(l - r))?,
        LazyExpr::Op(Op::Mul, l, r, aux) => math_op(Op::Mul, l, r, aux, decls, |l, r| Ok(l * r))?,
        LazyExpr::Op(Op::Div, l, r, aux) => math_op(Op::Div, l, r, aux, decls, |l, r| Ok(l / r))?,
        LazyExpr::Op(Op::Mod, l, r, aux) => math_op(Op::Mod, l, r, aux, decls, |l, r| Ok(l % r))?,
        LazyExpr::Variable(var, aux) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                decl.body.clone().into()
            } else {
                LazyExpr::Variable(var, aux)
            }
        }
        LazyExpr::WhereVar(num, aux) => {
            unimplemented!();
        }
    };
    Ok(expr)
}

/*
fn check_arg_normalization<Aux: Clone>(
    func: Symbol,
    args: &[LazyExpr<Aux>],
    decls: &[Decl<Aux>],
) -> Option<usize> {
    for (i, a) in args.iter().enumerate() {
        if !arg_normal_enough(a, i, func, decls) {
            return Some(i);
        }
    }
    None
}
*/

fn math_op<Aux: Clone, F: Fn(usize, usize) -> Result<usize, Error>>(
    op: Op,
    l: Box<LazyExpr<Aux>>,
    r: Box<LazyExpr<Aux>>,
    aux: Aux,
    decls: &[Decl<Aux>],
    f: F,
) -> Result<LazyExpr<Aux>, Error> {
    if let LazyExpr::Literal(Literal::Int(ln), _) = *l {
        if let LazyExpr::Literal(Literal::Int(rn), _) = *r {
            f(ln, rn).map(|n| LazyExpr::Literal(Literal::Int(n), aux))
        } else {
            Ok(LazyExpr::Op(op, l, Box::new(step(*r, decls)?), aux))
        }
    } else {
        Ok(LazyExpr::Op(op, Box::new(step(*l, decls)?), r, aux))
    }
}

/*
fn normalize_arg<Aux: Clone>(
    n: usize,
    func: LazyExpr<Aux>,
    mut args: Vec<LazyExpr<Aux>>,
    mut r_types: Vec<Aux>,
    decls: &[Decl<Aux>],
) -> Result<LazyExpr<Aux>, Error> {
    assert_eq!(args.len(), r_types.len());
    args.reverse();
    let mut out = func;
    let mut i = 0;
    while let Some(arg) = args.pop() {
        let arg = if i == n { step(arg, decls)? } else { arg };
        i += 1;
        out = LazyExpr::Op(
            Op::App,
            Box::new(out),
            Box::new(arg),
            r_types.pop().unwrap(),
        );
    }
    assert_eq!(n, 0);
    Ok(out)
}
*/
