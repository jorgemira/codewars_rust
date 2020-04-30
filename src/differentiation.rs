extern crate regex;

use std::fmt;

#[derive(Clone)]
enum Func {
    Cos,
    Sin,
    Tan,
    Exp,
    Ln,
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Func::Cos => write!(f, "cos"),
            Func::Sin => write!(f, "sin"),
            Func::Tan => write!(f, "tan"),
            Func::Exp => write!(f, "exp"),
            Func::Ln => write!(f, "ln"),
        }
    }
}

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Pow => write!(f, "^"),
        }
    }
}

#[derive(Clone)]
enum Expr {
    Fnc(Func, Box<Expr>),
    Opr(Op, Box<Expr>, Box<Expr>),
    Var,
    Cnt(f64),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Fnc(fun, exp) => write!(f, "({} {})", fun, exp),
            Expr::Opr(op, exp1, exp2) => write!(f, "({} {} {})", op, exp1, exp2),
            Expr::Var => write!(f, "x"),
            Expr::Cnt(n) => write!(f, "{}", n),
        }
    }
}

impl Expr {
    fn fnc(f: Func, e: Expr) -> Expr { Expr::Fnc(f, Box::new(e)) }

    fn opr(o: Op, e1: Expr, e2: Expr) -> Expr { Expr::Opr(o, Box::new(e1), Box::new(e2)) }

    fn from_iter(it: &mut dyn Iterator<Item=String>) -> Expr {
        if let Some(token) = it.next() {
            match &token[..] {
                "(" => Expr::from_iter(it),
                "+" => Expr::opr(Op::Add, Expr::from_iter(it), Expr::from_iter(it)),
                "-" => Expr::opr(Op::Sub, Expr::from_iter(it), Expr::from_iter(it)),
                "*" => Expr::opr(Op::Mul, Expr::from_iter(it), Expr::from_iter(it)),
                "/" => Expr::opr(Op::Div, Expr::from_iter(it), Expr::from_iter(it)),
                "^" => Expr::opr(Op::Pow, Expr::from_iter(it), Expr::from_iter(it)),

                "cos" => Expr::fnc(Func::Cos, Expr::from_iter(it)),
                "sin" => Expr::fnc(Func::Sin, Expr::from_iter(it)),
                "tan" => Expr::fnc(Func::Tan, Expr::from_iter(it)),
                "exp" => Expr::fnc(Func::Exp, Expr::from_iter(it)),
                "ln" => Expr::fnc(Func::Ln, Expr::from_iter(it)),

                "x" => Expr::Var,

                n if n.parse::<i32>().is_ok() => Expr::Cnt(n.parse::<f64>().unwrap()),

                _ => unreachable!(),
            }
        } else { unreachable!() }
    }

    pub fn parse(s: &str) -> Expr {
        //TODO: Maybe change rexgexp to remove ')' and ' ' and change funcs to symbols to use only one char
        let re = regex::Regex::new(r"(\(|-?\d+|\w+|\d+|\+|\*|\+|\^|-|/)").unwrap();
        let mut vec = Vec::new();
        for caps in re.captures_iter(s) {
            vec.push(caps.get(1).unwrap().as_str().to_string());
        }
        Self::from_iter(&mut vec.into_iter())
    }

    fn simplify(&self) -> Expr {
        match self {
            Expr::Fnc(f, e) => Expr::Fnc(f.clone(), Box::new(e.simplify())),
            Expr::Opr(op, e1, e2) => {
                let (e1, e2) = (e1.simplify(), e2.simplify());
                match (op, &e1, &e2) {
                    // Simplify sum
                    (Op::Add, Expr::Cnt(n), _) if *n == 0.0 => e2, // 0 + e = e
                    (Op::Add, _, Expr::Cnt(n)) if *n == 0.0 => e1, // e + 0 = e
                    (Op::Add, Expr::Cnt(n), Expr::Cnt(m)) => Expr::Cnt(n + m), // a + b
                    // Simplify substraction
                    (Op::Sub, _, Expr::Cnt(n)) if *n == 0.0 => e1, // e - 0 = e
                    (Op::Sub, Expr::Var, Expr::Var) => Expr::Cnt(0.0), // x - x = 0
                    (Op::Sub, Expr::Cnt(n), Expr::Cnt(m)) => Expr::Cnt(n - m), // a - b
                    //Simplify multiplication
                    (Op::Mul, Expr::Cnt(n), _) if *n == 1.0 => e2, // 1 * e = e
                    (Op::Mul, _, Expr::Cnt(n)) if *n == 1.0 => e1, // e * 1 = e
                    (Op::Mul, _, Expr::Cnt(n)) | (Op::Mul, Expr::Cnt(n), _) if *n == 0.0 => Expr::Cnt(0.0), // 0 * e or e * 0 = 0
                    (Op::Mul, Expr::Cnt(n), Expr::Cnt(m)) => Expr::Cnt(n * m), // a * b
                    // Simplify division
                    (Op::Div, Expr::Cnt(n), _) if *n == 0.0 => Expr::Cnt(0.0), // 0 / e = 0
                    (Op::Div, _, Expr::Cnt(n)) if *n == 1.0 => e1, // e / 1 = e
                    (Op::Div, Expr::Var, Expr::Var) => Expr::Cnt(1.0), // x / x = 1
                    (Op::Div, Expr::Cnt(n), Expr::Cnt(m)) => Expr::Cnt(n / m), // a / b
                    // Simplify power
                    (Op::Pow, _, Expr::Cnt(n)) if *n == 0.0 => Expr::Cnt(1.0), // e ^ 0 = 1
                    (Op::Pow, Expr::Cnt(n), _) if *n == 0.0 => Expr::Cnt(0.0), // 0 ^ e = 0
                    (Op::Pow, _, Expr::Cnt(n)) if *n == 1.0 => e1, // e ^ 1 = e
                    (Op::Pow, Expr::Cnt(n), _) if *n == 1.0 => Expr::Cnt(1.0), // 1 ^e = 0
                    (Op::Pow, Expr::Cnt(n), Expr::Cnt(m)) => Expr::Cnt(n.powf(*m as f64)), // a ^ b
                    // Unsimplifiable operations
                    _ => Expr::opr(op.clone(), e1, e2)
                }
            }
            _ => self.clone()
        }
    }

    fn derivative(&self) -> Expr {
        match self {
            Expr::Fnc(fun, g) => {
                let dg = g.derivative();
                match fun {
                    Func::Sin => Expr::opr(Op::Mul,
                                           dg,
                                           Expr::fnc(Func::Cos, *g.clone())),
                    Func::Cos => Expr::opr(Op::Mul,
                                           dg,
                                           Expr::opr(Op::Mul,
                                                     Expr::Cnt(-1.0),
                                                     Expr::fnc(Func::Sin, *g.clone()))),
                    Func::Tan => Expr::opr(Op::Mul,
                                           dg,
                                           Expr::opr(Op::Add,
                                                     Expr::Cnt(1.0),
                                                     Expr::opr(Op::Pow,
                                                               Expr::fnc(Func::Tan, *g.clone()),
                                                               Expr::Cnt(2.0)))),
                    Func::Exp => Expr::opr(Op::Mul, dg, self.clone()),
                    Func::Ln => Expr::opr(Op::Mul,
                                          dg,
                                          Expr::opr(Op::Div,
                                                    Expr::Cnt(1.0),
                                                    *g.clone())),
                }
            }
            Expr::Opr(op, f, g) => {
                let (df, dg) = (f.derivative(), g.derivative());
                match op {
                    // f'(x) + g'(x)
                    Op::Add => Expr::opr(Op::Add, df, dg),
                    // f'(x) - g'(x)
                    Op::Sub => Expr::opr(Op::Sub, df, dg),
                    // f'(x)g(x) + f(x)g'(x)
                    Op::Mul => Expr::opr(Op::Add,
                                         Expr::opr(Op::Mul, df, *g.clone()),
                                         Expr::opr(Op::Mul, *f.clone(), dg)),
                    // f'(x)g(x) - f(x)g'(x) / (g(x))^2
                    Op::Div => Expr::opr(Op::Div,
                                         Expr::opr(Op::Sub,
                                                   Expr::opr(Op::Mul, df, *g.clone()),
                                                   Expr::opr(Op::Mul, *f.clone(), dg)),
                                         Expr::opr(Op::Pow,
                                                   *g.clone(),
                                                   Expr::Cnt(2.0))),

                    Op::Pow => Expr::opr(Op::Mul,
                                         *g.clone(),
                                         Expr::opr(Op::Pow,
                                                   *f.clone(),
                                                   Expr::opr(Op::Sub,
                                                             *g.clone(),
                                                             Expr::Cnt(1.0))))
                }
            }
            Expr::Var => Expr::Cnt(1.0),
            Expr::Cnt(_) => Expr::Cnt(0.0),
        }
    }
}

pub fn diff(expr: &str) -> String {
    format!("{}", Expr::parse(expr).derivative().simplify())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("(+ x x)"), "2");
        assert_eq!(diff("(- x x)"), "0");
        assert_eq!(diff("(* x 2)"), "2");
        assert_eq!(diff("(/ x 2)"), "0.5");
        assert_eq!(diff("(^ x 2)"), "(* 2 x)");
        assert_eq!(diff("(cos x)"), "(* -1 (sin x))");
        assert_eq!(diff("(sin x)"), "(cos x)");
        assert_eq!(diff("(tan x)"), "(+ 1 (^ (tan x) 2))");
        assert_eq!(diff("(exp x)"), "(exp x)");
        assert_eq!(diff("(ln x)"), "(/ 1 x)");
        assert_eq!(diff("(+ x (+ x x))"), "3");
        assert_eq!(diff("(- (+ x x) x)"), "1");
        assert_eq!(diff("(* 2 (+ x 2))"), "2");
        assert_eq!(diff("(/ 2 (+ 1 x))"), "(/ -2 (^ (+ 1 x) 2))");
        assert_eq!(diff("(cos (+ x 1))"), "(* -1 (sin (+ x 1)))");

        let result = diff("(cos (* 2 x))");
        assert!(
            result == "(* 2 (* -1 (sin (* 2 x))))".to_string()
                || result == "(* -2 (sin (* 2 x)))".to_string()
                || result == "(* (* -1 (sin (* 2 x))) 2)".to_string()
        );

        assert_eq!(diff("(sin (+ x 1))"), "(cos (+ x 1))");
        assert_eq!(diff("(sin (* 2 x))"), "(* 2 (cos (* 2 x)))");
        assert_eq!(diff("(tan (* 2 x))"), "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))");
        assert_eq!(diff("(exp (* 2 x))"), "(* 2 (exp (* 2 x)))");
        assert_eq!(diff(&diff("(sin x)")), "(* -1 (sin x))");
        assert_eq!(diff(&diff("(exp x)")), "(exp x)");

        let result = diff(&diff("(^ x 3)"));
        assert!(result == "(* 3 (* 2 x))".to_string() || result == "(* 6 x)".to_string());
    }
}
