use std::path::PathBuf;

/// The `interpret` subcommand.
#[derive(Debug, StructOpt)]
pub struct InterpretOptions {
    /// The bytecode to run.
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,

    /// Any options to pass to the program being run.
    pub args: Vec<String>,
}
