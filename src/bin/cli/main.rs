extern crate evaltrees;
#[macro_use]
extern crate failure;
#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;
extern crate linefeed;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

mod options;
mod plain;
mod repl;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use evaltrees::cst::{parse_decls, Expr as CstExpr};
use evaltrees::typeck::{typeck_decls, typeck_expr};
use failure::Error;
use structopt::StructOpt;

use options::Options;

fn main() {
    let options = Options::from_args();
    options.start_logger();
    options.setup_panic();

    if let Err(err) = run(options) {
        let mut first = true;
        let num_errs = err.causes().count();
        if num_errs <= 1 {
            error!("{}", err);
        } else {
            for cause in err.causes() {
                if first {
                    first = false;
                    error!("           {}", cause);
                } else {
                    error!("caused by: {}", cause);
                }
            }
        }
        debug!("{}", err.backtrace());
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    // Load the CST of the declarations, if appropriate.
    let decls = match options.decls_path {
        Some(decls_path) => {
            let mut f = File::open(decls_path)?;
            let mut src = String::new();
            f.read_to_string(&mut src)?;
            parse_decls(&src)?
        }
        None => Vec::new(),
    };

    // Convert the CST to an AST.
    let decls = decls
        .into_iter()
        .map(|decl| decl.to_ast())
        .collect::<Result<Vec<_>, _>>()?;

    // Type-check the AST.
    let decls = typeck_decls(decls)?;

    // Actually run the thing.
    if let Some(expr) = options.expr {
        let expr = expr.parse::<CstExpr>()?.to_ast()?;
        let expr = typeck_expr(expr, &decls)?;
        plain::run(decls, expr)
    } else {
        repl::run(decls)
    }
}
