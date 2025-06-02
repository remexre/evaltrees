mod error;
mod options;
mod plain;
mod repl;

pub use error::CliError; // Make CliError available

use std::fs::File;
use std::io::Read;
use std::process::exit;

use evaltrees::ast::Decl;
use evaltrees::cst::{parse_decls, Expr as CstExpr};
use evaltrees::typeck::typeck;
// No longer use anyhow::Error, will use crate::CliError
use structopt::StructOpt;

use crate::options::Options;
use crate::CliError; // Import CliError

fn main() {
    let options = Options::from_args();
    options.start_logger();
    options.setup_panic();

    if let Err(err) = run(options) {
        // CliError implements std::error::Error and Display.
        // Using {err} should give a user-friendly message.
        // For debug, one might use {err:?} or log the source chain.
        log::error!("Error: {}", err);
        // Optionally, print source chain in verbose/debug mode
        // let mut source = err.source();
        // while let Some(e) = source {
        //     log::error!("Caused by: {}", e);
        //     source = e.source();
        // }
        exit(1);
    }
}

fn run(options: Options) -> Result<(), CliError> {
    // Load the CST of the declarations, if appropriate.
    let decls_cst = match options.decls_path.as_ref() {
        Some(decls_path) => {
            let mut f = File::open(decls_path)?; // -> CliError::Io
            let mut src = String::new();
            f.read_to_string(&mut src)?; // -> CliError::Io
            parse_decls(&src)? // -> CliError::SyntaxParse (via From)
        }
        None => Vec::new(),
    };

    // Convert the CST to an AST.
    // map_err is needed if the error type from collect isn't ASTConversionError directly
    // However, into_ast() returns ASTConversionError, so .collect::<Result<Vec<_>, _>>() should work if ASTConversionError is std::error::Error
    let decls_ast = decls_cst
        .into_iter()
        .map(|decl| decl.into_ast()) // -> Result<_, ASTConversionError>
        .collect::<Result<Vec<_>, ASTConversionError>>()?; // -> CliError::AstConvert (via From)

    // Type-check the AST.
    let mut decls = typeck(decls_ast, Vec::new())?; // -> CliError::TypeCheck (via From)

    // Actually run the thing.
    if let Some(expr_str) = options.expr.as_ref() {
        let expr_cst = expr_str.parse::<CstExpr>()?; // -> CliError::SyntaxParse (via From)
        let expr_ast = expr_cst.into_ast()?; // -> CliError::AstConvert (via From)
        let expr_decl = Decl {
            name: "".into(),
            args: vec![],
            body: expr_ast,
            aux: (),
        };
        // Need to ensure decls from file and expr decl are correctly combined for typecking
        // The existing typeck call seems to take new decls first, then context decls.
        decls = typeck(vec![expr_decl], decls)?; // -> CliError::TypeCheck (via From)
        plain::run(decls, &options)? // plain::run returns Result<(), CliError>
    } else {
        repl::run(decls, options.print_style(), options.make_evaluator()?)? // repl::run returns Result<(), CliError>
    }
    Ok(())
}
