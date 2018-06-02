use typeck::subst::SubstVar;

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
    /// Returns a fresh type variable.
    pub fn fresh() -> Ty {
        Ty::Var(SubstVar::gensym())
    }
}
