mod options;
mod plain;
mod repl;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use evaltrees::ast::Decl;
use evaltrees::cst::{parse_decls, Expr as CstExpr};
use evaltrees::typeck::typeck;
use anyhow::Error; // Changed from failure::Error
use structopt::StructOpt;

use crate::options::Options;

fn main() {
    let options = Options::from_args();
    options.start_logger();
    options.setup_panic();

    if let Err(err) = run(options) {
        // anyhow::Error's default Display includes the cause chain.
        // For debug builds or high verbosity, one might print with :? for more detail.
        log::error!("{:?}", err);
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> { // anyhow::Error
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
        repl::run(decls, options.print_style(), options.make_evaluator()?)
    }
}
