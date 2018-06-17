#[cfg(test)]
mod tests;

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, Pattern, PrintStyle};
use eval::util::{apply, beta_number, reducible};
use eval::Evaluator;

/// Call-by-name evaluation.
#[derive(Debug, DisplayAttr)]
#[display(fmt = "{}", arg = "expr.as_ref().unwrap().display_as(*print_style)")]
pub struct CallByName<Aux> {
    decls: Vec<Decl<Aux>>,
    expr: Option<Expr<Aux>>,
    print_style: PrintStyle,
}

impl<Aux: Clone> CallByName<Aux> {
    /// Creates a call-by-name interpreter from a list of declarations.
    /// The nameless declaration will be the expression.
    pub fn new(decls: Vec<Decl<Aux>>) -> CallByName<Aux> {
        let expr = {
            let nameless = decls
                .iter()
                .find(|decl| decl.name == "".into())
                .expect("Nameless declaration missing");
            Expr::Variable("".into(), nameless.aux_ref().clone())
        };
        CallByName {
            decls,
            expr: Some(expr),
            print_style: PrintStyle::AST,
        }
    }
}

impl<Aux: Clone> Evaluator<Aux> for CallByName<Aux> {
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
                let mut r_types = vec![aux];
                while let Expr::Op(Op::App, f, a, ty) = func {
                    args.push(*a);
                    func = *f;
                    r_types.push(ty);
                }
                args.reverse();
                let func_name = match func {
                    Expr::Variable(var, _) => var,
                    func => panic!("Invalid callable expression: {}", func),
                };
                if let Some(n) = check_arg_normalization(func_name, &args, decls) {
                    normalize_arg(n, func, args, r_types, decls)?
                } else {
                    apply(func_name, args, decls)?
                }
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

/// Returns whether the given expression is normalized enough to be a valid nth argument to the
/// decl with the given name.
pub fn arg_normal_enough<Aux: Clone>(
    value: &Expr<Aux>,
    n: usize,
    name: Symbol,
    decls: &[Decl<Aux>],
) -> bool {
    let pats = decls
        .iter()
        .filter(|decl| decl.name == name)
        .map(|decl| &decl.args[n]);
    for pat in pats {
        if arg_normal_enough_for_pat(value, pat, decls) {
            if pat.matches(value).is_some() {
                return true;
            }
        } else {
            return false;
        }
    }
    true
}

fn arg_normal_enough_for_pat<Aux>(
    value: &Expr<Aux>,
    pat: &Pattern<Aux>,
    decls: &[Decl<Aux>],
) -> bool {
    match (value, pat) {
        (_, &Pattern::Binding(_, _)) => true,
        (_, &Pattern::Literal(_, _)) => true,
        (&Expr::Op(Op::Cons, ref eh, ref et, _), &Pattern::Cons(ref ph, ref pt, _)) => {
            arg_normal_enough_for_pat(eh, ph, decls) && arg_normal_enough_for_pat(et, pt, decls)
        }
        (e, &Pattern::Cons(_, _, _)) => {
            if !reducible(e, decls) {
                panic!("Looks like we're stuck; this should've been a type error earlier on?");
            }
            false
        }
    }
}

fn check_arg_normalization<Aux: Clone>(
    func: Symbol,
    args: &[Expr<Aux>],
    decls: &[Decl<Aux>],
) -> Option<usize> {
    for (i, a) in args.iter().enumerate() {
        if !arg_normal_enough(a, i, func, decls) {
            return Some(i);
        }
    }
    None
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

fn normalize_arg<Aux: Clone>(
    n: usize,
    func: Expr<Aux>,
    mut args: Vec<Expr<Aux>>,
    mut r_types: Vec<Aux>,
    decls: &[Decl<Aux>],
) -> Result<Expr<Aux>, Error> {
    assert_eq!(args.len(), r_types.len());
    args.reverse();
    let mut out = func;
    let mut i = 0;
    while let Some(arg) = args.pop() {
        let arg = if i == n { step(arg, decls)? } else { arg };
        i += 1;
        out = Expr::Op(
            Op::App,
            Box::new(out),
            Box::new(arg),
            r_types.pop().unwrap(),
        );
    }
    assert_eq!(n, 0);
    Ok(out)
}
