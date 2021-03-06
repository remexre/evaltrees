mod apply;
#[cfg(test)]
mod tests;

use display_attr::DisplayAttr;
use failure::{err_msg, Error};

use crate::ast::{Decl, Expr, Literal, Op, PrintStyle};
use crate::eval::{
    util::{beta_number, reducible},
    value::apply::apply,
    Evaluator,
};

/// Call-by-value evaluation.
#[derive(Debug, DisplayAttr)]
#[display(fmt = "{}", arg = "expr.as_ref().unwrap().display_as(*print_style)")]
pub struct CallByValue {
    decls: Vec<Decl<()>>,
    expr: Option<Expr<()>>,
    print_style: PrintStyle,
}

impl CallByValue {
    /// Creates a call-by-value interpreter from a list of declarations.
    /// The nameless declaration will be the expression.
    pub fn new(decls: Vec<Decl<()>>) -> CallByValue {
        CallByValue {
            decls,
            expr: Some(Expr::Variable("".into(), ())),
            print_style: PrintStyle::AST,
        }
    }
}

impl Evaluator for CallByValue {
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

fn step(expr: Expr<()>, decls: &[Decl<()>]) -> Result<Expr<()>, Error> {
    let beta = beta_number(&expr, decls);
    let expr = match expr {
        Expr::If(c, t, e, ()) => match *c {
            Expr::Literal(Literal::True, ()) => *t,
            Expr::Literal(Literal::False, ()) => *e,
            c => Expr::If(Box::new(step(c, decls)?), t, e, ()),
        },
        Expr::Literal(l, ()) => Expr::Literal(l, ()),
        Expr::Op(Op::App, l, r, ()) => {
            if reducible(&l, decls) {
                Expr::Op(Op::App, Box::new(step(*l, decls)?), r, ())
            } else if reducible(&r, decls) {
                Expr::Op(Op::App, l, Box::new(step(*r, decls)?), ())
            } else {
                match beta {
                    Some(n) if n > 0 => Expr::Op(Op::App, l, r, ()),
                    Some(0) => {
                        let mut args = vec![*r];
                        let mut func = *l;
                        while let Expr::Op(Op::App, f, a, ()) = func {
                            args.push(*a);
                            func = *f;
                        }
                        args.reverse();
                        let func = match func {
                            Expr::Variable(var, ()) => var,
                            func => panic!("Invalid callable expression: {}", func),
                        };
                        apply(func, &args, decls)?
                    }
                    _ => Expr::Op(Op::App, Box::new(step(*l, decls)?), r, ()),
                }
            }
        }
        Expr::Op(Op::Cons, l, r, ()) => {
            if reducible(&l, decls) {
                Expr::Op(Op::Cons, Box::new(step(*l, decls)?), r, ())
            } else {
                Expr::Op(Op::Cons, l, Box::new(step(*r, decls)?), ())
            }
        }
        Expr::Op(Op::Add, l, r, ()) => math_op(Op::Add, l, r, decls, |l, r| Ok(l + r))?,
        Expr::Op(Op::Sub, l, r, ()) => math_op(Op::Sub, l, r, decls, |l, r| Ok(l - r))?,
        Expr::Op(Op::Mul, l, r, ()) => math_op(Op::Mul, l, r, decls, |l, r| Ok(l * r))?,
        Expr::Op(Op::Div, l, r, ()) => math_op(Op::Div, l, r, decls, |l, r| {
            if r == 0 {
                Err(err_msg("division by zero"))
            } else {
                Ok(l / r)
            }
        })?,
        Expr::Op(Op::Mod, l, r, ()) => math_op(Op::Mod, l, r, decls, |l, r| {
            if r == 0 {
                Err(err_msg("mod by zero"))
            } else {
                Ok(l % r)
            }
        })?,
        Expr::Variable(var, ()) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                decl.body.clone()
            } else {
                Expr::Variable(var, ())
            }
        }
    };
    Ok(expr)
}

fn math_op<F: Fn(usize, usize) -> Result<usize, Error>>(
    op: Op,
    l: Box<Expr<()>>,
    r: Box<Expr<()>>,
    decls: &[Decl<()>],
    f: F,
) -> Result<Expr<()>, Error> {
    if let Expr::Literal(Literal::Int(ln), ()) = *l {
        if let Expr::Literal(Literal::Int(rn), ()) = *r {
            f(ln, rn).map(|n| Expr::Literal(Literal::Int(n), ()))
        } else {
            Ok(Expr::Op(op, l, Box::new(step(*r, decls)?), ()))
        }
    } else {
        Ok(Expr::Op(op, Box::new(step(*l, decls)?), r, ()))
    }
}
