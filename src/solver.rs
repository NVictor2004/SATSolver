
use std::collections::{HashMap, HashSet};

use crate::parser::Expression::{self, *};

struct VarMap {
    map: HashMap<String, i32>,
    length: i32,
}

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
    fn cnf_to_cnfrep_helper(self, varmap: &mut VarMap) -> Vec<HashSet<i32>> {
        match self {
            Var(var) => vec!(match varmap.map.get(&var) {
                Some(&val) => HashSet::from([val]),
                None => HashSet::from([varmap.update(var)]),
            }),
            Not(expr) => match *expr {
                Var(var) => vec!(match varmap.map.get(&var) {
                    Some(val) => HashSet::from([-val]),
                    None => HashSet::from([-varmap.update(var)]),
                }),
                _ => panic!("Should not occur!"),
            },
            And(expr, expr2) => {
                let mut result = expr.cnf_to_cnfrep_helper(varmap);
                result.append(&mut expr2.cnf_to_cnfrep_helper(varmap));
                result
            },
            Or(expr, expr2) => {
                let mut result = expr.cnf_to_cnfrep_helper(varmap);
                result.append(&mut expr2.cnf_to_cnfrep_helper(varmap));
                vec!(result.iter().fold(HashSet::new(), |mut combine, set| { combine.extend(set); combine }))
            },
        }
    }
    fn cnf_to_cnfrep(self, varmap: &mut VarMap) -> CNFRep {
        CNFRep::new(self.cnf_to_cnfrep_helper(varmap))
    }
}

impl CNFRep {
    fn new(formula: Vec<HashSet<i32>>) -> CNFRep {
        CNFRep {
            formula,
            trues: Vec::new(),
        }
    }
    fn dpll(&mut self) -> Vec<Vec<i32>> {
        let mut all_unassigned = Vec::new();
    
        'clause_loop: for clause in &self.formula {
            let mut unassigned = Vec::new();
            for literal in clause {
                if self.trues.contains(literal) {
                    continue 'clause_loop;
                }
                if !self.trues.contains(&-literal) {
                    unassigned.push(*literal);
                }
            }
    
            if unassigned.is_empty() {
                return Vec::new();
            }
    
            if unassigned.len() == 1 {
                self.trues.push(unassigned.pop().unwrap());
                let result = self.dpll();
                self.trues.pop();
                return result;
            }
    
            all_unassigned.append(&mut unassigned);
        }
    
        if all_unassigned.is_empty() {
            return vec!(self.trues.clone());
        }
    
        let literal = all_unassigned.pop().unwrap();
    
        self.trues.push(literal);
        let mut result = self.dpll();
        self.trues.pop();
    
        self.trues.push(-literal);
        result.append(&mut self.dpll());
        self.trues.pop();
    
        result
    }
}

impl VarMap {
    fn new() -> VarMap {
        VarMap {
            map: HashMap::new(),
            length: 0,
        }
    }
    fn update(&mut self, var: String) -> i32 {
        self.length += 1;
        self.map.insert(var, self.length);
        self.length
    }
    fn reverse_lookup(&self, number: i32) -> String {
        if number < 0 {
            let number = -number;
            return format!("!{}", self.map.iter().find(|(_, i)| number == **i).unwrap().0.clone());
        }
        self.map.iter().find(|(_, i)| number == **i).unwrap().0.clone()
    }
}

fn all_assignments(varmap: &VarMap, solution: &mut Vec<i32>) -> Vec<Vec<i32>> {
    for (_, num) in &varmap.map {
        if solution.contains(num) {
            continue;
        }
        if solution.contains(&-num) {
            continue;
        }
        solution.push(*num);
        let mut result = all_assignments(varmap, solution);
        solution.pop();

        solution.push(-num);
        result.append(&mut all_assignments(varmap, solution));
        solution.pop();
        return result;
    }
    vec!(solution.clone())
}
fn expand(varmap: VarMap, solutions: Vec<Vec<i32>>) -> Vec<Vec<String>> {
    let all_solutions: Vec<_> = solutions.into_iter().map(|mut solution| all_assignments(&varmap, &mut solution)).collect();
    all_solutions.concat().into_iter().map(|solution| solution.into_iter().map(| number | varmap.reverse_lookup(number) ).collect()).collect()
}

pub fn solve(expr: Expression) -> Vec<Vec<String>> {
    let mut varmap = VarMap::new();
    let solutions = expr.expr_to_nnf().nnf_to_cnf().cnf_to_cnfrep(&mut varmap).dpll();
    expand(varmap, solutions)
}