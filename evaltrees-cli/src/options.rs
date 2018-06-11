use std::path::PathBuf;

use evaltrees::ast::{Decl, PrintStyle};
use evaltrees::eval::{CallByValue, Evaluator};
use failure::Error;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
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

    /// Sets the print style to ASTs.
    #[structopt(long = "print-ast")]
    pub print_ast: bool,
}

impl Options {
    /// Creates an evaluator for the given declarations as set by the flags.
    pub fn make_evaluator<Aux: 'static + Clone>(
        &self,
        decls: Vec<Decl<Aux>>,
    ) -> Result<Box<Evaluator<Aux>>, Error> {
        Ok(Box::new(CallByValue::new(decls)))
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
                error!("Warning: logging couldn't start: {}", err);
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn setup_panic(&self) {}

    #[cfg(not(debug_assertions))]
    pub fn setup_panic(&self) {
        if self.verbose == 0 {
            setup_panic!();
        }
    }
}
