use std::rc::Rc;

use symbol::Symbol;

use typeck::Ty;

/// An environment that allows for the insertion of "barriers."
#[derive(Clone, Debug)]
pub struct Env {
    inner: Option<Rc<EnvInner>>,
}

#[derive(Clone, Debug)]
pub enum EnvInner {
    Barrier(Rc<EnvInner>),
    Binding(Symbol, Ty, Rc<EnvInner>),
    Nil,
}

impl Env {
    /// Gets the type associated with a name. Ignores barriers.
    pub fn get(&self, name: Symbol) -> Option<Ty> {
        unimplemented!()
    }

    /// Gets the type associated by the name, unless a barrier appears first.
    pub fn get_local(&self, name: Symbol) -> Option<Ty> {
        unimplemented!()
    }

    /// Creates a barrier.
    pub fn barrier(&self) -> Env {
        Env {
            inner: Some(Rc::new(EnvInner::Barrier(self.inner.clone().unwrap()))),
        }
    }

    /// Creates a new binding.
    pub fn put(&self, name: Symbol, ty: Ty) -> Env {
        unimplemented!()
    }

    /// Returns the value associated with a name, unless it does not exist
    /// before the end of the environment list is reached. In that case, a
    /// fresh type variable is created.
    pub fn get_or_fresh(&self, name: Symbol) -> Ty {
        unimplemented!()
    }

    /// Returns the value associated with a name, unless it does not exist
    /// before the first barrier is hit or the end of the environment list is
    /// reached. In that case, a fresh type variable is created.
    pub fn get_local_or_fresh(&self, name: Symbol) -> Ty {
        unimplemented!()
    }
}
