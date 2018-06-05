use std::collections::HashMap;

use failure::Error;
use symbol::Symbol;

use ast::{Decl, Expr, Type};
use eval::util::{collect_types, reducible};
use eval::Evaluator;

/// Call-by-value evaluation.
#[derive(Debug, DisplayAttr)]
#[display(fmt = "{}", expr)]
pub struct CallByValue {
    decls: Vec<Decl<Type>>,
    expr: Expr<Type>,
    types: HashMap<Symbol, Type>,
}

impl CallByValue {
    /// Creates a call-by-value interpreter from a list of declarations.
    /// The nameless declaration will be the expression.
    pub fn new(decls: Vec<Decl<Type>>) -> CallByValue {
        let expr = {
            let nameless = decls
                .iter()
                .find(|decl| decl.name == "".into())
                .expect("Nameless declaration missing");
            Expr::Variable("".into(), nameless.aux_ref().clone())
        };
        let types = collect_types(&decls);
        CallByValue { decls, expr, types }
    }
}

impl Evaluator for CallByValue {
    fn normal_form(&self) -> bool {
        !reducible(&self.expr, &self.types)
    }

    fn step(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}
