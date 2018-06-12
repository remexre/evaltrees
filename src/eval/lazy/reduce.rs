//! The functions here are clones of the ones in eval::util, but adapted for LazyExpr.

use ast::{Decl, Op};
use eval::lazy::expr::LazyExpr;

pub fn reducible<Aux>(expr: &LazyExpr<Aux>, decls: &[Decl<Aux>]) -> bool {
    match *expr {
        LazyExpr::If(_, _, _, _) => true,
        LazyExpr::Literal(_, _) => false,
        LazyExpr::Op(Op::App, ref l, ref r, _) => {
            reducible(l, decls) || reducible(r, decls)
                || beta_number(expr, decls).map(|n| n == 0).unwrap_or(false)
        }
        LazyExpr::Op(Op::Cons, ref l, ref r, _) => reducible(l, decls) || reducible(r, decls),
        LazyExpr::Op(_, _, _, _) => true,
        LazyExpr::Variable(var, _) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            decl.args.is_empty()
        }
        LazyExpr::WhereVar(_, _) => true,
    }
}

pub fn beta_number<Aux>(expr: &LazyExpr<Aux>, decls: &[Decl<Aux>]) -> Option<isize> {
    match *expr {
        // An application whose arguments are in normal form has a beta number equal to the beta
        // number of its left argument minus one.
        LazyExpr::Op(Op::App, ref l, _, _) => beta_number(l, decls).map(|n| n - 1),

        // A variable has a beta number equal to its arity.
        LazyExpr::Variable(var, _) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            Some(decl.args.len() as isize)
        }
        LazyExpr::WhereVar(_, _) => Some(0),

        // Other expressions don't have a beta number.
        _ => None,
    }
}
