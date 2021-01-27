use std::collections::BTreeSet;

use crate::typeck::subst::SubstVar;

/// A partial type, used in substitutions.
#[derive(Clone, Debug, DisplayAttr, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ty {
    /// The type of a boolean.
    #[display(fmt = "bool")]
    Bool,

    /// A function type.
    #[display(fmt = "({} -> {})", _0, _1)]
    Func(Box<Ty>, Box<Ty>),

    /// The type of an integer.
    #[display(fmt = "int")]
    Int,

    /// The type of a list.
    #[display(fmt = "({} list)", _0)]
    List(Box<Ty>),

    /// A substitution variable.
    #[display(fmt = "{}", _0)]
    Var(SubstVar),
}

impl Ty {
    /// Returns a fresh type variable.
    pub fn fresh() -> Ty {
        Ty::Var(SubstVar::fresh())
    }

    /// Returns the free variables of the type.
    pub fn freevars(&self) -> BTreeSet<SubstVar> {
        match *self {
            Ty::Bool | Ty::Int => BTreeSet::new(),
            Ty::Func(ref l, ref r) => l.freevars().into_iter().chain(r.freevars()).collect(),
            Ty::List(ref t) => t.freevars(),
            Ty::Var(v) => {
                let mut s = BTreeSet::new();
                s.insert(v);
                s
            }
        }
    }
}
