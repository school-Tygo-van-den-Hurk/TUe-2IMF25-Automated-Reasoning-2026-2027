use crate::dimacs::*;
use ::std::collections::HashMap;
use ::std::str::FromStr;

pub fn parse(content: String) -> Result<Dimacs, String> {
    let mut lines = content.lines();
    let header_line = lines.next().ok_or_else(|| "No header".to_string())?;
    let mut result = parse_header(header_line)?;
    let clause_lines = lines
        .clone()
        .enumerate()
        .filter(|(_, l)| !l.starts_with("c vo"));
    for (index, line) in clause_lines {
        let clause = parse_clause((index + 1) as u32, line)?;
        result.add_clause(clause);
    }

    let ordering_lines: Vec<&str> = lines.filter(|l| l.starts_with("c vo")).collect();
    // we assume that if there is an ordering given then there is only one.
    if let Some(l) = ordering_lines.first() {
        let ordering = parse_ordering(l)?;

        if !ordering.is_empty() {
            result.add_ordering(ordering);
        }
    }
    Ok(result)
}

fn parse_header(line: &str) -> Result<Dimacs, String> {
    let trimmed = line
        .strip_prefix("p cnf ")
        .ok_or("Invalid header".to_string())?
        .trim();
    let parts: Vec<&str> = trimmed.split_whitespace().collect();

    let var_count = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
    let clause_count = parts[1].parse::<u32>().map_err(|e| e.to_string())?;

    Ok(Dimacs::new(var_count, clause_count))
}

fn parse_clause(line_index: u32, line: &str) -> Result<Vec<Variable>, String> {
    line.trim_end()
        .strip_suffix(" 0")
        .ok_or(format!("line {} does not end with '0'", line_index))?
        .split_whitespace()
        .map(|v| Variable::from_str(v).map_err(|e| e.to_string()))
        .collect()
}

fn parse_ordering(line: &str) -> Result<HashMap<u32, u32>, String> {
    line.strip_prefix("c vo")
        .ok_or("Line does not start with 'c vo'")?
        .split_whitespace()
        .enumerate()
        .map(|(i, v)| {
            v.parse::<u32>()
                .map(|var| (var, (i + 1) as u32))
                .map_err(|e| format!("Cannot parse integer '{}', error: {}", v, e))
        })
        .collect()
}
