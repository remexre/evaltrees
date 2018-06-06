use failure::Error;

use ast::{Decl, Expr, Literal, Op};
use eval::util::{apply, beta_number, reducible};
use eval::Evaluator;

/// Call-by-value evaluation.
#[derive(Debug, DisplayAttr)]
#[display(fmt = "{}", arg = "expr.as_ref().unwrap()")]
pub struct CallByValue<Aux> {
    decls: Vec<Decl<Aux>>,
    expr: Option<Expr<Aux>>,
}

impl<Aux: Clone> CallByValue<Aux> {
    /// Creates a call-by-value interpreter from a list of declarations.
    /// The nameless declaration will be the expression.
    pub fn new(decls: Vec<Decl<Aux>>) -> CallByValue<Aux> {
        let expr = {
            let nameless = decls
                .iter()
                .find(|decl| decl.name == "".into())
                .expect("Nameless declaration missing");
            Expr::Variable("".into(), nameless.aux_ref().clone())
        };
        CallByValue {
            decls,
            expr: Some(expr),
        }
    }
}

impl<Aux: Clone> Evaluator<Aux> for CallByValue<Aux> {
    fn normal_form(&self) -> bool {
        !reducible(self.expr.as_ref().unwrap(), &self.decls)
    }

    fn step(&mut self) -> Result<(), Error> {
        let expr = step(self.expr.take().unwrap(), &self.decls)?;
        self.expr = Some(expr);
        Ok(())
    }
}

fn step<Aux: Clone>(expr: Expr<Aux>, decls: &[Decl<Aux>]) -> Result<Expr<Aux>, Error> {
    let beta = beta_number(&expr, decls);
    let expr = match expr {
        Expr::If(c, t, e, aux) => match *c {
            Expr::Literal(Literal::True, _) => *t,
            Expr::Literal(Literal::False, _) => *e,
            c => Expr::If(Box::new(step(c, decls)?), t, e, aux),
        },
        Expr::Literal(l, aux) => Expr::Literal(l, aux),
        Expr::Op(Op::App, l, r, aux) => match beta {
            Some(n) if n > 0 => Expr::Op(Op::App, l, r, aux),
            Some(0) => {
                let mut args = vec![*r];
                let mut func = *l;
                while let Expr::Op(Op::App, f, a, _) = func {
                    args.push(*a);
                    func = *f;
                }
                args.reverse();
                apply(func, args, decls)?
            }
            _ => Expr::Op(Op::App, Box::new(step(*l, decls)?), r, aux),
        },
        Expr::Op(Op::Cons, l, r, aux) => if reducible(&l, decls) {
            Expr::Op(Op::Cons, Box::new(step(*l, decls)?), r, aux)
        } else {
            Expr::Op(Op::Cons, l, Box::new(step(*r, decls)?), aux)
        },
        Expr::Op(Op::Add, l, r, aux) => math_op(Op::Add, l, r, aux, decls, |l, r| Ok(l + r))?,
        Expr::Op(Op::Sub, l, r, aux) => math_op(Op::Sub, l, r, aux, decls, |l, r| Ok(l - r))?,
        Expr::Op(Op::Mul, l, r, aux) => math_op(Op::Mul, l, r, aux, decls, |l, r| Ok(l * r))?,
        Expr::Op(Op::Div, l, r, aux) => math_op(Op::Div, l, r, aux, decls, |l, r| Ok(l / r))?,
        Expr::Op(Op::Mod, l, r, aux) => math_op(Op::Mod, l, r, aux, decls, |l, r| Ok(l % r))?,
        Expr::Variable(var, aux) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                decl.body.clone()
            } else {
                Expr::Variable(var, aux)
            }
        }
    };
    Ok(expr)
}

fn math_op<Aux: Clone, F: Fn(usize, usize) -> Result<usize, Error>>(
    op: Op,
    l: Box<Expr<Aux>>,
    r: Box<Expr<Aux>>,
    aux: Aux,
    decls: &[Decl<Aux>],
    f: F,
) -> Result<Expr<Aux>, Error> {
    if let Expr::Literal(Literal::Int(ln), _) = *l {
        if let Expr::Literal(Literal::Int(rn), _) = *r {
            f(ln, rn).map(|n| Expr::Literal(Literal::Int(n), aux))
        } else {
            Ok(Expr::Op(op, l, Box::new(step(*r, decls)?), aux))
        }
    } else {
        Ok(Expr::Op(op, Box::new(step(*l, decls)?), r, aux))
    }
}
