use ::oxidd::BooleanFunction;
use ::oxidd::Manager;
use ::oxidd::ManagerRef;
use ::oxidd::bdd::BDDFunction;
use ::oxidd::util::AllocResult;
use ::std::collections::HashMap;
use ::std::fmt;
use ::std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dimacs {
    pub clauses_count: u32,
    pub variable_count: u32,
    pub has_ordering: bool,
    pub ordering: HashMap<u32, u32>,
    pub rev_ordering: HashMap<u32, u32>,
    pub clauses: Vec<Vec<Variable>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Variable {
    variable: u32,
    is_negated: bool,
}

impl Dimacs {
    pub fn new(var_count: u32, clause_count: u32) -> Self {
        Dimacs {
            clauses_count: clause_count,
            variable_count: var_count,
            has_ordering: false,
            ordering: HashMap::new(),
            rev_ordering: HashMap::new(),
            clauses: Vec::new(),
        }
    }

    pub fn add_ordering(&mut self, ordering: HashMap<u32, u32>) {
        self.has_ordering = true;
        self.ordering = ordering;
        self.rev_ordering = HashMap::new();

        for (k, v) in &self.ordering {
            self.rev_ordering.insert(*v, *k);
        }
    }

    pub fn add_clause(&mut self, clause: Vec<Variable>) {
        self.clauses.push(clause);
    }

    /// Build a BDD from the dimacs formula, with a given [BDDManagerRef].
    pub fn to_bdd(
        &self,
        manager_ref: &oxidd::bdd::BDDManagerRef,
    ) -> AllocResult<(BDDFunction, Vec<BDDFunction>)> {
        let variables: Vec<BDDFunction> = manager_ref.with_manager_exclusive(|m| {
            AllocResult::from_iter(
                m.add_vars(self.variable_count)
                    .map(|i| BDDFunction::var(m, i)),
            )
        })?;

        let t = manager_ref.with_manager_shared(|m| BDDFunction::t(m));
        let f = manager_ref.with_manager_shared(|m| BDDFunction::f(m));
        let formula = self.clauses.iter().try_fold(t, |cnf, clause| {
            let clause_fn = clause.iter().try_fold(f.clone(), |formula, var| {
                formula.or(&self.to_function(var, &variables)?)
            })?;
            cnf.and(&clause_fn)
        })?;
        Ok((formula, variables))
    }

    pub fn to_function(
        &self,
        var: &Variable,
        variables: &[BDDFunction],
    ) -> AllocResult<BDDFunction> {
        if var.is_negated {
            BDDFunction::not(&variables[self.to_index(var.variable)])
        } else {
            Ok(<BDDFunction as Clone>::clone(
                &variables[self.to_index(var.variable)],
            ))
        }
    }

    pub fn to_index(&self, var: u32) -> usize {
        // we subtract here so we have the number as the index
        if self.has_ordering {
            (self.ordering[&var] - 1) as usize
        } else {
            (var - 1) as usize
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVariableError {
    source: String,
    msg: String,
}

impl fmt::Display for ParseVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not parse '{}' as a variable: {}",
            self.source, self.msg
        )
    }
}

impl FromStr for Variable {
    type Err = ParseVariableError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("-").unwrap_or(s);
        match stripped.trim().parse::<u32>() {
            Ok(num) => Ok(Variable {
                is_negated: s.starts_with("-"),
                variable: num,
            }),
            Err(e) => Err(ParseVariableError {
                source: s.to_string(),
                msg: e.to_string(),
            }),
        }
    }
}
