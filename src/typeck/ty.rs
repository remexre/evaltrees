use typeck::subst::{SubstVar, Substitution};

/// A partial type, used in substitutions.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ty {
    /// A function type.
    Func(Box<Ty>, Box<Ty>),

    /// The type of an integer.
    Int,

    /// The type of a list.
    List(Box<Ty>),

    /// A substitution variable.
    Var(SubstVar),
}

impl Ty {
    /// Applies a substitution to the type.
    pub fn apply_subst(self, subst: &Substitution) -> Ty {
        match self {
            Ty::Func(l, r) => Ty::Func(
                Box::new(l.apply_subst(subst)),
                Box::new(r.apply_subst(subst)),
            ),
            Ty::Int => Ty::Int,
            Ty::List(t) => Ty::List(Box::new(t.apply_subst(subst))),
            Ty::Var(v) => if let Some(ty) = subst.lookup(v) {
                ty
            } else {
                Ty::Var(v)
            },
        }
    }

    /// Returns a fresh type variable.
    pub fn fresh() -> Ty {
        Ty::Var(SubstVar::gensym())
    }
}
