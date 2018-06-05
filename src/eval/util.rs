//! Helpers for implementing evaluators.

use ast::{Decl, Expr, Op, Type};

/// Returns whether the given expression is reducible, given the decls in scope.
pub fn reducible(expr: &Expr<Type>, decls: &[Decl<Type>]) -> bool {
    match *expr {
        Expr::If(_, _, _, _) => true,
        Expr::Literal(l, _) => false,
        Expr::Op(Op::App, ref l, ref r, _) => {
            reducible(l, decls) || reducible(r, decls) || unimplemented!()
        }
        Expr::Op(Op::Cons, ref l, ref r, _) => reducible(l, decls) || reducible(r, decls),
        Expr::Op(_, _, _, _) => false,
        Expr::Variable(_, _) => unimplemented!(),
    }
}

/// Returns a number corresponding to the beta-reducibility of the given function application. This
/// is necessary due to the fact that function applications occur all at once. For example:
///
/// `add 1 (2 + 3)` should reduce to
///
/// `add 1 5`, rather than the normal answer of
///
/// `(\x. 1 + x) (2 + 3)`.
///
/// Returns `None` if the given expression is not a function application or a reference to a
/// function. Panics if the expression refers to a decl not in scope.
pub fn beta_number(expr: &Expr<Type>, decls: &[Decl<Type>]) -> Option<usize> {
    match *expr {
        // A variable has a beta number equal to its arity.
        Expr::Variable(var, _) => {
            let decl = decls
                .iter()
                .find(|decl| decl.name == var)
                .unwrap_or_else(|| panic!("Unknown variable {}", var));
            Some(decl.args.len())
        }
        _ => None,
    }
}
