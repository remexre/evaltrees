mod compile;
mod interpret;
mod run;

pub use options::compile::CompileOptions;
pub use options::interpret::InterpretOptions;
pub use options::run::RunOptions;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {
    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increases the verbosity. Default verbosity is errors only.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// The subcommand to run.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl Options {
    /// Sets up logging as specified by the `-q` and `-v` flags.
    pub fn start_logger(&self) {
        if !self.quiet {
            let r = ::stderrlog::new().verbosity(self.verbose).init();
            if let Err(err) = r {
                error!("Warning: logging couldn't start: {}", err);
            }
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Precompiles a program.
    #[structopt(name = "compile")]
    Compile(CompileOptions),

    /// Interprets a precompiled program.
    #[structopt(name = "interpret")]
    Interpret(InterpretOptions),

    /// Runs a program.
    #[structopt(name = "run")]
    Run(RunOptions),
}
