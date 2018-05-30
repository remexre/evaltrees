use std::env::var_os;
use std::path::PathBuf;
use std::process::exit;

/// The `run` subcommand.
#[derive(Debug, StructOpt)]
pub struct RunOptions {
    /// The path to the main package.
    #[structopt(name = "PACKAGE-PATH", parse(from_os_str))]
    pub package_path: PathBuf,

    /// The binary to compile.
    #[structopt(name = "BINARY-NAME")]
    pub binary_name: String,

    /// Any options to pass to the program being run.
    pub args: Vec<String>,

    /// The path to the `std` package. If not present, defaults to
    /// `$OFTLISP_ROOT/std`.
    #[structopt(long = "std", name = "PATH", parse(from_os_str))]
    pub std_path: Option<PathBuf>,
}

impl RunOptions {
    /// Gets the path of the `std` package.
    pub fn std_path(&self) -> PathBuf {
        match self.std_path.as_ref() {
            Some(path) => path.as_path().into(),
            None => match var_os("OFTLISP_ROOT") {
                Some(path) => {
                    let mut path = PathBuf::from(path);
                    path.push("std");
                    path.into()
                }
                None => {
                    error!("Can't find the standard library; either pass --std or define the OFTLISP_ROOT environment variable");
                    exit(1);
                }
            },
        }
    }
}
