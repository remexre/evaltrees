//! The abstract syntax tree.

mod convert;
mod display;

use std::collections::BTreeSet;

use symbol::Symbol;

/// A function or value declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Decl<Aux> {
    /// The name of the function or value.
    pub name: Symbol,

    /// The arguments to the function. If empty, the decl is for a value.
    pub args: Vec<Pattern<Aux>>,

    /// The body of the function, or the expression assigned to the value.
    pub body: Expr<Aux>,

    /// Auxiliary data.
    pub aux: Aux,
}

impl<Aux> Decl<Aux> {
    /// Gets the auxiliary data as a reference.
    pub fn aux_ref(&self) -> &Aux {
        &self.aux
    }

    /// Returns the free variables of a declaration.
    pub fn freevars(&self) -> BTreeSet<Symbol> {
        let mut vars = self.body.freevars();
        for arg in &self.args {
            for var in arg.freevars() {
                vars.remove(&var);
            }
        }
        vars.remove(&self.name);
        vars
    }
}

impl<Aux: Clone> Decl<Aux> {
    /// Clones the auxiliary data out.
    pub fn aux(&self) -> Aux {
        self.aux_ref().clone()
    }
}

/// A pattern.
#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<Aux> {
    /// A name.
    Binding(Symbol, Aux),

    /// A cons.
    Cons(Box<Pattern<Aux>>, Box<Pattern<Aux>>, Aux),

    /// A literal value.
    Literal(Literal, Aux),
}

impl<Aux> Pattern<Aux> {
    /// Gets the auxiliary data as a reference.
    pub fn aux_ref(&self) -> &Aux {
        match *self {
            Pattern::Binding(_, ref aux)
            | Pattern::Cons(_, _, ref aux)
            | Pattern::Literal(_, ref aux) => aux,
        }
    }

    /// Returns the bound variables of a pattern.
    pub fn freevars(&self) -> BTreeSet<Symbol> {
        match *self {
            Pattern::Binding(var, _) => {
                let mut set = BTreeSet::new();
                set.insert(var);
                set
            }
            Pattern::Cons(ref l, ref r, _) => {
                l.freevars().into_iter().chain(r.freevars()).collect()
            }
            Pattern::Literal(_, _) => BTreeSet::new(),
        }
    }
}

impl<Aux: Clone> Pattern<Aux> {
    /// Clones the auxiliary data out.
    pub fn aux(&self) -> Aux {
        self.aux_ref().clone()
    }
}

/// An expression.
#[derive(Clone, Debug, DisplayAttr, PartialEq)]
pub enum Expr<Aux> {
    /// A conditional expression.
    #[display(fmt = "If({}, {}, {})", _0, _1, _2)]
    If(Box<Expr<Aux>>, Box<Expr<Aux>>, Box<Expr<Aux>>, Aux),

    /// A literal value.
    #[display(fmt = "{}", _0)]
    Literal(Literal, Aux),

    /// A binary operator.
    #[display(fmt = "{}({}, {})", _0, _1, _2)]
    Op(Op, Box<Expr<Aux>>, Box<Expr<Aux>>, Aux),

    /// A variable.
    #[display(fmt = "{}", _0)]
    Variable(Symbol, Aux),
}

impl<Aux> Expr<Aux> {
    /// Gets the auxiliary data as a reference.
    pub fn aux_ref(&self) -> &Aux {
        match *self {
            Expr::If(_, _, _, ref aux)
            | Expr::Literal(_, ref aux)
            | Expr::Op(_, _, _, ref aux)
            | Expr::Variable(_, ref aux) => aux,
        }
    }

    /// Returns the free variables of an expression.
    pub fn freevars(&self) -> BTreeSet<Symbol> {
        match *self {
            Expr::If(ref c, ref t, ref e, _) => c.freevars()
                .into_iter()
                .chain(t.freevars())
                .chain(e.freevars())
                .collect(),
            Expr::Literal(_, _) => BTreeSet::new(),
            Expr::Op(_, ref l, ref r, _) => l.freevars().into_iter().chain(r.freevars()).collect(),
            Expr::Variable(var, _) => {
                let mut set = BTreeSet::new();
                set.insert(var);
                set
            }
        }
    }
}

impl<Aux: Clone> Expr<Aux> {
    /// Clones the auxiliary data out.
    pub fn aux(&self) -> Aux {
        self.aux_ref().clone()
    }
}

/// A binary operator.
#[derive(Clone, Copy, Debug, DisplayAttr, PartialEq)]
#[allow(missing_docs)]
pub enum Op {
    /// Addition.
    #[display(fmt = "Add")]
    Add,

    /// Function application.
    #[display(fmt = "App")]
    App,

    /// List construction.
    #[display(fmt = "Cons")]
    Cons,

    /// Division.
    #[display(fmt = "Div")]
    Div,

    /// Modulus.
    #[display(fmt = "Mod")]
    Mod,

    /// Multiplication.
    #[display(fmt = "Mul")]
    Mul,

    /// Subtraction.
    #[display(fmt = "Sub")]
    Sub,
}

/// A literal value.
#[derive(Clone, Copy, Debug, DisplayAttr, PartialEq)]
pub enum Literal {
    /// The false boolean value.
    #[display(fmt = "false")]
    False,

    /// An (unsigned) integer.
    #[display(fmt = "{}", _0)]
    Int(usize),

    /// An empty list.
    #[display(fmt = "[]")]
    Nil,

    /// The true boolean value.
    #[display(fmt = "true")]
    True,
}

/// A (fully formed) type.
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// The boolean type.
    Bool,

    /// Universal quantification over a variable.
    ///
    /// De Brujin indices are used here, so no explicit names are needed.
    Forall(Box<Type>),

    /// A function type.
    Func(Box<Type>, Box<Type>),

    /// The unsigned integer type.
    Int,

    /// A list type.
    List(Box<Type>),

    /// A type variable.
    Var(usize),
}

impl Type {
    /// Returns the "number of arguments" the type takes.
    pub fn argn(&self) -> usize {
        match *self {
            Type::Bool | Type::Int | Type::List(_) | Type::Var(_) => 0,
            Type::Forall(ref ty) => ty.argn(),
            Type::Func(_, ref r) => 1 + r.argn(),
        }
    }
}
