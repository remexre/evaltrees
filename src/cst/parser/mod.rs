#[cfg_attr(feature = "cargo-clippy", allow(clippy))]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod grammar;
#[cfg(test)]
mod tests;

use std::str::FromStr;

use lalrpop_util::ParseError;

use cst::parser::grammar::Token;
use cst::{Decl, Expr};
use repl::ReplCommand;

impl FromStr for Decl {
    type Err = ParseError<usize, String, &'static str>;
    fn from_str(s: &str) -> Result<Decl, ParseError<usize, String, &'static str>> {
        grammar::DeclParser::new()
            .parse(s)
            .map_err(|e| e.map_token(|Token(_, s)| s.to_string()))
    }
}

impl FromStr for Expr {
    type Err = ParseError<usize, String, &'static str>;
    fn from_str(s: &str) -> Result<Expr, ParseError<usize, String, &'static str>> {
        grammar::ExprParser::new()
            .parse(s)
            .map_err(|e| e.map_token(|Token(_, s)| s.to_string()))
    }
}

/// Parses multiple semicolon-terminated decls.
pub fn parse_decls(src: &str) -> Result<Vec<Decl>, ParseError<usize, String, &'static str>> {
    grammar::DeclsParser::new()
        .parse(src)
        .map_err(|e| e.map_token(|Token(_, s)| s.to_string()))
}

impl FromStr for ReplCommand {
    type Err = ParseError<usize, String, &'static str>;
    fn from_str(s: &str) -> Result<ReplCommand, ParseError<usize, String, &'static str>> {
        grammar::ReplCommandParser::new()
            .parse(s)
            .map_err(|e| e.map_token(|Token(_, s)| s.to_string()))
    }
}
