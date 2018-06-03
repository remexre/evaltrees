//! A Hindley-Milner type-checker (with inference).

mod annotations;
mod constraint;
mod reify;
mod subst;
#[cfg(test)]
mod tests;
mod ty;
mod util;

use std::collections::BTreeSet;

use ast::{Decl, Expr, Type};
use typeck::{annotations::add_annotations_to_decls,
             constraint::Constraint,
             subst::{SubstVar, Substitution},
             ty::Ty,
             util::AnnotEnv};

/// An error during typechecking.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum TypeError {
    /// A constraint between two types couldn't be unified.
    // TODO: collect type errors and continue to unify (incl. on errors) to be
    // able to display multiple, and display them better?
    #[fail(display = "Can't unify {} with {}", _0, _1)]
    CantUnify(Ty, Ty),

    /// The occurs check was failed (we've got an infinite type on our hands!).
    #[fail(display = "{} occurs within {}", _0, _1)]
    Occurs(SubstVar, Ty),
}

/// Completely type-checks a series of declarations.
pub fn typeck_decls(decls: Vec<Decl<()>>) -> Result<Vec<Decl<Type>>, TypeError> {
    // First, annotate the decls with type variables.
    let mut decls = add_annotations_to_decls(decls);

    // Next, collect the type constraints and unify them into a substitution.
    let constraints = decls
        .iter()
        .flat_map(|decl| decl.collect_constraints())
        .collect();
    let subst = unify(constraints)?;

    // Then, apply the substitution across the AST.
    for decl in &mut decls {
        decl.apply_subst(&subst);
    }

    // Finally, reify the types across the AST.
    let decls = decls.into_iter().map(|decl| decl.reify()).collect();
    Ok(decls)
}

/// Type-checks an expression given the declarations that are in scope.
pub fn typeck_expr(expr: Expr<()>, decls: &[Decl<Type>]) -> Result<Expr<Type>, TypeError> {
    let mut env = AnnotEnv::new();
    for decl in decls {
        env.put(decl.name, decl.aux().unreify());
    }

    let mut expr = expr.add_type_annotations(&mut env);
    let constraints = expr.collect_constraints();
    let subst = unify(constraints)?;
    expr.apply_subst(&subst);
    Ok(expr.reify())
}

/// Generates a substitution from a set of constraints.
///
/// Note that no occurs check is currently implemented.
fn unify(constraints: BTreeSet<Constraint>) -> Result<Substitution, TypeError> {
    // We go BTreeSet->Vec instead of working with Vecs all the way through to
    // ensure uniqueness, and because it feels semantically closer to what we
    // want anyway. The Vec is only here because there's no .remove_arbitrary()
    // operation on sets.
    let mut constraints = constraints.into_iter().collect::<Vec<_>>();

    let mut subst = Substitution::new();
    while let Some(Constraint(s, t)) = constraints.pop() {
        trace!("Applying constraint {} ~ {}...", s, t);
        if s == t {
            // Yay, nothing to do.
        } else {
            match (s, t) {
                (Ty::Var(x), t) => {
                    if t.freevars().contains(&x) {
                        return Err(TypeError::Occurs(x, t));
                    }
                    for &mut Constraint(ref mut cs, ref mut ct) in &mut constraints {
                        cs.sub(x, &t);
                        ct.sub(x, &t);
                    }
                    subst.add(x, t);
                }
                (s, Ty::Var(x)) => {
                    if s.freevars().contains(&x) {
                        return Err(TypeError::Occurs(x, s));
                    }
                    for &mut Constraint(ref mut cs, ref mut ct) in &mut constraints {
                        cs.sub(x, &s);
                        ct.sub(x, &s);
                    }
                    subst.add(x, s);
                }
                (Ty::Func(s1, s2), Ty::Func(t1, t2)) => {
                    constraints.push(Constraint(*s1, *t1));
                    constraints.push(Constraint(*s2, *t2));
                }
                (Ty::List(s), Ty::List(t)) => {
                    constraints.push(Constraint(*s, *t));
                }
                (s, t) => {
                    return Err(TypeError::CantUnify(s, t));
                }
            }
        }
    }
    Ok(subst)
}
