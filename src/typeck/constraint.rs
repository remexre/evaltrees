use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::once;

use ast::{Decl, Expr, Literal, Op, Pattern};
use typeck::ty::Ty;

/// A constraint, which holds two types equal.
#[derive(Clone, Debug, Eq)]
pub struct Constraint(Ty, Ty);

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
    pub fn collect_constraints(&self) -> BTreeSet<Constraint> {
        let mut ty = self.body.ty();
        for arg in self.args.iter().rev() {
            ty = Ty::Func(Box::new(arg.ty()), Box::new(ty));
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
    pub fn collect_constraints(&self) -> BTreeSet<Constraint> {
        match *self {
            Expr::Literal(lit, ref ty) => once(Constraint(ty.clone(), lit.ty())).collect(),
            Expr::Op(Op::App, ref l, ref r, ref ty) => once(Constraint(
                l.ty(),
                Ty::Func(Box::new(r.ty()), Box::new(ty.clone())),
            )).collect(),
            Expr::Op(Op::Cons, ref l, ref r, ref ty) => vec![
                Constraint(r.ty(), ty.clone()),
                Constraint(r.ty(), Ty::List(Box::new(l.ty()))),
            ].into_iter()
                .collect(),
            // All other ops are int->int->int
            Expr::Op(_, ref l, ref r, ref ty) => vec![
                Constraint(l.ty(), Ty::Int),
                Constraint(r.ty(), Ty::Int),
                Constraint(ty.clone(), Ty::Int),
            ].into_iter()
                .collect(),
            Expr::Variable(_, _) => BTreeSet::new(),
        }
    }

    /// Returns the type stored in the expression.
    fn ty(&self) -> Ty {
        match *self {
            Expr::Literal(_, ref ty) => ty.clone(),
            Expr::Op(_, _, _, ref ty) => ty.clone(),
            Expr::Variable(_, ref ty) => ty.clone(),
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
    pub fn collect_constraints(&self) -> BTreeSet<Constraint> {
        match *self {
            Pattern::Binding(_, _) => BTreeSet::new(),
            Pattern::Cons(_, _, ref ty) => unimplemented!(),
            Pattern::Literal(lit, ref ty) => once(Constraint(ty.clone(), lit.ty())).collect(),
        }
    }

    /// Returns the type stored in the pattern.
    fn ty(&self) -> Ty {
        match *self {
            Pattern::Binding(_, ref ty) => ty.clone(),
            Pattern::Cons(_, _, ref ty) => ty.clone(),
            Pattern::Literal(_, ref ty) => ty.clone(),
        }
    }
}
