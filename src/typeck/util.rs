use std::rc::Rc;

use symbol::Symbol;

use typeck::Ty;

/// The lexical environment, used when assigning type variables.
///
/// Supports O(1) cloning.
#[derive(Clone, Debug)]
pub struct Env {
    inner: Option<Rc<EnvInner>>,
}

#[derive(Clone, Debug)]
pub enum EnvInner {
    Cons(Symbol, Ty, Rc<EnvInner>),
    Nil,
}

impl Env {
    /// Returns the value associated with a name, unless it does not exist.
    /// In that case, a fresh type variable is created.
    pub fn get(&self, name: Symbol) -> (Env, Ty) {
        unimplemented!()
    }

    /// Creates a new Env.
    pub fn new() -> Env {
        unimplemented!()
    }

    /// Creates a new binding.
    pub fn put(&self, name: Symbol, ty: Ty) -> Env {
        unimplemented!()
    }
}
