//! The functions here are clones of the ones in eval::util, but adapted for LazyExpr.

use crate::ast::{Decl, Op};
use crate::eval::lazy::expr::LazyExpr;

pub fn reducible(expr: &LazyExpr, decls: &[Decl<()>]) -> bool {
    match *expr {
        LazyExpr::If(_, _, _) => true,
        LazyExpr::Literal(_) => false,
        LazyExpr::Op(Op::App, ref l, ref r) => {
            reducible(l, decls)
                || reducible(r, decls)
                || beta_number(expr, decls).map(|n| n == 0).unwrap_or(false)
        }
        LazyExpr::Op(Op::Cons, ref l, ref r) => reducible(l, decls) || reducible(r, decls),
        LazyExpr::Op(_, _, _) => true,
        LazyExpr::Variable(var) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            decl.args.is_empty()
        }
        LazyExpr::WhereVar(_) => true,
    }
}

pub fn beta_number(expr: &LazyExpr, decls: &[Decl<()>]) -> Option<isize> {
    match *expr {
        // An application whose arguments are in normal form has a beta number equal to the beta
        // number of its left argument minus one.
        LazyExpr::Op(Op::App, ref l, _) => beta_number(l, decls).map(|n| n - 1),

        // A variable has a beta number equal to its arity.
        LazyExpr::Variable(var) => {
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
