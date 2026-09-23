//! The main CLI entrance. For SMT logic, look at `lib.rs`.

use ::clap::Parser;
use ::clap::Subcommand;
use ::clap::ValueEnum;
use ::exercise_3::Question3;
use ::exercise_3::exercise;
use itertools::Itertools;

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

        /// From what iteration to start
        #[arg(short, long, default_value_t = usize::default())]
        from: usize,
    },

    /// Solve the sub-question and return the solution if it is SAT, or return
    /// UNSAT if it is not SAT.
    Solve {
        /// The sub-question to do this action for.
        #[arg(value_enum, default_value_t = SubQuestion::default())]
        sub_question: SubQuestion,

        /// From what iteration to start
        #[arg(short, long, default_value_t = usize::default())]
        from: usize,
    },

    /// Print the question to stdout in the SMT2 format.
    Print {
        /// The sub-question to do this action for.
        #[arg(value_enum, default_value_t = SubQuestion::default())]
        sub_question: SubQuestion,

        /// What step to print the SMT formula for.
        #[arg(short, long, default_value_t = usize::default())]
        step: usize,
    },
}

impl Default for SubCommand {
    fn default() -> Self {
        let sub_question = SubQuestion::default();
        Self::Solve {
            sub_question,
            from: usize::default(),
        }
    }
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The action to take.
    #[command(subcommand)]
    command: Option<SubCommand>,
}

fn check(sub_question: SubQuestion, from: usize) {
    use ::z3::SatResult::*;

    fn check_a(from: usize) {
        let mut max_step = from;
        let mut result = Sat;
        while result == Sat {
            max_step += 1;
            let exercise = exercise::A::smt(max_step);
            result = exercise.solver.check();
            println!("a@{max_step:04}) step {max_step} is {result:?}");
        }
    }

    fn check_b(from: usize) {
        let mut max_step = from;
        let mut result = Unsat;
        while result == Unsat {
            max_step += 1;
            let exercise = exercise::B::smt(max_step);
            result = exercise.solver.check();
            println!("b@{max_step:04}) step {max_step} is {result:?}");
        }
    }

    match sub_question {
        SubQuestion::A => check_a(from),
        SubQuestion::B => check_b(from),
        SubQuestion::Both => {
            check_a(from);
            check_b(from);
        }
    }
}

fn print(sub_question: SubQuestion, step: usize) {
    match sub_question {
        SubQuestion::A => println!("{}", exercise::A::smt(step).solver.to_smt2()),
        SubQuestion::B => println!("{}", exercise::B::smt(step).solver.to_smt2()),
        SubQuestion::Both => {
            println!("question a: ");
            println!("{}", exercise::A::smt(step).solver.to_smt2());
            println!("question b: ");
            println!("{}", exercise::B::smt(step).solver.to_smt2());
        }
    }
}

fn solve(sub_question: SubQuestion, from: usize) {
    use ::z3::SatResult::*;

    fn solve_a(from: usize) {
        let mut max_step = from;
        let mut result = Sat;
        while result == Sat {
            max_step += 1;
            let exercise = exercise::A::smt(max_step);
            result = exercise.solver.check();
            println!("a@{max_step:04}) step {max_step} is {result:?}");
        }
    }

    fn solve_b(from: usize) {
        let mut max_step = from;
        let mut result = Unsat;
        while result == Unsat {
            max_step += 1;
            let exercise = exercise::B::smt(max_step);
            result = exercise.solver.check();
            println!("b@{max_step:04}) step {max_step} is {result:?}");
            if let Some(model) = exercise.solver.get_model() {
                for line in model.to_string().lines().sorted() {
                    println!("b@{max_step:04})  {line}")
                }
            }
        }
    }

    match sub_question {
        SubQuestion::A => solve_a(from),
        SubQuestion::B => solve_b(from),
        SubQuestion::Both => {
            println!("question a: ");
            solve_a(from);
            println!("question b: ");
            solve_b(from);
        }
    }
}

fn main() {
    // let exercise = exercise::B::smt(2);
    // dbg!("{:?}", exercise.solver.get_model().unwrap());
    match Args::parse().command.unwrap_or_default() {
        SubCommand::Print {
            sub_question, step, ..
        } => print(sub_question, step),
        SubCommand::Solve {
            sub_question, from, ..
        } => solve(sub_question, from),
        SubCommand::Check {
            sub_question, from, ..
        } => check(sub_question, from),
    }
}
