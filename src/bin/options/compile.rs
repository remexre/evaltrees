use std::env::var_os;
use std::fs::create_dir_all;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::process::exit;

/// The `compile` subcommand.
#[derive(Debug, StructOpt)]
pub struct CompileOptions {
    /// The path to the main package.
    #[structopt(name = "PACKAGE-PATH", parse(from_os_str))]
    pub package_path: PathBuf,

    /// The binary to compile.
    #[structopt(name = "BINARY-NAME")]
    pub binary_name: String,

    /// The path to write the output file to.
    #[structopt(short = "o", long = "output", name = "OUTPUT-PATH", parse(from_os_str))]
    pub output_path: Option<PathBuf>,

    /// The path to the `std` package. If not present, defaults to
    /// `$OFTLISP_ROOT/std`.
    #[structopt(long = "std", name = "PATH", parse(from_os_str))]
    pub std_path: Option<PathBuf>,
}

impl CompileOptions {
    /// Returns the path to write the output file to.
    pub fn output_path(&self) -> Result<PathBuf, IoError> {
        match self.output_path {
            Some(ref path) => Ok(path.clone()),
            None => {
                let mut path = self.package_path.clone();
                path.push("build");
                create_dir_all(&path)?;
                path.push(format!("{}.ofta", self.binary_name));
                Ok(path)
            }
        }
    }

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
