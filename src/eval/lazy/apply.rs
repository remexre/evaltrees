use std::collections::{BTreeMap, BTreeSet};

use failure::Error;
use symbol::Symbol;

use crate::ast::{Decl, Expr, Literal, Op, Pattern};
use crate::eval::lazy::{expr::LazyExpr, reduce::reducible, step};

/// Performs function application if possible, or reduces one of the arguments if not.
pub fn try_apply(
    func: Symbol,
    args: Vec<LazyExpr>,
    decls: &[Decl<()>],
    wherevars: &mut Vec<LazyExpr>,
) -> Result<(LazyExpr, Option<(usize, LazyExpr)>), Error> {
    for decl in decls.iter().filter(|decl| decl.name == func) {
        assert_eq!(args.len(), decl.args.len());
        if let Some(i) = args
            .iter()
            .zip(&decl.args)
            .position(|(a, p)| !arg_normal_enough_for_pat(a, p, decls, wherevars))
        {
            let mut expr = LazyExpr::Variable(func);
            let mut replacement = None;
            for (n, arg) in args.into_iter().enumerate() {
                let arg = if n == i {
                    let (arg, r) = step(arg, decls, wherevars)?;
                    replacement = r;
                    arg
                } else {
                    arg
                };
                expr = LazyExpr::Op(Op::App, Box::new(expr), Box::new(arg));
            }
            return Ok((expr, replacement));
        } else if let Some(bindings) = bind_args(&decl.args, &args) {
            return Ok((apply_to(decl, bindings, wherevars), None));
        }
    }
    bail!("No matching clauses for call to {}", func)
}

fn apply_to(
    decl: &Decl<()>,
    mut bindings: BTreeMap<Symbol, LazyExpr>,
    wherevars: &mut Vec<LazyExpr>,
) -> LazyExpr {
    let argvars = decl
        .args
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
            let arg = bindings.remove(&var).unwrap();
            wherevars.push(arg);
            bindings.insert(var, LazyExpr::WhereVar(n));
        }
    }

    instantiate_body(&decl.body, &bindings)
}

fn arg_normal_enough_for_pat(
    value: &LazyExpr,
    pat: &Pattern<()>,
    decls: &[Decl<()>],
    wherevars: &[LazyExpr],
) -> bool {
    match (value, pat) {
        (&LazyExpr::WhereVar(n), _) => {
            arg_normal_enough_for_pat(&wherevars[n], pat, decls, wherevars)
        }
        (_, &Pattern::Binding(_, _)) => true,
        (&LazyExpr::Literal(Literal::Int(_)), &Pattern::Literal(Literal::Int(_), _)) => true,
        (&LazyExpr::Literal(Literal::Nil), &Pattern::Literal(Literal::Nil, _)) => true,
        (&LazyExpr::Op(Op::Cons, _, _), &Pattern::Literal(Literal::Nil, _)) => true,
        (_, &Pattern::Literal(_, _)) => false,
        (&LazyExpr::Op(Op::Cons, ref eh, ref et), &Pattern::Cons(ref ph, ref pt, _)) => {
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

fn bind_arg(pattern: &Pattern<()>, expr: &LazyExpr) -> Option<BTreeMap<Symbol, LazyExpr>> {
    match (pattern, expr) {
        (&Pattern::Binding(var, _), e) => {
            let mut map = BTreeMap::new();
            map.insert(var, e.clone());
            Some(map)
        }
        (&Pattern::Cons(ref pl, ref pr, _), &LazyExpr::Op(Op::Cons, ref el, ref er)) => {
            let mut map = bind_arg(pl, el)?;
            map.extend(bind_arg(pr, er)?);
            Some(map)
        }
        (&Pattern::Literal(l1, _), &LazyExpr::Literal(l2)) if l1 == l2 => Some(BTreeMap::new()),
        _ => None,
    }
}

fn bind_args(pats: &[Pattern<()>], args: &[LazyExpr]) -> Option<BTreeMap<Symbol, LazyExpr>> {
    let mut map = BTreeMap::new();
    for (pat, arg) in pats.iter().zip(args) {
        map.extend(bind_arg(pat, arg)?);
    }
    Some(map)
}

fn instantiate_body(expr: &Expr<()>, replacement: &BTreeMap<Symbol, LazyExpr>) -> LazyExpr {
    match *expr {
        Expr::If(ref c, ref t, ref e, _) => LazyExpr::If(
            Box::new(instantiate_body(c, replacement)),
            Box::new(instantiate_body(t, replacement)),
            Box::new(instantiate_body(e, replacement)),
        ),
        Expr::Literal(lit, _) => LazyExpr::Literal(lit),
        Expr::Op(op, ref l, ref r, _) => LazyExpr::Op(
            op,
            Box::new(instantiate_body(l, replacement)),
            Box::new(instantiate_body(r, replacement)),
        ),
        Expr::Variable(ref var, _) => {
            if let Some(expr) = replacement.get(var) {
                expr.clone()
            } else {
                LazyExpr::Variable(*var)
            }
        }
    }
}
