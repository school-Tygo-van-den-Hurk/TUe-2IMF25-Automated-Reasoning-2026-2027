//! The main CLI entrance. For SMT logic, look at `lib.rs`.

use ::clap::Parser;
use ::clap::Subcommand;

#[derive(Debug, Clone, Default, Subcommand)]
enum SubCommand {
    /// Check whether or not it is `SAT` or `UNSAT`.
    #[default]
    Check,

    /// Solve the sub-question and return the solution if it is SAT, or return
    /// UNSAT if it is not SAT.
    Solve,

    /// Print the question to stdout in the SMT2 format.
    Print,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The action to take.
    #[command(subcommand)]
    command: Option<SubCommand>,
}

fn main() {
    match Args::parse().command.unwrap_or_default() {
        SubCommand::Check => println!("SubCommand::Check"),
        SubCommand::Solve => println!("SubCommand::Solve"),
        SubCommand::Print => println!("SubCommand::Print"),
    }
}
