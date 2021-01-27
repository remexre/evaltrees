use std::path::PathBuf;

use evaltrees::ast::{Decl, PrintStyle};
use evaltrees::eval::{CallByName, CallByValue, Evaluator, LazyEvaluation};
use failure::{bail, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increases the verbosity. Default verbosity is errors and warnings.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// A file to load declarations from.
    #[structopt(short = "d", long = "decls", name = "FILE", parse(from_os_str))]
    pub decls_path: Option<PathBuf>,

    /// The expression to evaluate. If not present, a REPL will be started.
    #[structopt(short = "e", long = "expr", name = "EXPR")]
    pub expr: Option<String>,

    /// The evaluator to use.
    #[structopt(long = "evaluator", name = "EVALUATOR")]
    pub evaluator: Option<String>,

    /// Sets the print style to ASTs.
    #[structopt(long = "print-ast")]
    pub print_ast: bool,
}

impl Options {
    /// Creates an evaluator for the given declarations as set by the flags.
    pub fn make_evaluator(&self) -> Result<fn(Vec<Decl<()>>) -> Box<dyn Evaluator>, Error> {
        match self.evaluator.as_ref().map(|s| s as &str) {
            Some("lazy") => Ok(|decls| Box::new(LazyEvaluation::new(decls))),
            Some("name") => Ok(|decls| Box::new(CallByName::new(decls))),
            Some("value") | None => Ok(|decls| Box::new(CallByValue::new(decls))),
            Some(e) => bail!(
                "Unknown evaluator `{}' (valid evaluators are `lazy', `name', and `value')",
                e
            ),
        }
    }

    /// Gets the print style specified by the flags.
    pub fn print_style(&self) -> PrintStyle {
        if self.print_ast {
            PrintStyle::AST
        } else {
            PrintStyle::CST
        }
    }

    /// Sets up logging as specified by the `-q` and `-v` flags.
    pub fn start_logger(&self) {
        if !self.quiet {
            let r = ::stderrlog::new().verbosity(self.verbose).init();
            if let Err(err) = r {
                log::error!("Warning: logging couldn't start: {}", err);
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn setup_panic(&self) {}

    #[cfg(not(debug_assertions))]
    pub fn setup_panic(&self) {
        if self.verbose == 0 {
            human_panic::setup_panic!();
        }
    }
}
