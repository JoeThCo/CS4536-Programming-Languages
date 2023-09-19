/* Assignment 4: Static Types and Type-Checking 
 * Assignment Materials Copyright Rose Bohrer 2023
 * 
 * In this assignment, you implement a type-system for the Toi PL.
 * Your code takes in an abstract syntax tree (AST) as an input and,
 * without running the code, determines its type (or determines that
 * it does not type-check, i.e., it is ill-typed)
 * 
 * Your task is to implement the functions
 * type_check_expr (for expressions)
 * type_check_defn (for definitions)
 * 
 * Your submission is auto-graded using test cases; the score is the
 * sum of the points from each test case that passes. The rough point
 * breakdown is: 
 *  - 12 pts for numeric operations
 *  - 3 pts for strings
 *  - 30 pts for Boolean operations
 *  - 30 pts for functions
 * 
 * Specification + How to get started:
 * Chapter 9 of Human-Centered Programming Languages (Types) gives 
 * the typing rules for Toi and the pseudo-code for the type-checker.
 * 
 * Start by implementing the pseudocode from Chapter 9 in Rust. This will
 * get you close to correct, but the test cases for this assignment require
 * you to support a few extra things:
 * - We add a type String to Toi, but it only has values, no operations
 * - Your rule for function definitions needs to support recursive functions.
 *   The rule would look like this:
 *   (Γ, x : t1, f : t1 -> t2) ⊢ e : t2
 *   ---------------------------------------
 *   Γ ⊢ (f(x : t1) : t2 = e) : (f : t1→t2)
 *  
 *   For full points, implement this rule instead of the one in the book.
 *   If you get stuck, try translating it to pseudocode first. This inference
 *   rule notation often requires practice; reviewing rule notation from 
 *   Chapter 8 may help.
 * 
 * Additional resource: crates.io is a great resource for documentation about
 * Rust libraries. For any questions about using the HashTrieMap type, go to 
 * crates.io and search for "rpds"
 * 
 * Reminder: In the handout, I just mention the length of the staff solution 
 * so you can tell if your solution is getting over-complicated. It is fine 
 * if your solution is longer or shorter than mine.
 * */

/* BEGIN STARTER CODE: Read the following type  definitions because your
 * code will use these types */
use std::hash::Hash;
use rpds::HashTrieMap;


/* Syntax reminder: #[derive(...)] tells Rust to auto-generate certain helpful
 * code for working with this type. In particular, we auto-generate code that
 * supports putting Types in HashTrieMaps, printing them in debug messages, 
 * and copying them with .clone()*/
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
/* A value of t : Type represents a given type in the Toi language */
pub enum Type {
    Number,  /* represents "num" type */
    String,  /* represents "string" type */
    Boolean, /* represents "boolean" type */
    Function(Vec<Type>, Box<Type>), /* represents type of function t1 -> t2 */
}

/* This enumeration type lists out the different comparison operators */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Comparison {
    LessEqual,
    Less,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
}

/* A value e : Expr is an AST for a Toi expression  */
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Expr {
    Id(String),  /* Identifier, i.e., variable name */
    Numeral(i64), /* Number literal, e.g., 5 */
    StringLiteral(String), /* String literal, e.g., "hi" */
    True, /* Boolean literal true */
    False, /* Boolean literal true */
    /* To reduce the number of cases, we combine all the comparison operators.
     * The arguments indicate the first operand, what kind of comparison, 
     * then the second operand.
     * For example, Compare(e1,Greater,e2) is (e1 > e2) */
    Compare(Box<Expr>, Comparison, Box<Expr>),
    Times(Box<Expr>,Box<Expr>), /* Multiplication */
    Plus(Box<Expr>,Box<Expr>),  /* Addition */
    Minus(Box<Expr>,Box<Expr>), /* Subtraction */
    Let(Box<Defn>,Box<Expr>),   /* Let-definitions */
    Call(String, Vec<Expr>),    /* Function calls */
} 
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Defn {
    /* Variable definitions. Note that the AST does not list out the
     * type, because it can be inferred during type-checking */
    VarDefn(String, Box<Expr>), 
    /* Function definitions. We allow functions to be recursive, and as
     * a result, inferring types during type-checking is significantly
     * harder. To avoid this challenge, we list all the argument types
     * and the result type in the AST. Example in pseudo-Rust:
     * FunDecl("f", [("x",number),("y",number)], bool, Compare(x,Equal,y))
     * is the AST for the Toi function definition:
     *   function f(x:number,y:number):bool = 
     *     (x = y)
     * */
    FunDefn(String, Vec<(String,Type)>, Type, Box<Expr>),
}
/* END STARTER CODE */
/* START: You write the following functions */

/* Implement type-checking for definitions.
 * Arguments: "con" is the typing context Γ (pronounced Gamma)
 *            "d" is the AST for a definition
 * Specification:
 *   If the judgement Γ ⊢ d : Γ' holds, then 
 *   type_check_defn(Γ, d) = Some(Γ'). If not, type_check_defn(Γ,d) = None */
/* Staff solution length: 28 lines */
pub fn type_check_defn(con: &HashTrieMap<String, Type>, d: &Defn) -> Option<(String,Type)> {
    None /* Dummy implementation so it compiles */
}

/* Implement type-checking for expressions.
 * Arguments: "con" is the typing context Γ (pronounced Gamma)
 *            "e" is the AST for an expression
 * Specification:
 *   If the judgement Γ ⊢ e : t holds, then 
 *   type_check_expr(Γ, e) = Some(t). If not, type_check_expr(Γ,e) = None */
/* Staff solution length: 64 lines. The longest case is Call (18 lines) */
pub fn type_check_expr(con: &HashTrieMap<String, Type>, e: &Expr) -> Option<Type> {
  None /* Dummy implementation so it compiles */
}