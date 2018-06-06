//! Helpers for implementing evaluators.

use std::collections::BTreeMap;

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Op, Pattern};

/// Returns whether the given expression is reducible, given the decls in scope.
pub fn reducible<Aux>(expr: &Expr<Aux>, decls: &[Decl<Aux>]) -> bool {
    match *expr {
        Expr::If(_, _, _, _) => true,
        Expr::Literal(_, _) => false,
        Expr::Op(Op::App, ref l, ref r, _) => {
            reducible(l, decls) || reducible(r, decls)
                || beta_number(expr, decls).map(|n| n == 0).unwrap_or(false)
        }
        Expr::Op(Op::Cons, ref l, ref r, _) => reducible(l, decls) || reducible(r, decls),
        Expr::Op(_, _, _, _) => true,
        Expr::Variable(var, _) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            decl.args.is_empty()
        }
    }
}

/// Returns a number corresponding to the beta-reducibility of the given function application. This
/// is necessary due to the fact that function applications occur all at once. For example:
///
/// `add 1 (2 + 3)` should reduce to
///
/// `add 1 5`, rather than the "correct" answer of
///
/// `(\x. 1 + x) (2 + 3)`.
///
/// Returns `None` if the given expression is not a function application or a reference to a
/// function. Panics if the expression refers to a decl not in scope.
///
/// Note that beta numbers may be negative; for example, `id id 0` has a beta number of -1.
pub fn beta_number<Aux>(expr: &Expr<Aux>, decls: &[Decl<Aux>]) -> Option<isize> {
    match *expr {
        // An application has a beta number equal to the beta number of its left argument minus
        // one.
        Expr::Op(Op::App, ref l, _, _) => beta_number(l, decls).map(|n| n - 1),

        // A variable has a beta number equal to its arity.
        Expr::Variable(var, _) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            Some(decl.args.len() as isize)
        }

        // Other expressions don't have a beta number.
        _ => None,
    }
}

/// Performs "normal" (for call-by-name and call-by-value) function application.
pub fn apply<Aux: Clone>(
    func: Expr<Aux>,
    args: Vec<Expr<Aux>>,
    decls: &[Decl<Aux>],
) -> Result<Expr<Aux>, Error> {
    let func = match func {
        Expr::Variable(var, _) => var,
        func => panic!("Invalid callable expression: {}", func),
    };
    let (decl, args) = decls
        .iter()
        .filter(|decl| decl.name == func)
        .filter_map(|decl| matches_all(&decl.args, &args).map(|args| (decl, args)))
        .next()
        .ok_or_else(|| format_err!("Pattern match failed when calling {}", func))?;
    Ok(apply_replacement(&decl.body, &args))
}

fn apply_replacement<Aux: Clone>(
    expr: &Expr<Aux>,
    replacement: &BTreeMap<Symbol, Expr<Aux>>,
) -> Expr<Aux> {
    match *expr {
        Expr::If(ref c, ref t, ref e, ref aux) => Expr::If(
            Box::new(apply_replacement(c, replacement)),
            Box::new(apply_replacement(t, replacement)),
            Box::new(apply_replacement(e, replacement)),
            aux.clone(),
        ),
        Expr::Literal(lit, ref aux) => Expr::Literal(lit, aux.clone()),
        Expr::Op(op, ref l, ref r, ref aux) => Expr::Op(
            op,
            Box::new(apply_replacement(l, replacement)),
            Box::new(apply_replacement(r, replacement)),
            aux.clone(),
        ),
        Expr::Variable(ref var, ref aux) => {
            if let Some(expr) = replacement.get(var) {
                expr.clone()
            } else {
                Expr::Variable(*var, aux.clone())
            }
        }
    }
}

fn matches_all<Aux: Clone>(
    pats: &[Pattern<Aux>],
    args: &[Expr<Aux>],
) -> Option<BTreeMap<Symbol, Expr<Aux>>> {
    assert_eq!(pats.len(), args.len());
    let mut map = BTreeMap::new();
    for (pat, arg) in pats.iter().zip(args) {
        map.extend(pat.matches(arg)?);
    }
    Some(map)
}
