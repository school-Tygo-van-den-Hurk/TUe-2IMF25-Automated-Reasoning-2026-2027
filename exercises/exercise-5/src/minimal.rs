use ::oxidd::BooleanFunction;
use ::oxidd::Function;
use ::oxidd::HasLevel;
use ::oxidd::Manager;
use ::oxidd::bdd::BDDFunction;
use ::oxidd::util::AllocResult;
use ::std::collections::HashMap;
use ::std::collections::HashSet;

/// Calculate the minimum amount of variables that would need to be **true**
/// for the formula to be true
pub fn min_cost(f: &BDDFunction, cache: &mut CostCache) -> Option<usize> {
    if let Some(c) = cache.get(f) {
        return *c;
    }
    let result = match f.cofactors() {
        Some((hi, lo)) => {
            let hi_cost = min_cost(&hi, cache);
            let lo_cost = min_cost(&lo, cache);
            match (lo_cost, hi_cost) {
                (None, None) => None,
                (None, Some(y)) => Some(y + 1),
                (Some(x), None) => Some(x),
                (Some(x), Some(y)) => Some(std::cmp::min(x, y + 1)),
            }
        }
        None => {
            if f.satisfiable() {
                Some(0)
            } else {
                None
            }
        }
    };
    cache.insert(f.clone(), result);
    result
}

/// Calculate the model count with te minimum cost.
/// Comparable with sat_count for a [BBDFunction],
/// but does not take in to account te skipped levels.
/// This is since a skipped level means that those variables need to be false.
pub fn minimum_count(
    f: &BDDFunction,
    cache: &mut HashMap<BDDFunction, u32>,
    cost_cache: &mut CostCache,
) -> u32 {
    if let Some(c) = cache.get(f) {
        return *c;
    }
    let cost = min_cost(f, cost_cache);
    if cost.is_none() {
        return 0;
    }

    let result = match f.cofactors() {
        Some((hi, lo)) => {
            let hi_count = if let Some(hi_cost) = min_cost(&hi, cost_cache)
                && hi_cost < cost.unwrap()
            {
                minimum_count(&hi, cache, cost_cache)
            } else {
                0
            };
            let lo_count = if let Some(lo_cost) = min_cost(&lo, cost_cache)
                && lo_cost <= cost.unwrap()
            {
                minimum_count(&lo, cache, cost_cache)
            } else {
                0
            };
            hi_count + lo_count
        }
        None => {
            if f.satisfiable() {
                1
            } else {
                0
            }
        }
    };

    cache.insert(f.clone(), result);
    result
}

/// Get the unused variables of a BDDFunction
/// Walks through the BDD and removes the variable of the function to the used HashSet.
fn calculate_unused_variables(
    f: &BDDFunction,
    vars: &[BDDFunction],
    out: &mut HashSet<BDDFunction>,
    visited: &mut HashSet<BDDFunction>,
) {
    if visited.contains(f) || is_terminal(f) {
        return;
    }
    visited.insert(f.clone());
    let level = get_level(f);
    out.remove(&vars[level]);
    if let Some((hi, lo)) = f.cofactors() {
        calculate_unused_variables(&hi, vars, out, visited);
        calculate_unused_variables(&lo, vars, out, visited);
    }
}

/// Constrict the given BDD function by asserting that the unused variables are false
pub fn constrict_function_to_used_variables(
    f: &BDDFunction,
    vars: &[BDDFunction],
) -> AllocResult<BDDFunction> {
    let mut unused_variables = HashSet::from_iter(vars.iter().cloned());
    calculate_unused_variables(f, vars, &mut unused_variables, &mut HashSet::new());
    unused_variables
        .iter()
        .try_fold(f.clone(), |f, v| f.and(&BDDFunction::not(v)?))
}

fn is_terminal(f: &BDDFunction) -> bool {
    f.cofactors().is_none()
}

fn get_level(f: &BDDFunction) -> usize {
    f.with_manager_shared(|m, e| m.get_node(e).unwrap_inner().level()) as usize
}

/// Builds a vector for the BDDFunctions per level between the parent_level and the level
fn get_skipped_variables(
    parent_level: usize,
    level: usize,
    vars: &[BDDFunction],
) -> Vec<BDDFunction> {
    vars[parent_level + 1..level].to_vec()
}

// Build a sub BDDFunction, do not use on its own, its part of [minimum_configuration]
fn resolve_sub_min_config<F>(
    f: &BDDFunction,
    vars: &[BDDFunction],
    cost_cache: &mut CostCache,
    cache: &mut Cache,
    parent_level: usize,
    cost_predicate: F,
) -> AllocResult<BDDFunction>
where
    F: Fn(usize) -> bool,
{
    let f_false = f.with_manager_shared(|m, _| BDDFunction::f(m));
    if is_terminal(f) && !f.valid() {
        return Ok(f_false);
    }

    let level = if is_terminal(f) {
        vars.len()
    } else {
        get_level(f)
    };

    if let Some(cost) = min_cost(f, cost_cache)
        && cost_predicate(cost)
    {
        let t = f.with_manager_shared(|m, _| BDDFunction::t(m));

        let skipped_vars = get_skipped_variables(parent_level, level, vars)
            .iter()
            .try_fold(t, |acc, v| acc.and(&BDDFunction::not(v)?))?;

        let r = if is_terminal(f) {
            f.clone()
        } else {
            minimum_configuration(f, vars, cost_cache, cache)?
        };
        return r.and(&skipped_vars);
    }
    Ok(f_false)
}

pub type CostCache = HashMap<BDDFunction, Option<usize>>;
pub type Cache = HashMap<BDDFunction, BDDFunction>;

/// Transform the given BDDFunction into the BDDFunction that only reasons
/// about the assignments with the minimum amount of variables set to true.
pub fn minimum_configuration(
    f: &BDDFunction,
    vars: &[BDDFunction],
    cost_cache: &mut CostCache,
    cache: &mut Cache,
) -> AllocResult<BDDFunction> {
    if let Some(c) = cache.get(f) {
        return Ok(c.clone());
    }

    let cost = min_cost(f, cost_cache);
    if cost.is_none() {
        return Ok(f.with_manager_shared(|m, _| BDDFunction::f(m)));
    }

    let result = match f.cofactors() {
        Some((hi, lo)) => {
            let parent_level = get_level(f);
            let v = &vars[parent_level];
            let hi_pred = |x| x < cost.unwrap();
            let lo_pred = |x| x <= cost.unwrap();
            let hi_func =
                resolve_sub_min_config(&hi, vars, cost_cache, cache, parent_level, hi_pred)?;
            let lo_func =
                resolve_sub_min_config(&lo, vars, cost_cache, cache, parent_level, lo_pred)?;

            v.ite(&hi_func, &lo_func)
        }

        None => Ok(f.with_manager_shared(|m, _| {
            if f.satisfiable() {
                BDDFunction::t(m)
            } else {
                BDDFunction::f(m)
            }
        })),
    }?;

    cache.insert(f.clone(), result.clone());
    Ok(result)
}

/// Create a new BDDFunction that asserts the given essential variables.
pub fn make_essential_formula(
    f: &BDDFunction,
    vars: &[BDDFunction],
    essential_vars: &[usize],
) -> AllocResult<BDDFunction> {
    essential_vars
        .iter()
        .try_fold(f.clone(), |f, v| f.and(&vars[*v].clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxidd::ManagerRef;
    use oxidd::bdd::BDDManagerRef;
    use oxidd::util::AllocResult;

    fn setup(n: usize) -> (BDDManagerRef, Vec<BDDFunction>) {
        let mref = oxidd::bdd::new_manager(1024, 128, 2);
        let vars: Vec<BDDFunction> = mref
            .with_manager_exclusive(|manager| {
                AllocResult::from_iter(
                    manager
                        .add_vars(n as u32)
                        .map(|i| BDDFunction::var(manager, i)),
                )
            })
            .expect("cannot allocate functions");
        (mref, vars)
    }

    fn min_of(f: &BDDFunction) -> Option<usize> {
        let mut cache = HashMap::new();
        min_cost(f, &mut cache)
    }

    fn cases() -> Vec<(usize, Vec<Vec<i32>>)> {
        vec![
            // (1 ∨ 2) ∧ (¬1 ∨ 3)
            (3, vec![vec![1, 2], vec![-1, 3]]),
            // (1 ∨ 2) ∧ (4 ∨ 3)
            (4, vec![vec![1, 2], vec![4, 3]]),
            // (1 ∨ 2) ∧ (4 ∨ 3) ∧ 5
            (5, vec![vec![1, 2], vec![4, 3], vec![5]]),
            // just 5 (assuming 5 variables, so x1..x4 are free)
            (5, vec![vec![5]]),
            // (1 ∨ 2) ∧ (4 ∨ 3) with 5 variables (x5 appears in no clause)
            (5, vec![vec![1, 2], vec![4, 3]]),
            // (¬1 ∨ ¬2) ∧ (¬4 ∨ ¬3)
            (4, vec![vec![-1, -2], vec![-4, -3]]),
        ]
    }

    fn for_all_assignments(n: usize, mut check: impl FnMut(&[bool])) {
        for mask in 0u64..(1 << n) {
            let a: Vec<bool> = (0..n).map(|i| (mask >> i) & 1 == 1).collect();
            check(&a);
        }
    }

    // 3. Evaluate a BDD on a concrete assignment
    fn eval_at(f: &BDDFunction, vars: &[BDDFunction], a: &[bool]) -> bool {
        f.eval((0..vars.len() as u32).zip(a.iter().copied()))
    }

    fn cnf_holds(clauses: &[Vec<i32>], a: &[bool]) -> bool {
        clauses.iter().all(|clause| {
            clause.iter().any(|&lit| {
                let v = a[lit.unsigned_abs() as usize - 1];
                if lit > 0 { v } else { !v }
            })
        })
    }

    #[test]
    fn bdd_matches_cnf() {
        for (n, clauses) in cases() {
            let (mref, vars) = setup(n);
            let slices: Vec<&[i32]> = clauses.iter().map(|c| c.as_slice()).collect();
            let f = cnf_to_bdd(&mref, &vars, &slices).expect("");

            for_all_assignments(n, |a| {
                assert_eq!(
                    eval_at(&f, &vars, a),
                    cnf_holds(&clauses, a),
                    "mismatch for clauses {:?} at assignment {:?}",
                    clauses,
                    a
                );
            });
        }
    }

    fn cnf_to_bdd(
        mref: &BDDManagerRef,
        vars: &[BDDFunction],
        clauses: &[&[i32]],
    ) -> AllocResult<BDDFunction> {
        let mut f = mref.with_manager_shared(|m| BDDFunction::t(m));
        for clause in clauses {
            let mut c = mref.with_manager_shared(|m| BDDFunction::f(m));
            for &lit in *clause {
                let v = &vars[lit.unsigned_abs() as usize - 1];
                let l = if lit > 0 { v.clone() } else { v.not()? };
                c = c.or(&l)?;
            }
            f = f.and(&c)?;
        }
        Ok(f)
    }

    fn print_tree(f: &BDDFunction, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match f.cofactors() {
            Some((hi, lo)) => {
                let level = f.with_manager_shared(|m, e| m.get_node(e).unwrap_inner().level());
                let s = format!("{pad}x{level}?");
                let hi_str = format!("{pad} hi:");
                let hi_tree = print_tree(&hi, indent + 2);
                let lo_str = format!("{pad} lo:");
                let lo_tree = print_tree(&lo, indent + 2);
                format!("{s}\n{hi_str}\n{hi_tree}\n{lo_str}\n{lo_tree}")
            }
            None => format!("{pad}{}", if f.satisfiable() { "TRUE" } else { "FALSE" }),
        }
    }

    fn print_assignment(a: &[bool]) -> String {
        a.iter()
            .enumerate()
            .map(|(i, b)| format!("{}x{i}", if *b { "" } else { "!" }))
            .fold("".to_owned(), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
    }

    #[test]
    fn my_functions_on_cases() -> AllocResult<()> {
        for (n, clauses) in cases() {
            let (mref, vars) = setup(n);
            let slices: Vec<&[i32]> = clauses.iter().map(|c| c.as_slice()).collect();
            let f = cnf_to_bdd(&mref, &vars, &slices).expect("");

            let min_cost = min_of(&f).expect("");
            let mut brute_cost = usize::MAX;
            for_all_assignments(n, |a| {
                let count = a.iter().filter(|&&b| b).count();
                if eval_at(&f, &vars, a) {
                    brute_cost = std::cmp::min(brute_cost, count);
                }
            });

            assert_eq!(brute_cost, min_cost, "costs must be the same");
            let minimized = constrict_function_to_used_variables(
                &minimum_configuration(&f, &vars, &mut HashMap::new(), &mut HashMap::new())?,
                &vars,
            )
            .expect("");

            for_all_assignments(n, |a| {
                let count = a.iter().filter(|&&b| b).count();
                if !eval_at(&f, &vars, a) {
                    assert!(
                        !eval_at(&minimized, &vars, a),
                        "minimized cannot have a assignment to be true is its false in the original"
                    );
                } else {
                    if count == min_cost {
                        assert!(
                            eval_at(&minimized, &vars, a),
                            "original function evals to true, therefore with min assignment the minimized should also be true: \nAssignment:{} \nORIGINAL\n{} \n Minimized:\n{}",
                            print_assignment(a),
                            print_tree(&f, 0),
                            print_tree(&minimized, 0),
                        );
                    } else {
                        assert!(
                            !eval_at(&minimized, &vars, a),
                            "bdd with assignment of more than min cost should be false in minimized function \nAssignment:{} \nORIGINAL\n{} \n Minimized:\n{}\ncost: {}",
                            print_assignment(a),
                            print_tree(&f, 0),
                            print_tree(&minimized, 0),
                            min_cost,
                        );
                    }
                }
            });

            // todo: call your min-cost function on `f`
            // kk
            //
            // todo: compute the expected minimum with a brute-force loop over
            //       for_all_assignments, using cnf_holds and the selected-feature count
            // todo: call your pruning function and compare it per assignment
            let _ = f;
        }
        Ok(())
    }

    #[test]
    fn all_three_required() {
        let (_m, v) = setup(3);
        // x /\ y /\ z
        let f = v[0].and(&v[1]).unwrap().and(&v[2]).unwrap();
        assert_eq!(min_of(&f), Some(3));
    }

    #[test]
    fn single_or() {
        let (_m, v) = setup(2);

        // x \/ y
        let f = v[0].or(&v[1]).unwrap();
        assert_eq!(min_of(&f), Some(1));
    }

    #[test]
    fn two_disjunctions() {
        let (_m, v) = setup(4);
        // (x \/ y) /\ (z \/ w)
        let f = v[0]
            .or(&v[1])
            .unwrap()
            .and(&v[2].or(&v[3]).unwrap())
            .unwrap();
        assert_eq!(min_of(&f), Some(2));
    }

    #[test]
    fn negated_variable_is_free() {
        let (_m, v) = setup(2);
        // (x \/ ~y)
        let f = v[0].and(&v[1].not().unwrap()).unwrap();
        assert_eq!(min_of(&f), Some(1));
    }

    #[test]
    fn skipped_variable_costs_nothing() {
        // x1 does not appear in the function at all
        let (_m, v) = setup(3);
        // (x /\ y) /\ (~z \/ z)
        let f = v[0].and(&v[2]).unwrap();
        assert_eq!(min_of(&f), Some(2));
    }

    #[test]
    fn unsatisfiable_is_none() {
        let (_m, v) = setup(1);
        let f = v[0].and(&v[0].not().unwrap()).unwrap();
        assert_eq!(min_of(&f), None);
    }

    #[test]
    fn tautology_is_zero() {
        let (_m, v) = setup(1);
        let f = v[0].or(&v[0].not().unwrap()).unwrap();
        assert_eq!(min_of(&f), Some(0));
    }
    #[test]
    fn matches_brute_force() {
        let n = 5;
        let (_m, v) = setup(n);
        // (x0 ∨ x1) ∧ (¬x0 ∨ x2) ∧ (x3 ∨ x4)
        let c1 = v[0].or(&v[1]).unwrap();
        let c2 = v[0].not().unwrap().or(&v[2]).unwrap();
        let c3 = v[3].or(&v[4]).unwrap();
        let f = c1.and(&c2).unwrap().and(&c3).unwrap();
        // x1, and x3 or x4. keep x0 false
        assert_eq!(min_of(&f), Some(2));
    }
}
