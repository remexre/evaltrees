mod expr;
mod reduce;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem::replace;

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, Pattern, PrintStyle};
pub(crate) use eval::lazy::expr::LazyExpr;
use eval::{lazy::reduce::{beta_number, reducible},
           util::transitive_closure,
           Evaluator};

/// Lazy evaluation.
#[derive(Debug)]
pub struct LazyEvaluation<Aux> {
    decls: Vec<Decl<Aux>>,
    expr: Option<LazyExpr<Aux>>,
    print_style: PrintStyle,
    wherevars: Vec<LazyExpr<Aux>>,
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
}

impl<Aux: Clone> Display for LazyEvaluation<Aux> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "{}",
            self.expr.as_ref().unwrap().display_as(self.print_style)
        )?;
        let iter = transitive_closure(self.expr.as_ref().unwrap().where_vars_indices(), |&num| {
            self.wherevars[num].where_vars_indices()
        }).into_iter()
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

impl<Aux: Clone> Evaluator<Aux> for LazyEvaluation<Aux> {
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
fn step<Aux: Clone>(
    expr: LazyExpr<Aux>,
    decls: &[Decl<Aux>],
    wherevars: &mut Vec<LazyExpr<Aux>>,
) -> Result<(LazyExpr<Aux>, Option<(usize, LazyExpr<Aux>)>), Error> {
    let beta = beta_number(&expr, decls);
    let expr = match expr {
        LazyExpr::If(c, t, e, aux) => match *c {
            LazyExpr::Literal(Literal::True, _) => (*t, None),
            LazyExpr::Literal(Literal::False, _) => (*e, None),
            c => {
                let (c, replacement) = step(c, decls, wherevars)?;
                (LazyExpr::If(Box::new(c), t, e, aux), replacement)
            }
        },
        LazyExpr::Literal(l, aux) => (LazyExpr::Literal(l, aux), None),
        LazyExpr::Op(Op::App, l, r, aux) => match beta {
            Some(n) if n > 0 => (LazyExpr::Op(Op::App, l, r, aux), None),
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
                if let Some(n) = check_arg_normalization(func_name, &args, decls, wherevars) {
                    normalize_arg(n, func, args, r_types, decls, wherevars)?
                } else {
                    apply(func_name, args, decls, wherevars)?
                }
            }
            _ => {
                let (l, replacement) = step(*l, decls, wherevars)?;
                (LazyExpr::Op(Op::App, Box::new(l), r, aux), replacement)
            }
        },
        LazyExpr::Op(Op::Cons, l, r, aux) => if reducible(&l, decls) {
            let (l, replacement) = step(*l, decls, wherevars)?;
            (LazyExpr::Op(Op::Cons, Box::new(l), r, aux), replacement)
        } else {
            let (r, replacement) = step(*r, decls, wherevars)?;
            (LazyExpr::Op(Op::Cons, l, Box::new(r), aux), replacement)
        },
        LazyExpr::Op(Op::Add, l, r, aux) => {
            math_op(Op::Add, l, r, aux, decls, |l, r| Ok(l + r), wherevars)?
        }
        LazyExpr::Op(Op::Sub, l, r, aux) => {
            math_op(Op::Sub, l, r, aux, decls, |l, r| Ok(l - r), wherevars)?
        }
        LazyExpr::Op(Op::Mul, l, r, aux) => {
            math_op(Op::Mul, l, r, aux, decls, |l, r| Ok(l * r), wherevars)?
        }
        LazyExpr::Op(Op::Div, l, r, aux) => {
            math_op(Op::Div, l, r, aux, decls, |l, r| Ok(l / r), wherevars)?
        }
        LazyExpr::Op(Op::Mod, l, r, aux) => {
            math_op(Op::Mod, l, r, aux, decls, |l, r| Ok(l % r), wherevars)?
        }
        LazyExpr::Variable(var, aux) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            if decl.args.is_empty() {
                (decl.body.clone().into(), None)
            } else {
                (LazyExpr::Variable(var, aux), None)
            }
        }
        LazyExpr::WhereVar(num, aux) => {
            if reducible(&wherevars[num], decls) {
                let expr = LazyExpr::WhereVar(num, aux);
                let expr = replace(&mut wherevars[num], expr);
                let (expr, replacement) = step(expr, decls, wherevars)?;
                (replace(&mut wherevars[num], expr), replacement)
            } else {
                let expr = wherevars[num].clone();
                (LazyExpr::WhereVar(num, aux), Some((num, expr)))
            }
        }
    };
    Ok(expr)
}

/// Performs function application.
pub fn apply<Aux: Clone>(
    func: Symbol,
    args: Vec<LazyExpr<Aux>>,
    decls: &[Decl<Aux>],
    wherevars: &mut Vec<LazyExpr<Aux>>,
) -> Result<(LazyExpr<Aux>, Option<(usize, LazyExpr<Aux>)>), Error> {
    let (decl, mut args) = decls
        .iter()
        .filter(|decl| decl.name == func)
        .filter_map(|decl| matches_all(&decl.args, &args, wherevars).map(|args| (decl, args)))
        .next()
        .ok_or_else(|| format_err!("Pattern match failed when calling {}", func))?;

    let argvars = decl.args
        .iter()
        .map(|pat| pat.freevars())
        .fold(BTreeSet::new(), |mut a, b| {
            a.extend(b);
            a
        });
    let bodyvars = decl.body.free_count();
    for var in argvars {
        if bodyvars.get(&var).cloned().unwrap_or(0) > 1 {
            let n = wherevars.len();
            let arg = args.remove(&var).unwrap();
            let aux = arg.aux_ref().clone();
            wherevars.push(arg);
            args.insert(var, LazyExpr::WhereVar(n, aux));
        }
    }

    Ok((instantiate_body(&decl.body, &args), None))
}

fn instantiate_body<Aux: Clone>(
    expr: &Expr<Aux>,
    replacement: &BTreeMap<Symbol, LazyExpr<Aux>>,
) -> LazyExpr<Aux> {
    match *expr {
        Expr::If(ref c, ref t, ref e, ref aux) => LazyExpr::If(
            Box::new(instantiate_body(c, replacement)),
            Box::new(instantiate_body(t, replacement)),
            Box::new(instantiate_body(e, replacement)),
            aux.clone(),
        ),
        Expr::Literal(lit, ref aux) => LazyExpr::Literal(lit, aux.clone()),
        Expr::Op(op, ref l, ref r, ref aux) => LazyExpr::Op(
            op,
            Box::new(instantiate_body(l, replacement)),
            Box::new(instantiate_body(r, replacement)),
            aux.clone(),
        ),
        Expr::Variable(ref var, ref aux) => {
            if let Some(expr) = replacement.get(var) {
                expr.clone()
            } else {
                LazyExpr::Variable(*var, aux.clone())
            }
        }
    }
}

fn matches_all<Aux: Clone>(
    pats: &[Pattern<Aux>],
    args: &[LazyExpr<Aux>],
    wherevars: &[LazyExpr<Aux>],
) -> Option<BTreeMap<Symbol, LazyExpr<Aux>>> {
    assert_eq!(pats.len(), args.len());
    let mut map = BTreeMap::new();
    for (pat, arg) in pats.iter().zip(args) {
        map.extend(arg_matches_pattern(pat, arg, wherevars)?);
    }
    Some(map)
}

fn arg_matches_pattern<Aux: Clone>(
    pattern: &Pattern<Aux>,
    expr: &LazyExpr<Aux>,
    wherevars: &[LazyExpr<Aux>],
) -> Option<BTreeMap<Symbol, LazyExpr<Aux>>> {
    match (pattern, expr) {
        (_, &LazyExpr::WhereVar(n, _)) => arg_matches_pattern(pattern, &wherevars[n], wherevars),
        (&Pattern::Binding(var, _), e) => {
            let mut map = BTreeMap::new();
            map.insert(var, e.clone());
            Some(map)
        }
        (&Pattern::Cons(ref pl, ref pr, _), &LazyExpr::Op(Op::Cons, ref el, ref er, _)) => {
            let mut lm = arg_matches_pattern(pl, el, wherevars)?;
            lm.extend(arg_matches_pattern(pr, er, wherevars)?);
            Some(lm)
        }
        (&Pattern::Literal(l1, _), &LazyExpr::Literal(l2, _)) if l1 == l2 => Some(BTreeMap::new()),
        _ => None,
    }
}

/// Returns whether the given expression is normalized enough to be a valid nth argument to the
/// decl with the given name.
pub fn arg_normal_enough<Aux: Clone>(
    value: &LazyExpr<Aux>,
    n: usize,
    name: Symbol,
    decls: &[Decl<Aux>],
    wherevars: &[LazyExpr<Aux>],
) -> bool {
    let pats = decls
        .iter()
        .filter(|decl| decl.name == name)
        .map(|decl| &decl.args[n]);
    for pat in pats {
        if arg_normal_enough_for_pat(value, pat, decls, wherevars) {
            if arg_matches_pattern(pat, value, wherevars).is_some() {
                return true;
            }
        } else {
            return false;
        }
    }
    true
}

fn arg_normal_enough_for_pat<Aux>(
    value: &LazyExpr<Aux>,
    pat: &Pattern<Aux>,
    decls: &[Decl<Aux>],
    wherevars: &[LazyExpr<Aux>],
) -> bool {
    match (value, pat) {
        (&LazyExpr::WhereVar(n, _), _) => {
            arg_normal_enough_for_pat(&wherevars[n], pat, decls, wherevars)
        }
        (_, &Pattern::Binding(_, _)) => true,
        (&LazyExpr::Literal(_, _), &Pattern::Literal(_, _)) => true,
        (_, &Pattern::Literal(_, _)) => false,
        (&LazyExpr::Op(Op::Cons, ref eh, ref et, _), &Pattern::Cons(ref ph, ref pt, _)) => {
            arg_normal_enough_for_pat(eh, ph, decls, wherevars)
                && arg_normal_enough_for_pat(et, pt, decls, wherevars)
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
    args: &[LazyExpr<Aux>],
    decls: &[Decl<Aux>],
    wherevars: &[LazyExpr<Aux>],
) -> Option<usize> {
    for (i, a) in args.iter().enumerate() {
        if !arg_normal_enough(a, i, func, decls, wherevars) {
            return Some(i);
        }
    }
    None
}

fn math_op<Aux: Clone, F: Fn(usize, usize) -> Result<usize, Error>>(
    op: Op,
    l: Box<LazyExpr<Aux>>,
    r: Box<LazyExpr<Aux>>,
    aux: Aux,
    decls: &[Decl<Aux>],
    f: F,
    wherevars: &mut Vec<LazyExpr<Aux>>,
) -> Result<(LazyExpr<Aux>, Option<(usize, LazyExpr<Aux>)>), Error> {
    if let LazyExpr::Literal(Literal::Int(ln), _) = *l {
        if let LazyExpr::Literal(Literal::Int(rn), _) = *r {
            f(ln, rn).map(|n| (LazyExpr::Literal(Literal::Int(n), aux), None))
        } else {
            let (r, replacement) = step(*r, decls, wherevars)?;
            Ok((LazyExpr::Op(op, l, Box::new(r), aux), replacement))
        }
    } else {
        let (l, replacement) = step(*l, decls, wherevars)?;
        Ok((LazyExpr::Op(op, Box::new(l), r, aux), replacement))
    }
}

fn normalize_arg<Aux: Clone>(
    n: usize,
    func: LazyExpr<Aux>,
    mut args: Vec<LazyExpr<Aux>>,
    mut r_types: Vec<Aux>,
    decls: &[Decl<Aux>],
    wherevars: &mut Vec<LazyExpr<Aux>>,
) -> Result<(LazyExpr<Aux>, Option<(usize, LazyExpr<Aux>)>), Error> {
    assert_eq!(args.len(), r_types.len());
    args.reverse();
    let mut out = func;
    let mut i = 0;
    let mut replacement = None;
    while let Some(arg) = args.pop() {
        let arg = if i == n {
            let (arg, r) = step(arg, decls, wherevars)?;
            replacement = Some(r);
            arg
        } else {
            arg
        };
        i += 1;
        out = LazyExpr::Op(
            Op::App,
            Box::new(out),
            Box::new(arg),
            r_types.pop().unwrap(),
        );
    }
    assert_eq!(n, 0);
    Ok((out, replacement.unwrap()))
}
