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

use symbol::Symbol;

use ast::{Decl, Type};
use typeck::{constraint::Constraint,
             subst::{SubstVar, Substitution},
             ty::Ty,
             util::{group, toposort, AnnotEnv}};

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

    /// A mutually-recursive declaration was found. We don't currently support these.
    #[fail(display = "Mutual recursion involving {}", _0)]
    MutualRecursion(Symbol),

    /// The occurs check was failed (we've got an infinite type on our hands!).
    #[fail(display = "{} occurs within {} when solving {} ~ {}", _0, _1, _0, _1)]
    Occurs(SubstVar, Ty),
}

/// Completely type-checks a series of declarations.
pub fn typeck(
    decls: Vec<Decl<()>>,
    mut checked: Vec<Decl<Type>>,
) -> Result<Vec<Decl<Type>>, TypeError> {
    // Check for free variables.
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

    // Sort the decls into sets.
    let known = checked.iter().map(|decl| decl.name).collect::<HashSet<_>>();
    let decls = toposort(group(decls, |decl| decl.name), known, |decls| {
        let mut vars = BTreeSet::new();
        for decl in decls {
            vars.extend(decl.freevars());
        }
        vars.into_iter().collect()
    }).map_err(TypeError::MutualRecursion)?;

    // Check each "level" of the decls.
    for decl in decls {
        let decl = typeck_decls_with(decl, &checked)?;
        checked.extend(decl);
    }
    Ok(checked)
}

fn typeck_decls_with(
    decls: Vec<Decl<()>>,
    checked: &[Decl<Type>],
) -> Result<Vec<Decl<Type>>, TypeError> {
    // We assume later on that at least one decl exists.
    if decls.is_empty() {
        return Ok(Vec::new());
    }

    // First, create an environment containing the already-checked decls and use it to annotate
    // the decls.
    let mut env = AnnotEnv::new();
    for decl in checked {
        env.put_poly(decl.name, decl.aux());
    }
    env.put(decls[0].name, Ty::fresh());
    let mut decls = decls
        .into_iter()
        .map(|decl| decl.add_type_annotations(&mut env.clone()))
        .collect::<Vec<_>>();

    // Next, collect the type constraints and unify them into a substitution.
    let constraints = decls
        .iter()
        .flat_map(|decl| decl.collect_constraints())
        .collect();
    debug!("decls = {:?}", decls);
    debug!("constraints = {:?}", constraints);
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
    // We go BTreeSet->Vec instead of working with Vecs all the way through to
    // ensure uniqueness, and because it feels semantically closer to what we
    // want anyway. The Vec is only here because there's no .remove_arbitrary()
    // operation on sets.
    let mut constraints = constraints.into_iter().collect::<Vec<_>>();

    let mut subst = Substitution::new();
    while let Some(Constraint(s, t)) = constraints.pop() {
        debug!("Applying constraint {} ~ {}...", s, t);
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
