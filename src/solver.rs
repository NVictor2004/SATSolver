
use crate::parser::Expression::{self, *};

type VarMap = Vec<(String, i32)>;
type CNFRep = Vec<Vec<i32>>;

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

    fn expr_to_cnfrep_helper(self, varmap: &mut VarMap) -> CNFRep {
        match self {
            Var(var) => vec!(vec!(match lookup(&var, varmap) {
                Some(val) => val,
                None => update(var, varmap),
            })),
            Not(expr) => match *expr {
                Var(var) => vec!(vec!(match lookup(&var, varmap) {
                    Some(val) => -val,
                    None => -update(var, varmap),
                })),
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
                vec!(result.concat())
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

pub fn expr_to_cnfrep(expr: Expression) -> CNFRep {
    let cnf = expr.expr_to_cnf();
    println!("{cnf}");
    let mut varmap = Vec::new();
    cnf.expr_to_cnfrep_helper(&mut varmap)
}

fn update(var: String, varmap: &mut VarMap) -> i32 {
    let val = match varmap.last() {
        Some((_, val)) => val + 1,
        None => 1,
    };
    varmap.push((var, val));
    val
}

