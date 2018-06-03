use std::collections::BTreeSet;

use typeck::subst::SubstVar;

/// A partial type, used in substitutions.
#[derive(Clone, Debug, DisplayAttr, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ty {
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
            Ty::Func(ref l, ref r) => l.freevars().into_iter().chain(r.freevars()).collect(),
            Ty::Int => BTreeSet::new(),
            Ty::List(ref t) => t.freevars(),
            Ty::Var(v) => {
                let mut s = BTreeSet::new();
                s.insert(v);
                s
            }
        }
    }
}
