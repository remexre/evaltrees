use std::collections::BTreeSet;

use ast::{Decl, Expr, Pattern};
use typeck::Ty;

/// A constraint, which holds two types equal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Constraint(Ty, Ty);

impl Decl<Ty> {
    /// Collects type constraints.
    fn collect_constraints(self) -> BTreeSet<Constraint> {
        /*
        Decl {
            name: self.name,
            args: self.args
                .into_iter()
                .map(|a| a.add_type_annotations())
                .collect(),
            body: self.body,
            aux: Ty::fresh(),
        }
        */
        unimplemented!()
    }
}

/*
impl Expr<Ty> {
    /// Converts the expression to one that has type annotations. All type
    /// annotations refer initially to fresh variables.
    fn add_type_annotations(self) -> BTreeSet<Constraint> {
        match self {
            Expr::Literal(l, ()) => Expr::Literal(l, Ty::fresh()),
            Expr::Op(op, l, r, ()) => Expr::Op(
                op,
                Box::new(l.add_type_annotations()),
                Box::new(r.add_type_annotations()),
                Ty::fresh(),
            ),
            Expr::Variable(n, ()) => Expr::Variable(n, Ty::fresh()),
        }
    }
}

impl Pattern<Ty> {
    /// Converts the pattern to one that has type annotations. All type
    /// annotations refer initially to fresh variables.
    fn add_type_annotations(self) -> BTreeSet<Constraint> {
        match self {
            Pattern::Binding(n, ()) => Pattern::Binding(n, Ty::fresh()),
            Pattern::Cons(l, r, ()) => Pattern::Cons(
                Box::new(l.add_type_annotations()),
                Box::new(r.add_type_annotations()),
                Ty::fresh(),
            ),
            Pattern::Literal(l, ()) => Pattern::Literal(l, Ty::fresh()),
        }
    }
}
*/
