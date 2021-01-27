//! A Hindley-Milner type-checker (with inference).

mod annotations;
mod constraint;
mod reify;
mod subst;
#[cfg(test)]
mod tests;
mod ty;
mod util;

use std::collections::{BTreeSet, HashSet};

use failure::Fail;
use symbol::Symbol;

use crate::ast::{Decl, Type};
use crate::typeck::{
    constraint::Constraint,
    subst::{SubstVar, Substitution},
    ty::Ty,
    util::{group, scc_decls, AnnotEnv},
};

/// An error during typechecking.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum TypeError {
    /// A constraint between two types couldn't be unified.
    // TODO: collect type errors and continue to unify (incl. on errors) to be
    // able to display multiple, and display them better?
    #[fail(display = "Can't unify {} with {}", _0, _1)]
    CantUnify(Ty, Ty),

    /// A variable was undefined. This technically isn't a type error, but the error is only found
    /// during type-checking.
    #[fail(display = "Undefined variables: {:?}", _0)]
    Freevars(BTreeSet<Symbol>),

    /// The occurs check was failed (we've got an infinite type on our hands!).
    #[fail(display = "{} occurs within {} when solving {} ~ {}", _0, _1, _0, _1)]
    Occurs(SubstVar, Ty),
}

/// Completely type-checks a series of declarations.
///
/// We take a list of declarations to be checked, and a list of already-typed declarations. For a
/// declaration to be treated as polymorphic, it must already have been typechecked, and assigned a
/// polymorphic type.
pub fn typeck(
    decls: Vec<Decl<()>>,
    mut checked: Vec<Decl<Type>>,
) -> Result<Vec<Decl<Type>>, TypeError> {
    // Check for free variables, erroring out if any are found.
    let mut freevars = decls
        .iter()
        .flat_map(|decl| decl.freevars())
        .collect::<BTreeSet<_>>();
    for decl in &decls {
        freevars.remove(&decl.name);
    }
    for decl in &checked {
        freevars.remove(&decl.name);
    }
    if !freevars.is_empty() {
        return Err(TypeError::Freevars(freevars));
    }

    // Split the decls into strongly connected components.
    let known = checked.iter().map(|decl| decl.name).collect::<HashSet<_>>();
    let decls = group(decls, |decl| decl.name);
    let decls = scc_decls(decls, &known);

    // Check each "level" of the decls.
    for decl in decls {
        let decl = typeck_decls_with(decl, &checked)?;
        checked.extend(decl);
    }
    Ok(checked)
}

/// Type-checks a series of decls that are at the same "stratification level," i.e. that form a
/// strongly connected component.
fn typeck_decls_with(
    decls: Vec<Decl<()>>,
    checked: &[Decl<Type>],
) -> Result<Vec<Decl<Type>>, TypeError> {
    // We assume later on that at least one decl exists.
    if decls.is_empty() {
        return Ok(Vec::new());
    }

    // First, create an environment containing the already-checked decls.
    let mut env = AnnotEnv::new();
    for decl in checked {
        env.put_poly(decl.name, decl.aux());
    }

    // Add names for the decls in the component.
    let names = decls.iter().map(|decl| decl.name).collect::<HashSet<_>>();
    for name in names {
        env.put(name, Ty::fresh());
    }

    // Use the annotation environment to annotate the decls.
    let mut decls = decls
        .into_iter()
        .map(|decl| decl.add_type_annotations(&mut env.clone()))
        .collect::<Vec<_>>();

    // Next, collect the type constraints and unify them into a substitution.
    let constraints = decls
        .iter()
        .flat_map(|decl| decl.collect_constraints())
        .collect();
    log::debug!("decls = {:?}", decls);
    log::debug!("constraints = {:?}", constraints);
    let subst = unify(constraints)?;

    // Then, apply the substitution across the AST.
    for decl in &mut decls {
        decl.apply_subst(&subst);
    }

    // Finally, reify the types across the AST.
    let decls = decls.into_iter().map(|decl| decl.reify()).collect();
    Ok(decls)
}

/// Generates a substitution from a set of constraints.
fn unify(constraints: BTreeSet<Constraint>) -> Result<Substitution, TypeError> {
    // We go BTreeSet->Vec instead of working with Vecs all the way through to ensure uniqueness of
    // constraints, and because it feels semantically closer to what we want anyway. The Vec is
    // only here because there's no .remove_arbitrary() operation on sets.
    let mut constraints = constraints.into_iter().collect::<Vec<_>>();

    let mut subst = Substitution::new();
    while let Some(Constraint(s, t)) = constraints.pop() {
        log::debug!("Applying constraint {} ~ {}...", s, t);
        if s == t {
            // Yay, nothing to do; the constraint is e.g. Int ~ Int.
        } else {
            match (s, t) {
                (Ty::Var(x), t) => {
                    if t.freevars().contains(&x) {
                        return Err(TypeError::Occurs(x, t));
                    }
                    for Constraint(cs, ct) in &mut constraints {
                        cs.sub(x, &t);
                        ct.sub(x, &t);
                    }
                    subst.add(x, t);
                }
                (s, Ty::Var(x)) => {
                    if s.freevars().contains(&x) {
                        return Err(TypeError::Occurs(x, s));
                    }
                    for Constraint(cs, ct) in &mut constraints {
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
