use linked_hash_set::LinkedHashSet;

use ast::{Decl, Expr, Pattern, Type};
use typeck::{subst::SubstVar, ty::Ty, util::UnreifyEnv};

// TODO: This reification is unsound in the presence of higher rank polymorphism.

impl Decl<Ty> {
    /// Reifies a declaration.
    pub(in typeck) fn reify(self) -> Decl<Type> {
        let mut vars = LinkedHashSet::new();
        for arg in &self.args {
            arg.collect_vars(&mut vars);
        }
        self.body.collect_vars(&mut vars);

        let env = vars.into_iter().rev().collect::<Vec<_>>();
        let mut ty = self.aux.reify_in(&env);
        for _ in &env {
            ty = Type::Forall(Box::new(ty));
        }

        let args = self.args
            .into_iter()
            .map(|arg| arg.reify_in(&env))
            .collect();
        let body = self.body.reify_in(&env);
        Decl {
            name: self.name,
            args,
            body,
            aux: ty,
        }
    }
}

impl Expr<Ty> {
    fn collect_vars(&self, vars: &mut LinkedHashSet<SubstVar>) {
        match *self {
            Expr::If(ref c, ref t, ref e, ref ty) => {
                c.collect_vars(vars);
                t.collect_vars(vars);
                e.collect_vars(vars);
                ty.collect_vars(vars);
            }
            Expr::Literal(_, ref ty) | Expr::Variable(_, ref ty) => {
                ty.collect_vars(vars);
            }
            Expr::Op(_, ref l, ref r, ref ty) => {
                l.collect_vars(vars);
                r.collect_vars(vars);
                ty.collect_vars(vars);
            }
        }
    }

    fn reify_in(self, env: &[SubstVar]) -> Expr<Type> {
        match self {
            Expr::If(c, t, e, ty) => Expr::If(
                Box::new(c.reify_in(env)),
                Box::new(t.reify_in(env)),
                Box::new(e.reify_in(env)),
                ty.reify_in(env),
            ),
            Expr::Literal(l, ty) => Expr::Literal(l, ty.reify_in(env)),
            Expr::Op(o, l, r, ty) => Expr::Op(
                o,
                Box::new(l.reify_in(env)),
                Box::new(r.reify_in(env)),
                ty.reify_in(env),
            ),
            Expr::Variable(v, ty) => Expr::Variable(v, ty.reify_in(env)),
        }
    }
}

impl Pattern<Ty> {
    fn collect_vars(&self, vars: &mut LinkedHashSet<SubstVar>) {
        match *self {
            Pattern::Binding(_, ref ty) | Pattern::Literal(_, ref ty) => {
                ty.collect_vars(vars);
            }
            Pattern::Cons(ref h, ref t, ref ty) => {
                h.collect_vars(vars);
                t.collect_vars(vars);
                ty.collect_vars(vars);
            }
        }
    }

    fn reify_in(self, env: &[SubstVar]) -> Pattern<Type> {
        match self {
            Pattern::Binding(n, ty) => Pattern::Binding(n, ty.reify_in(env)),
            Pattern::Cons(h, t, ty) => Pattern::Cons(
                Box::new(h.reify_in(env)),
                Box::new(t.reify_in(env)),
                ty.reify_in(env),
            ),
            Pattern::Literal(l, ty) => Pattern::Literal(l, ty.reify_in(env)),
        }
    }
}

impl Ty {
    fn collect_vars(&self, vars: &mut LinkedHashSet<SubstVar>) {
        match *self {
            Ty::Func(ref l, ref r) => {
                l.collect_vars(vars);
                r.collect_vars(vars);
            }
            Ty::Bool | Ty::Int => {}
            Ty::List(ref t) => t.collect_vars(vars),
            Ty::Var(v) => {
                vars.insert(v);
            }
        }
    }

    fn reify_in(self, env: &[SubstVar]) -> Type {
        match self {
            Ty::Bool => Type::Bool,
            Ty::Func(l, r) => Type::Func(Box::new(l.reify_in(env)), Box::new(r.reify_in(env))),
            Ty::Int => Type::Int,
            Ty::List(t) => Type::List(Box::new(t.reify_in(env))),
            Ty::Var(v) => {
                for (i, &v2) in env.iter().enumerate() {
                    if v == v2 {
                        return Type::Var(i);
                    }
                }
                panic!("Error: unknown var {:?} in env {:?}", v, env)
            }
        }
    }
}

impl Type {
    pub(in typeck) fn unreify(&self) -> Ty {
        fn helper(mut ty: &Type, mut env: UnreifyEnv) -> Ty {
            // First, peel off any Foralls.
            while let Type::Forall(ref t) = *ty {
                env.push(SubstVar::fresh());
                ty = t;
            }

            match *ty {
                Type::Bool => Ty::Bool,
                Type::Forall(_) => unreachable!(),
                Type::Func(ref l, ref r) => {
                    let l = helper(l, env.clone());
                    let r = helper(r, env);
                    Ty::Func(Box::new(l), Box::new(r))
                }
                Type::Int => Ty::Int,
                Type::List(ref t) => Ty::List(Box::new(helper(t, env))),
                Type::Var(n) => Ty::Var(env.get(n)),
            }
        }
        helper(self, UnreifyEnv::new())
    }
}
