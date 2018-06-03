use std::collections::BTreeSet;

use symbol::Symbol;

use ast::{Decl, Expr, Literal, Op, Pattern};
use cst::{Decl as CstDecl, Expr as CstExpr};

/// An error converting a concrete syntax tree to an abstract syntax tree.
/// These, generally, are errors which are valid syntax but have ill-defined
/// or invalid semantics.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum ASTConversionError {
    #[fail(display = "Found duplicate variable `{}' in arguments to `{}'.", _1, _0)]
    DuplicateArgVar(Symbol, Symbol),
}

/// Checks that the arguments contain no duplicate variables.
pub fn check_args(args: &[Pattern]) -> Result<(), Symbol> {
    let mut vars = BTreeSet::new();

    fn check_arg(vars: &mut BTreeSet<Symbol>, arg: &Pattern) -> Result<(), Symbol> {
        match *arg {
            Pattern::Binding(n, _) => if vars.insert(n) {
                Ok(())
            } else {
                Err(n)
            },
            Pattern::Cons(ref l, ref r, _) => {
                check_arg(vars, l)?;
                check_arg(vars, r)
            }
            Pattern::Literal(_, _) => Ok(()),
        }
    }

    for arg in args {
        check_arg(&mut vars, arg)?;
    }
    Ok(())
}

impl CstDecl {
    /// Converts a CST decl to an AST decl.
    pub fn into_ast(self) -> Result<Decl, ASTConversionError> {
        check_args(&self.args).map_err(|n| ASTConversionError::DuplicateArgVar(self.name, n))?;
        Ok(Decl {
            name: self.name,
            args: self.args,
            body: self.body.into_ast()?,
            aux: (),
        })
    }
}

impl CstExpr {
    /// Converts a CST decl to an AST decl.
    pub fn into_ast(self) -> Result<Expr, ASTConversionError> {
        match self {
            CstExpr::List(mut es) => {
                let mut expr = Expr::Literal(Literal::Nil, ());
                while let Some(e) = es.pop() {
                    expr = Expr::Op(Op::Cons, Box::new(e.into_ast()?), Box::new(expr), ());
                }
                Ok(expr)
            }
            CstExpr::Literal(lit) => Ok(Expr::Literal(lit, ())),
            CstExpr::Op(op, l, r) => Ok(Expr::Op(
                op,
                Box::new(l.into_ast()?),
                Box::new(r.into_ast()?),
                (),
            )),
            CstExpr::Variable(sym) => Ok(Expr::Variable(sym, ())),
        }
    }
}
