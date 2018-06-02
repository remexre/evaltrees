use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::once;

use ast::{Decl, Expr, Literal, Op, Pattern};
use typeck::ty::Ty;

/// A constraint, which holds two types equal.
///
/// Eq and Ord here are opaque to flipping; i.e. if
/// `Constraint(a, b) > Constraint(c, d)`, then
/// `Constraint(b, a) > Constraint(c, d)` as well.
#[derive(Clone, Debug, Eq)]
pub struct Constraint(pub Ty, pub Ty);

impl PartialEq for Constraint {
    fn eq(&self, other: &Constraint) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Ord for Constraint {
    fn cmp(&self, other: &Constraint) -> Ordering {
        let l = if self.0 <= self.1 {
            (&self.0, &self.1)
        } else {
            (&self.1, &self.0)
        };
        let r = if other.0 <= other.1 {
            (&other.0, &other.1)
        } else {
            (&other.1, &other.0)
        };
        l.cmp(&r)
    }
}

impl PartialOrd for Constraint {
    fn partial_cmp(&self, other: &Constraint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Decl<Ty> {
    /// Collects type constraints.
    pub(in typeck) fn collect_constraints(&self) -> BTreeSet<Constraint> {
        let mut ty = self.body.aux();
        for arg in self.args.iter().rev() {
            ty = Ty::Func(Box::new(arg.aux()), Box::new(ty));
        }

        self.args
            .iter()
            .flat_map(|arg| arg.collect_constraints())
            .chain(self.body.collect_constraints())
            .chain(once(Constraint(self.aux.clone(), ty)))
            .collect()
    }
}

impl Expr<Ty> {
    /// Collects type constraints.
    pub(in typeck) fn collect_constraints(&self) -> BTreeSet<Constraint> {
        match *self {
            Expr::Literal(lit, ref ty) => once(Constraint(ty.clone(), lit.ty())).collect(),
            Expr::Op(Op::App, ref l, ref r, ref ty) => once(Constraint(
                l.aux(),
                Ty::Func(Box::new(r.aux()), Box::new(ty.clone())),
            )).chain(l.collect_constraints())
                .chain(r.collect_constraints())
                .collect(),
            Expr::Op(Op::Cons, ref l, ref r, ref ty) => vec![
                Constraint(r.aux(), ty.clone()),
                Constraint(r.aux(), Ty::List(Box::new(l.aux()))),
            ].into_iter()
                .chain(l.collect_constraints())
                .chain(r.collect_constraints())
                .collect(),
            // All other ops are int->int->int
            Expr::Op(_, ref l, ref r, ref ty) => vec![
                Constraint(l.aux(), Ty::Int),
                Constraint(r.aux(), Ty::Int),
                Constraint(ty.clone(), Ty::Int),
            ].into_iter()
                .chain(l.collect_constraints())
                .chain(r.collect_constraints())
                .collect(),
            Expr::Variable(_, _) => BTreeSet::new(),
        }
    }
}

impl Literal {
    /// Returns the type of the literal.
    fn ty(&self) -> Ty {
        match *self {
            Literal::Int(_) => Ty::Int,
            Literal::Nil => Ty::List(Box::new(Ty::fresh())),
        }
    }
}

impl Pattern<Ty> {
    /// Collects type constraints.
    fn collect_constraints(&self) -> BTreeSet<Constraint> {
        let mut constraints = BTreeSet::new();
        match *self {
            Pattern::Binding(_, _) => {}
            Pattern::Cons(ref l, ref r, ref ty) => {
                constraints.insert(Constraint(ty.clone(), r.aux()));
                constraints.insert(Constraint(ty.clone(), Ty::List(Box::new(l.aux()))));
                constraints.extend(l.collect_constraints());
                constraints.extend(r.collect_constraints());
            }
            Pattern::Literal(lit, ref ty) => {
                constraints.insert(Constraint(ty.clone(), lit.ty()));
            }
        }
        constraints
    }
}
