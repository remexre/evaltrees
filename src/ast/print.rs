use std::fmt::{Display, Formatter, Result as FmtResult};

use ast::{Decl, Expr};
use eval::lazy::LazyExpr;

/// The style of printing to `Display` with.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrintStyle {
    /// The abstract syntax tree style, which is fairly close to `Debug`.
    ///
    /// For example, `App(App(f, 1), App(g, 2))`.
    AST,

    /// The concrete syntax tree style.
    ///
    /// For example, `f 1 (g 2)`.
    CST,
}

macro_rules! display_type {
    ($name:ident, $ty:ident $(, $trait:path)*) => {
        struct $name<'a, Aux: 'a $(+ $trait)*>(&'a $ty<Aux>, PrintStyle);
        impl<'a, Aux: 'a $(+ $trait)*> Display for $name<'a, Aux> {
            fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
                match self.1 {
                    PrintStyle::AST => Display::fmt(self.0, fmt),
                    PrintStyle::CST => Display::fmt(&self.0.to_cst(), fmt),
                }
            }
        }

        impl<Aux> $ty<Aux>
            where $(Aux:$trait),*
        {
            /// Returns a Display that follows the given print style.
            pub fn display_as<'a>(&'a self, style: PrintStyle) -> impl 'a + Display {
                $name(self, style)
            }
        }
    };
}

display_type!(DisplayDecl, Decl);
display_type!(DisplayExpr, Expr);
display_type!(DisplayLazyExpr, LazyExpr, Clone);
