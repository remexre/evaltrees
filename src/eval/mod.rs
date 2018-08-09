//! Evaluation and evaluators.

mod lazy;
mod name;
pub mod util;
mod value;

use std::fmt::Display;

use failure::Error;

use ast::PrintStyle;
pub use eval::lazy::LazyEvaluation;
pub use eval::name::CallByName;
pub use eval::value::CallByValue;

// TODO: Should there be proptest/quickcheck tests for progress/preservation properties?
// What about confluence?

/// An evaluator, which determines the evaluation strategy. When printed, it should
pub trait Evaluator: Display {
    /// Determines whether the primary expression is currently in a normal form, i.e. one that
    /// cannot be further reduced.
    fn normal_form(&self) -> bool;

    /// Sets the print style.
    fn set_print_style(&mut self, print_style: PrintStyle);

    /// Performs a single reduction step.
    fn step(&mut self) -> Result<(), Error>;

    /// Performs *n* steps.
    fn step_many(&mut self, mut n: usize) -> Result<(), Error> {
        while !self.normal_form() && n > 0 {
            self.step()?;
            n -= 1;
        }
        Ok(())
    }
}
