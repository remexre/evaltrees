//! Evaluation and evaluators.

pub mod util;
mod value;

use std::fmt::Display;

use failure::Error;

use ast::PrintStyle;
pub use eval::value::CallByValue;

// TODO: Should there be proptest/quickcheck tests for progress/preservation properties?

/// An evaluator, which determines the evaluation strategy.
pub trait Evaluator<Aux>: Display {
    /// Determines whether the primary expression is currently in a normal form, i.e. one that
    /// cannot be further reduced.
    fn normal_form(&self) -> bool;

    /// Sets the print style.
    fn set_print_style(&mut self, print_style: PrintStyle);

    /// Performs a single reduction step.
    fn step(&mut self) -> Result<(), Error>;
}
