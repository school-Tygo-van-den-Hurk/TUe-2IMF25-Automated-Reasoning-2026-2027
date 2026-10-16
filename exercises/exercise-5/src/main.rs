use ::oxidd::BooleanFunction;
use ::oxidd::Function;
use ::oxidd::Manager;
use ::oxidd::util::SatCountCache;
use ::oxidd::util::num::Natural;
use ::std::collections::HashMap;
use ::std::collections::hash_map::RandomState;
use ::clap::Parser;

pub mod dimacs;
pub mod minimal;
mod parser;
mod data;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of a builtin dataset (buildroot, busybox, embtoolkit, toybox, 
    /// uClinux) or file path
    #[arg(default_value = "buildroot")]
    dataset: String,
}

fn main() {
    let args = Args::parse();
    let file_content = data::get_dimacs(&args.dataset);
    let dimacs = parser::parse(file_content).expect("could not parse dataset");

    println!(
        "Processing: {}, has {} variables",
        args.dataset, dimacs.variable_count
    );

    let manager_ref = oxidd::bdd::new_manager(8_000_000, 4_000_000, 4);
    let (formula, vars) = dimacs.to_bdd(&manager_ref).expect("could not make bdd");
    let mut cost_cache = HashMap::new();

    formula.with_manager_shared(|m, _| m.gc());

    let min_cost = minimal::min_cost(&formula, &mut cost_cache).expect("formula is unsatiable");
    let mut cache: SatCountCache<Natural, RandomState> = SatCountCache::default();
    println!("φ");
    println!("- BDD nodes: {}", formula.node_count());
    println!(
        "- SAT count: {}",
        formula.sat_count(dimacs.variable_count, &mut cache)
    );

    println!("Minimum cost: {}", min_cost);
    println!(
        "SAT count of minimum assignment: {}",
        minimal::minimum_count(&formula, &mut HashMap::new(), &mut cost_cache)
    );

    let mut reduced_formula =
        minimal::minimum_configuration(&formula, &vars, &mut cost_cache, &mut HashMap::new())
            .expect("could not reduce formula");
    reduced_formula = minimal::constrict_function_to_used_variables(&reduced_formula, &vars)
        .expect("could not reduce formula");

    formula.with_manager_shared(|m, _| m.gc());

    let essential_variables: Vec<usize> = [218, 311]
        .iter()
        .map(|v| dimacs.to_index(*v as u32))
        .collect();

    let essential_formula = minimal::make_essential_formula(&formula, &vars, &essential_variables)
        .expect("could not make formula with essential variables");

    let min_cost_essential =
        minimal::min_cost(&essential_formula, &mut HashMap::new()).expect("formula is unsatiable");

    println!(
        "Minimum cost for essential variables {:?}: {}",
        essential_variables, min_cost_essential
    );

    let mut r_cache: SatCountCache<Natural, RandomState> = SatCountCache::default();
    println!("φ min");
    println!("- BDD nodes: {}", reduced_formula.node_count());
    println!(
        "- SAT count: {}",
        reduced_formula.sat_count(dimacs.variable_count, &mut r_cache)
    );
    println!("Done processing");
}
