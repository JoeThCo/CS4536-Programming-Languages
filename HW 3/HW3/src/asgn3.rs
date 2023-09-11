/**
 * Assignment 3: Your task is to implement the functions
 * eval_decl and eval_expr. Together, these are an evaluator for the
 * language Toi. Both of these functions can call each other. See
 *
 * Specification: Your Rust code should agree with the pseudocode
 * provided in the book:
 *  interp_expr(E, Id(x)) = E(x)
 *  interp_expr(E, Number(n)) = n
 *  interp_expr(E, Times(e1,e2)) = interp_expr(E, e1) * interp_expr(E, e2)
 *  interp_expr(E, Plus(e1,e2)) = interp_expr(E, e1) + interp_expr(E, e2)
 *  interp_expr(E, Minus(e1,e2)) = interp_expr(E, e1) - interp_expr(E, e2)
 *  interp_expr(E, Let(d,e)) = interp_expr(interp_defn(E,d), e)
 *  interp_expr(E, Call(f,e1)) = interp_expr(E[x↦interp_expr(E,e1)], e2)
 *                               where E(f(x))=e2
 *  interp_defn(E,Var(x,e)) = E[x ↦ interp_expr(E, e)]
 *  interp_defn(E,Fun(f,x,e)) = E[f(x)↦e]
 *
 * In addition, your evaluator should support functions with multiple arguments.
 *
 * Scoring:  Each test case in main.rs is assigned a point value. Your score is
 * the sum of point values across all passing tests.
 * Function calls are a substantial part of the grade (35pts of 75pts)
 *
 * You are not graded on the length of your code. We mention the length of
 * the staff solutions to help you catch yourself if your solution is far more
 * complex than necessary. "A little longer" is perfectly fine.
 */
use rpds::HashTrieMap;

//Joe Colley 9/11/2023 - 9/15/2023
/** BEGIN STARTER CODE  */
/* The following definitions are provided to you. Take time to
 * read and understand them, because your code will build on them.
 *
 * EnvRecord defines a single record stored in the environment.
 * In the environment, the name of a variable or function is used
 * as its key and is thus not part of this record.
 * A FunRecord stores the argument names and the function body expression
 * A VarRecord stores the value of the variable
 *
 * Note: This "derive" line tells Rust to automatically generate code
 * which lets this type be used in maps (hash tables), equality tests,
 * and print statements in debugging code.*/
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum EnvRecord {
    FunRecord(Vec<String>, Box<Expr>),
    VarRecord(Value),
}

/* Values are programs that are pure data and require no further
 * computation. In this assignment, the only values are numerals,
 * i.e., literal numbers. */
/* Note: This "derive" line additionally tells Rust to generate code
 * for copying values of this type. */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Value {
    Numeral(i64),
}

/* Expressions are programs that we can evaluate. If they terminate,
* they give back a value. Expressions can be:
* Id: identifiers (variable names)
* Numeral: literal numbers
* Times: e1 * e2
* Plus: e1 + e2
* Let: let d in e  (see Defn for the different kinds of d)
* Call:  f(arg1, ..., argN)  (function calls, any number of args)
*/
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Expr {
    Id(String),
    Numeral(i64),
    Times(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Let(Box<Defn>, Box<Expr>),
    Call(String, Vec<Expr>),
}

/** Definitions are programs that, when we run them,
 * they define something, like a variable or function.
 * They can be:
 *   VarDefn(x,e) = defines x to equal the value of e
 *   FunDefn(f,[x1,...,xN],e) = defines function f(x1,...,xN)=e
 */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Defn {
    VarDefn(String, Box<Expr>),
    FunDefn(String, Vec<String>, Box<Expr>),
}

/** Dummy code, so the starter code compiles */
//TODO, be able to delete this
fn unimplemented_expr() -> Value {
    Value::Numeral(0)
}
/** END STARTER CODE  
 *  You write the rest */

/** Staff solution length: 11 lines */
pub fn eval_defn(env: &HashTrieMap<String, EnvRecord>, d: &Defn) -> HashTrieMap<String, EnvRecord> {
    match d {
        Defn::VarDefn(x, e) => {
            let value = eval_expr(env, e);
            let new_env = env.clone();
            let _ = new_env.insert(x.clone(), EnvRecord::VarRecord(value));
            new_env
        }
        Defn::FunDefn(f, params, body) => {
            let fun_record = EnvRecord::FunRecord(params.clone(), body.clone());
            let new_env = env.clone();
            let _ = new_env.insert(f.clone(), fun_record);
            new_env
        }
    }
}

/* Staff solution length: 55 lines, 27 of which are for function calls */
pub fn eval_expr(env: &HashTrieMap<String, EnvRecord>, e: &Expr) -> Value {
    match e {
        Expr::Id(x) => {
            // Lookup the identifier in the environment and return the corresponding value.
            match env.get(x) {
                Some(EnvRecord::VarRecord(value)) => value.clone(),
                // Handle undefined variables, TODO
                _ => unimplemented_expr(),
            }
        }
        // Literal numbers
        Expr::Numeral(n) => Value::Numeral(*n),
        Expr::Times(e1, e2) => {
            let v1 = eval_expr(env, e1);
            let v2 = eval_expr(env, e2);
            match (v1, v2) {
                (Value::Numeral(n1), Value::Numeral(n2)) => Value::Numeral(n1 * n2),
            }
        }
        Expr::Plus(e1, e2) => {
            let v1 = eval_expr(env, e1);
            let v2 = eval_expr(env, e2);
            match (v1, v2) {
                (Value::Numeral(n1), Value::Numeral(n2)) => Value::Numeral(n1 + n2),
            }
        }
        Expr::Minus(e1, e2) => {
            let v1 = eval_expr(env, e1);
            let v2 = eval_expr(env, e2);
            match (v1, v2) {
                (Value::Numeral(n1), Value::Numeral(n2)) => Value::Numeral(n1 - n2),
            }
        }
        Expr::Let(defn, body) => {
            // Evaluate the definition and update the environment
            let new_env = eval_defn(env, defn);
            // Evaluate the body with the updated environment
            eval_expr(&new_env, body)
        }
        Expr::Call(f, args) => {
            // Lookup the function in the environment
            match env.get(f) {
                Some(EnvRecord::FunRecord(params, body)) => {
                    // Create a new environment with parameter bindings
                    let new_env = env.clone();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let _ = new_env
                            .insert(param.clone(), EnvRecord::VarRecord(eval_expr(env, arg)));
                    }
                    // Evaluate the function body with the new environment
                    eval_expr(&new_env, body)
                }
                // Handle undefined functions, TODO
                _ => unimplemented_expr(),
            }
        }
    }
}
