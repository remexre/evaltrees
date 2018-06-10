extern crate evaltrees;
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
extern crate symbol;

mod options;
mod plain;
mod repl;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use evaltrees::ast::Decl;
use evaltrees::cst::{parse_decls, Expr as CstExpr};
use evaltrees::typeck::typeck;
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
    let decls = match options.decls_path.as_ref() {
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
        .map(|decl| decl.into_ast())
        .collect::<Result<Vec<_>, _>>()?;

    // Type-check the AST.
    let decls = typeck(decls, Vec::new())?;

    // Actually run the thing.
    if let Some(expr) = options.expr.as_ref() {
        let expr = expr.parse::<CstExpr>()?.into_ast()?;
        let expr_decl = Decl {
            name: "".into(),
            args: vec![],
            body: expr,
            aux: (),
        };
        let decls = typeck(vec![expr_decl], decls)?;
        plain::run(decls, &options)
    } else {
        repl::run(decls)
    }
}