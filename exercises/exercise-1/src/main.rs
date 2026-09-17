//! # Exercise 1: Magic Factory
//!
//! Six trucks (with rated speed 80km/h) have to deliver obscure building
//! blocks to a magic factory. The distance to the factory is 120 km. Every
//! truck has a max load of 8000 kg and can fit at most ten building blocks.
//! In total, the following has to be delivered:
//!
//! - Six nuzzles, each of weight 800 kg.
//! - Twelve prittles, each of weight 405 kg.
//! - Fifteen skipples, each of weight 500 kg.
//! - Eight crottles, each of weight 2500 kg.
//! - A number of dupples, each of weight 600 kg.
//!
//! If more than two types of goods are present in a truck, it has to drive at
//! a reduced speed of 60 km/h to lower the risk of magic interactions between
//! the goods. Skipples are round and elastic, only two out of the six trucks
//! have facilities for fastening them. Prittles are somewhat valuable: those
//! pallets need to be distributed over at least five trucks.
//!
//! a. Investigate what is the maximum number of dupples that can be delivered
//!    in a single run, and show how for that number all goods may be divided
//!    over the six trucks.
//! b. Do the same, with the extra information that crottles get too cold if
//!    there are less than two dupples in the same truck.
//!

use ::clap::Parser;
use ::clap::Subcommand;
use ::clap::ValueEnum;
use ::exercise_1::Exercise;

#[derive(Debug, Default, Clone, ValueEnum)]
enum SubQuestion {
    /// Sub-question 1a.
    A,
    /// Sub-question 1b.
    B,
    /// Both sub-question 1a and 1b.
    #[default]
    Both,
}

#[derive(Debug, Clone, Subcommand)]
enum SubCommand {
    /// Check whether or not it is `SAT` or `UNSAT`.
    Check {
        /// The sub-question to do this action for.
        #[arg(value_enum, default_value_t = SubQuestion::default())]
        sub_question: SubQuestion,
    },

    /// Solve the sub-question and return the solution if it is SAT, or return
    /// UNSAT if it is not SAT.
    Solve {
        /// The sub-question to do this action for.
        #[arg(value_enum, default_value_t = SubQuestion::default())]
        sub_question: SubQuestion,
    },

    /// Print the question to stdout in the SMT2 format.
    Print {
        /// The sub-question to do this action for.
        #[arg(value_enum, default_value_t = SubQuestion::default())]
        sub_question: SubQuestion,
    },
}

impl Default for SubCommand {
    fn default() -> Self {
        let sub_question = SubQuestion::default();
        Self::Solve { sub_question }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The action to take.
    #[command(subcommand)]
    command: Option<SubCommand>,
}

fn check(exercise: Exercise) {
    match exercise.optimizer.check(&[]) {
        z3::SatResult::Unsat => println!("UNSAT"),
        z3::SatResult::Sat => println!("SAT"),
        z3::SatResult::Unknown => println!("Unknown"),
    }
}

fn print(exercise: Exercise) {
    let mut smt2 = String::from(";> [!WARNING]");
    smt2.push_str(";> This is probably not complete and for debug purposes only!\n");
    smt2.push_str(";> I wrote this quickly to debug, **DO NOT RUN AS IS**.\n");
    smt2.push_str("\n");

    for assertion in exercise.optimizer.get_assertions() {
        smt2.push_str(&format!("(assert {})\n", assertion));
    }

    for objective in exercise.optimizer.get_objectives() {
        smt2.push_str(&format!("(minimize {})\n", objective));
    }

    smt2.push_str("(check-sat)\n");
    smt2.push_str("(get-model)\n");

    println!("{smt2}");
}

fn solve(exercise: Exercise) {
    match exercise.optimizer.check(&[]) {
        ::z3::SatResult::Unsat => println!("UNSAT"),
        ::z3::SatResult::Unknown => println!("Unknown SatResult"),
        ::z3::SatResult::Sat => match exercise.optimizer.get_model() {
            Some(model) => println!("{model:?}"),
            None => panic!("Unknown fatal error: SAT but no such model?"),
        },
    }
}

fn main() {
    let args = Args::parse();
    match args.command.unwrap_or_default() {
        SubCommand::Solve { sub_question } => match sub_question {
            SubQuestion::A => solve(Exercise::a()),
            SubQuestion::B => solve(Exercise::b()),
            SubQuestion::Both => {
                println!("question a: ");
                solve(Exercise::a());
                println!("question b: ");
                solve(Exercise::b());
            }
        },

        SubCommand::Check { sub_question } => match sub_question {
            SubQuestion::A => check(Exercise::a()),
            SubQuestion::B => check(Exercise::b()),
            SubQuestion::Both => {
                print!("question a: ");
                check(Exercise::a());
                print!("question b: ");
                check(Exercise::b());
            }
        },

        SubCommand::Print { sub_question } => match sub_question {
            SubQuestion::A => print(Exercise::a()),
            SubQuestion::B => print(Exercise::b()),
            SubQuestion::Both => {
                println!("question a: ");
                print(Exercise::a());
                println!("question b: ");
                print(Exercise::b());
            }
        },
    }
}
