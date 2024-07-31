
use std::collections::HashSet;

use crate::parser::Expression::{self, *};

type VarMap = Vec<(String, i32)>;
struct CNFRep {
    formula: Vec<HashSet<i32>>,
    trues: Vec<i32>,
}

impl Expression {
    fn expr_to_nnf(self) -> Expression {
        match self {
            Not(expr) => match *expr {
                Not(expr) => expr.expr_to_nnf(),
                Or(expr, expr2) => And(Box::new(Not(expr)), Box::new(Not(expr2))).expr_to_nnf(),
                And(expr, expr2) => Or(Box::new(Not(expr)), Box::new(Not(expr2))).expr_to_nnf(),
                _ => Not(Box::new(expr.expr_to_nnf())),
            },
            And(expr, expr2) => And(Box::new(expr.expr_to_nnf()), Box::new(expr2.expr_to_nnf())),
            Or(expr, expr2) => Or(Box::new(expr.expr_to_nnf()), Box::new(expr2.expr_to_nnf())),
            _ => self,
        }
    }
    fn distribute(expr: Expression, expr2: Expression) -> Expression {
        match expr {
            And(a, b) => And(Box::new(Expression::distribute(*a, expr2.clone())), Box::new(Expression::distribute(*b, expr2))),
            a => match expr2 {
                And(b, c) => And(Box::new(Expression::distribute(a.clone(), *b)), Box::new(Expression::distribute(a, *c))),
                b => Or(Box::new(a), Box::new(b)),
            }
        }
    }
    fn nnf_to_cnf(self) -> Expression {
        match self {
            Not(expr) => Not(Box::new(expr.nnf_to_cnf())),
            And(expr, expr2) => And(Box::new(expr.nnf_to_cnf()), Box::new(expr2.nnf_to_cnf())),
            Or(expr, expr2) => Expression::distribute(expr.nnf_to_cnf(), expr2.nnf_to_cnf()),
            Var(_) => self,
        }
    }

    fn expr_to_cnfrep_helper(self, varmap: &mut VarMap) -> Vec<HashSet<i32>> {
        match self {
            Var(var) => vec!(match lookup(&var, varmap) {
                Some(val) => HashSet::from([val]),
                None => HashSet::from([update(var, varmap)]),
            }),
            Not(expr) => match *expr {
                Var(var) => vec!(match lookup(&var, varmap) {
                    Some(val) => HashSet::from([-val]),
                    None => HashSet::from([-update(var, varmap)]),
                }),
                _ => panic!("Should not occur!"),
            },
            And(expr, expr2) => {
                let mut result = expr.expr_to_cnfrep_helper(varmap);
                result.append(&mut expr2.expr_to_cnfrep_helper(varmap));
                result
            },
            Or(expr, expr2) => {
                let mut result = expr.expr_to_cnfrep_helper(varmap);
                result.append(&mut expr2.expr_to_cnfrep_helper(varmap));
                vec!(result.iter().fold(HashSet::new(), |mut combine, set| { combine.extend(set); combine }))
            },
        }
    }

    fn expr_to_cnf(self) -> Expression {
        self.expr_to_nnf().nnf_to_cnf()
    }
}

fn lookup(value: &String, varmap: &VarMap) -> Option<i32> {
    Some(varmap.iter().find(|(v, _)| value == v)?.1)
}

fn update(var: String, varmap: &mut VarMap) -> i32 {
    let val = match varmap.last() {
        Some((_, val)) => val + 1,
        None => 1,
    };
    varmap.push((var, val));
    val
}

pub fn solve(expr: Expression) -> Vec<Vec<String>> {
    let mut varmap = Vec::new();
    let formula = expr.expr_to_cnf().expr_to_cnfrep_helper(&mut varmap);
    let mut cnfrep = CNFRep { formula, trues: Vec::new() };
    let solutions = dpll(&mut cnfrep);
    expand(solutions, varmap)
}

fn dpll(cnfrep: &mut CNFRep) -> Vec<Vec<i32>> {
    let mut all_unassigned = Vec::new();

    'clause_loop: for clause in &cnfrep.formula {
        let mut unassigned = Vec::new();
        for literal in clause {
            if cnfrep.trues.contains(literal) {
                continue 'clause_loop;
            }
            if !cnfrep.trues.contains(&-literal) {
                unassigned.push(*literal);
            }
        }

        if unassigned.is_empty() {
            return Vec::new();
        }

        if unassigned.len() == 1 {
            cnfrep.trues.push(unassigned.pop().unwrap());
            let result = dpll(cnfrep);
            cnfrep.trues.pop();
            return result;
        }

        all_unassigned.append(&mut unassigned);
    }

    if all_unassigned.is_empty() {
        return vec!(cnfrep.trues.clone());
    }

    let literal = all_unassigned.pop().unwrap();

    cnfrep.trues.push(literal);
    let mut result = dpll(cnfrep);
    cnfrep.trues.pop();

    cnfrep.trues.push(-literal);
    result.append(&mut dpll(cnfrep));
    cnfrep.trues.pop();

    result
}

fn reverse_lookup(number: i32, varmap: &VarMap) -> String {
    if number < 0 {
        let number = -number;
        return format!("!{}", varmap.iter().find(|(_, i)| number == *i).unwrap().0.clone());
    }
    varmap.iter().find(|(_, i)| number == *i).unwrap().0.clone()
}

fn all_assignments(solution: &mut Vec<i32>, varmap: &VarMap) -> Vec<Vec<i32>> {
    for (_, num) in varmap {
        if solution.contains(num) {
            continue;
        }
        if solution.contains(&-num) {
            continue;
        }
        solution.push(*num);
        let mut result = all_assignments(solution, varmap);
        solution.pop();

        solution.push(-num);
        result.append(&mut all_assignments(solution, varmap));
        solution.pop();
        return result;
    }
    vec!(solution.clone())
}

fn expand(solutions: Vec<Vec<i32>>, varmap: VarMap) -> Vec<Vec<String>> {
    let all_solutions: Vec<_> = solutions.into_iter().map(|mut solution| all_assignments(&mut solution, &varmap)).collect();
    all_solutions.concat().into_iter().map(|solution| solution.into_iter().map(| number | reverse_lookup(number, &varmap) ).collect()).collect()
}