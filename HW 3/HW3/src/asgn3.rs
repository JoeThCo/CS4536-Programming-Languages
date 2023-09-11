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
#[derive(Hash,Eq, PartialEq, Debug)]
pub enum EnvRecord {
    FunRecord(Vec<String>, Box<Expr>),
    VarRecord(Value),
}

/* Values are programs that are pure data and require no further
 * computation. In this assignment, the only values are numerals,
 * i.e., literal numbers. */
/* Note: This "derive" line additionally tells Rust to generate code
 * for copying values of this type. */
#[derive(Hash,Eq, PartialEq, Debug, Clone)]
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
    Times(Box<Expr>,Box<Expr>),
    Plus(Box<Expr>,Box<Expr>),
    Minus(Box<Expr>,Box<Expr>),
    Let(Box<Defn>,Box<Expr>),
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
fn unimplemented_defn() -> HashTrieMap<String,EnvRecord>  {
  HashTrieMap::new()
}
/** Dummy code, so the starter code compiles */
fn unimplemented_expr() -> Value {
  Value::Numeral(0)
}
/** END STARTER CODE  
 *  You write the rest */

/** Staff solution length: 11 lines */
pub fn eval_defn(env: &HashTrieMap<String,EnvRecord>, d: &Defn) -> HashTrieMap<String,EnvRecord> {
  unimplemented_defn()
}

/* Staff solution length: 55 lines, 27 of which are for function calls */
pub fn eval_expr(env: &HashTrieMap<String,EnvRecord>, e: &Expr) -> Value {
  unimplemented_expr()
}