//! Helpers for implementing evaluators.

use std::collections::BTreeSet;

use ast::{Decl, Expr, Op};

/// Returns whether the given expression is reducible, given the decls in scope.
pub fn reducible<Aux>(expr: &Expr<Aux>, decls: &[Decl<Aux>]) -> bool {
    match *expr {
        Expr::If(_, _, _, _) => true,
        Expr::Literal(_, _) => false,
        Expr::Op(Op::App, ref l, ref r, _) => {
            reducible(l, decls)
                || reducible(r, decls)
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
        // An application whose arguments are in normal form has a beta number equal to the beta
        // number of its left argument minus one.
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

/// Computes the transitive closure,
pub fn transitive_closure<F, I1, I2, T>(initial: I1, op: F) -> BTreeSet<T>
where
    F: Fn(&T) -> I2,
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    T: Clone + Ord,
{
    let mut set = initial.into_iter().collect::<BTreeSet<T>>();
    let mut prev = BTreeSet::new();
    while set != prev {
        prev = set.clone();
        set.extend(prev.iter().flat_map(&op));
    }
    set
}
