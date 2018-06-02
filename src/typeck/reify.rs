use linked_hash_set::LinkedHashSet;

use ast::{Decl, Expr, Pattern, Type};
use typeck::{subst::SubstVar, ty::Ty};

impl Decl<Ty> {
    /// Reifies a declaration.
    pub fn reify(self) -> Decl<Type> {
        let mut vars = LinkedHashSet::new();
        for arg in &self.args {
            arg.collect_vars(&mut vars);
        }

        let mut env = vars.into_iter().collect::<Vec<_>>();
        env.reverse();

        let mut ty = self.aux.reify(&env);
        for _ in &env {
            ty = Type::Forall(Box::new(ty));
        }

        let args = self.args.into_iter().map(|arg| arg.reify(&env)).collect();
        let body = self.body.reify(&env);
        Decl {
            name: self.name,
            args,
            body,
            aux: ty,
        }
    }
}

impl Expr<Ty> {
    fn reify(self, env: &[SubstVar]) -> Expr<Type> {
        match self {
            Expr::Literal(l, ty) => Expr::Literal(l, ty.reify(env)),
            Expr::Op(o, l, r, ty) => Expr::Op(
                o,
                Box::new(l.reify(env)),
                Box::new(r.reify(env)),
                ty.reify(env),
            ),
            Expr::Variable(v, ty) => Expr::Variable(v, ty.reify(env)),
        }
    }
}

impl Pattern<Ty> {
    fn collect_vars(&self, vars: &mut LinkedHashSet<SubstVar>) {
        match *self {
            Pattern::Cons(ref h, ref t, ref ty) => {
                if let Ty::Var(var) = *ty {
                    vars.insert(var);
                }
                h.collect_vars(vars);
                t.collect_vars(vars);
            }
            Pattern::Binding(_, ref ty) | Pattern::Literal(_, ref ty) => {
                if let Ty::Var(var) = *ty {
                    vars.insert(var);
                }
            }
        }
    }

    fn reify(self, env: &[SubstVar]) -> Pattern<Type> {
        match self {
            Pattern::Binding(n, ty) => Pattern::Binding(n, ty.reify(env)),
            Pattern::Cons(h, t, ty) => Pattern::Cons(
                Box::new(h.reify(env)),
                Box::new(t.reify(env)),
                ty.reify(env),
            ),
            Pattern::Literal(l, ty) => Pattern::Literal(l, ty.reify(env)),
        }
    }
}

impl Ty {
    fn reify(self, env: &[SubstVar]) -> Type {
        match self {
            Ty::Func(l, r) => Type::Func(Box::new(l.reify(env)), Box::new(r.reify(env))),
            Ty::Int => Type::Int,
            Ty::List(t) => Type::List(Box::new(t.reify(env))),
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
