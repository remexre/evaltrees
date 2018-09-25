use std::collections::BTreeMap;

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, Pattern};
use eval::{name::step, util::reducible};

/// Performs function application if possible, or reduces one of the arguments if not.
pub fn try_apply(func: Symbol, args: Vec<Expr<()>>, decls: &[Decl<()>]) -> Result<Expr<()>, Error> {
    for decl in decls.iter().filter(|decl| decl.name == func) {
        assert_eq!(args.len(), decl.args.len());
        if let Some(i) = args
            .iter()
            .zip(&decl.args)
            .position(|(a, p)| !arg_normal_enough_for_pat(a, p, decls))
        {
            let mut expr = Expr::Variable(func, ());
            for (n, arg) in args.into_iter().enumerate() {
                let arg = if n == i { step(arg, decls)? } else { arg };
                expr = Expr::Op(Op::App, Box::new(expr), Box::new(arg), ());
            }
            return Ok(expr);
        } else if let Some(bindings) = bind_args(&decl.args, &args) {
            return Ok(instantiate_body(&decl.body, &bindings));
        }
    }
    bail!("No matching clauses for call to {}", func)
}

fn arg_normal_enough_for_pat(value: &Expr<()>, pat: &Pattern<()>, decls: &[Decl<()>]) -> bool {
    match (value, pat) {
        (_, &Pattern::Binding(_, ())) => true,
        (&Expr::Literal(Literal::Int(_), ()), &Pattern::Literal(Literal::Int(_), ())) => true,
        (&Expr::Literal(Literal::Nil, ()), &Pattern::Literal(Literal::Nil, ())) => true,
        (&Expr::Op(Op::Cons, _, _, ()), &Pattern::Literal(Literal::Nil, ())) => true,
        (_, &Pattern::Literal(_, ())) => false,
        (&Expr::Op(Op::Cons, ref eh, ref et, ()), &Pattern::Cons(ref ph, ref pt, ())) => {
            arg_normal_enough_for_pat(eh, ph, decls) && arg_normal_enough_for_pat(et, pt, decls)
        }
        (e, &Pattern::Cons(_, _, ())) => {
            if !reducible(e, decls) {
                panic!("Looks like we're stuck; this should've been a type error earlier on?");
            }
            false
        }
    }
}

fn bind_arg(pattern: &Pattern<()>, expr: &Expr<()>) -> Option<BTreeMap<Symbol, Expr<()>>> {
    match (pattern, expr) {
        (&Pattern::Binding(var, ()), e) => {
            let mut map = BTreeMap::new();
            map.insert(var, e.clone());
            Some(map)
        }
        (&Pattern::Cons(ref pl, ref pr, ()), &Expr::Op(Op::Cons, ref el, ref er, ())) => {
            let mut map = bind_arg(pl, el)?;
            map.extend(bind_arg(pr, er)?);
            Some(map)
        }
        (&Pattern::Literal(l1, ()), &Expr::Literal(l2, ())) if l1 == l2 => Some(BTreeMap::new()),
        _ => None,
    }
}

fn bind_args(pats: &[Pattern<()>], args: &[Expr<()>]) -> Option<BTreeMap<Symbol, Expr<()>>> {
    let mut map = BTreeMap::new();
    for (pat, arg) in pats.iter().zip(args) {
        map.extend(bind_arg(pat, arg)?);
    }
    Some(map)
}

fn instantiate_body(expr: &Expr<()>, replacement: &BTreeMap<Symbol, Expr<()>>) -> Expr<()> {
    match *expr {
        Expr::If(ref c, ref t, ref e, ()) => Expr::If(
            Box::new(instantiate_body(c, replacement)),
            Box::new(instantiate_body(t, replacement)),
            Box::new(instantiate_body(e, replacement)),
            (),
        ),
        Expr::Literal(lit, ()) => Expr::Literal(lit, ()),
        Expr::Op(op, ref l, ref r, ()) => Expr::Op(
            op,
            Box::new(instantiate_body(l, replacement)),
            Box::new(instantiate_body(r, replacement)),
            (),
        ),
        Expr::Variable(ref var, ()) => {
            if let Some(expr) = replacement.get(var) {
                expr.clone()
            } else {
                Expr::Variable(*var, ())
            }
        }
    }
}
