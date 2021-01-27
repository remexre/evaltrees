use std::collections::HashMap;

use failure::{format_err, Error};
use symbol::Symbol;

use crate::ast::{Decl, Expr, Pattern};

/// Performs "normal" (for call-by-name and call-by-value) function application.
pub fn apply<Aux: Clone>(
    func: Symbol,
    args: &[Expr<Aux>],
    decls: &[Decl<Aux>],
) -> Result<Expr<Aux>, Error> {
    let (args, body) = decls
        .iter()
        .filter(|decl| decl.name == func)
        .filter_map(|decl| matches_all(&decl.args, &args).map(|args| (args, &decl.body)))
        .next()
        .ok_or_else(|| format_err!("No matching declaration for call to function {}", func))?;
    Ok(apply_replacement(body, &args))
}

fn matches_all<Aux: Clone>(
    pats: &[Pattern<Aux>],
    args: &[Expr<Aux>],
) -> Option<HashMap<Symbol, Expr<Aux>>> {
    assert_eq!(pats.len(), args.len());
    let mut map = HashMap::new();
    for (pat, arg) in pats.iter().zip(args) {
        map.extend(pat.matches(arg)?);
    }
    Some(map)
}

fn apply_replacement<Aux: Clone>(
    expr: &Expr<Aux>,
    replacement: &HashMap<Symbol, Expr<Aux>>,
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
