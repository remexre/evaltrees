extern crate evaltrees;
#[macro_use]
extern crate failure;
#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

mod options;

use std::process::exit;

use structopt::StructOpt;

use options::{Options, Subcommand};

fn main() {
    let options = Options::from_args();
    options.start_logger();
    setup_panic(&options);

    let result = match options.subcommand {
        Subcommand::Compile(options) => compile::run(options),
        Subcommand::Interpret(options) => interpret::run(options),
        Subcommand::Run(options) => run::run(options),
    };

    if let Err(err) = result {
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

#[cfg(debug_assertions)]
fn setup_panic(_: &Options) {}

#[cfg(not(debug_assertions))]
fn setup_panic(options: &Options) {
    if options.verbose == 0 {
        setup_panic!();
    }
}
