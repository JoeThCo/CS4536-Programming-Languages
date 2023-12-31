use peg::*;
//Joe Colley 9/8/2023

/* Implement a PEG parser for the following context-free grammar.
 *
 * Implement each terminal symbol according to its English specification
 * Terminal Symbols:
 * An id starts with any letter, optionally followed by any combination
 * of letters, numbers and underscores.
 *
 * A numeral is an optional minus sign, then a required integer part,
 * then an optional period and fractional part.
 * An integer part optionally starts with a minus sign, which is then
 * followed by either a single 0 or a nonzero digit.
 * A fractional part is a nonempty sequence of digits.
 *
 * Variable Symbols:
 * Atom <- numeral |  id "(" ArgList ")" | id | "(" Expr ")"
 * Op2 <- Atom * Op2 | Atom
 * Op1 <- Op2 + Op1 | Op2 - Op1 | Op2
 * Expr <- "let" Decl "in" Expr | Op1
 * Decl <- "var" id "=" Expr | "function" id "(" ArgList ")" "{" Expr "}"
 *
 * NonEmptyArgList <- id, NonEmptyArgList | id
 * ArgList <-  NonEmptyArgList | <empty string>
 */

/* BEGIN STARTER CODE */
/* Your code will use Expr and Decl.
Do read these definitions */
#[derive(Clone)]
pub enum Expr {
    Id(String),
    Numeral(f64),
    Times(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Let(Box<Decl>, Box<Expr>),
    FunCall(String, Vec<Expr>),
}
#[derive(Clone)]
pub enum Decl {
    VarDecl(String, Box<Expr>),
    FunDecl(String, Vec<String>, Box<Expr>),
}

/* The following three functions can be used in debugging and testing code.
You do not need them to implement the homework itself. */
pub fn expr_eq(e1: Expr, e2: Expr) -> bool {
    match (e1, e2) {
        (Expr::Id(s1), Expr::Id(s2)) => s1 == s2,
        (Expr::Numeral(n1), Expr::Numeral(n2)) => n1 == n2,
        (Expr::Times(l1, r1), Expr::Times(l2, r2)) => expr_eq(*l1, *l2) && expr_eq(*r1, *r2),
        (Expr::Plus(l1, r1), Expr::Plus(l2, r2)) => expr_eq(*l1, *l2) && expr_eq(*r1, *r2),
        (Expr::Minus(l1, r1), Expr::Minus(l2, r2)) => expr_eq(*l1, *l2) && expr_eq(*r1, *r2),
        (Expr::Let(d1, e1), Expr::Let(d2, e2)) => decl_eq(*d1, *d2) && expr_eq(*e1, *e2),
        (Expr::FunCall(f1, args1), Expr::FunCall(f2, args2)) => {
            for (x, y) in args1.iter().zip(args2.iter()) {
                if !expr_eq(x.clone(), y.clone()) {
                    return false;
                }
            }

            return f1 == f2;
        }
        _ => false,
    }
}

pub fn decl_eq(d1: Decl, d2: Decl) -> bool {
    match (d1, d2) {
        (Decl::FunDecl(f1, args1, body1), Decl::FunDecl(f2, args2, body2)) => {
            f1 == f2 && args1 == args2 && expr_eq(*body1, *body2)
        }
        (Decl::VarDecl(x1, body1), Decl::VarDecl(x2, body2)) => x1 == x2 && expr_eq(*body1, *body2),
        _ => false,
    }
}

pub fn expr_to_string(e: Expr) -> String {
    match e {
        Expr::Id(s) => s,
        Expr::Numeral(f) => f.to_string(),
        Expr::Times(l, r) => format!("{}*{}", expr_to_string(*l), expr_to_string(*r)),
        Expr::Plus(l, r) => format!("{}+{}", expr_to_string(*l), expr_to_string(*r)),
        Expr::Minus(l, r) => format!("{}-{}", expr_to_string(*l), expr_to_string(*r)),
        Expr::Let(d, e) => format!("let {} in {}", decl_to_string(*d), expr_to_string(*e)),
        Expr::FunCall(f, args) => {
            let mut arg_str = "".to_string();
            for s in args {
                arg_str = format!("{},{}", arg_str, expr_to_string(s))
            }
            format!("{}({})", f, arg_str)
        }
    }
}

pub fn decl_to_string(d: Decl) -> String {
    match d {
        Decl::FunDecl(f, al, b) => {
            let mut arg_str = "".to_string();
            for s in al {
                arg_str = format!("{}{}", arg_str, s)
            }
            format!("function {}({}){{{}}}", f, arg_str, expr_to_string(*b))
        }
        Decl::VarDecl(x, e) => format!("var {} = {}", x, expr_to_string(*e)),
    }
}

pub fn e_res_to_str(r: Result<Expr, peg::error::ParseError<peg::str::LineCol>>) -> String {
    match r {
        Ok(s) => expr_to_string(s),
        Err(_s) => "err".to_string(),
    }
}
/* END STARTER CODE */

/* BEGIN ASSIGNMENT: */

/* We list how long the staff solutions are in order to help you
find out if you are overcomplicating a problem. Yours do not
need to be the same length as ours, nor use the same number of helpers. */
peg::parser! {
  pub grammar parser() for str {
  /* Provided helper functions to make the starter code type-check */
  //pub rule unimplemented_string() -> String = empty:$("") {? Ok(empty.to_string())}
  //pub rule unimplemented_expr() -> Expr = empty:$("") {? Ok(Expr::Numeral(383962395862.0)) }
  //pub rule unimplemented_decl() -> Decl = empty:$("") {? Ok(Decl::VarDecl("x".to_string(), Box::new(Expr::Numeral(893.923))))}

  /* YOUR CODE: */
  /* Parse a single identifier (i.e., variable name)
     Staff solution length: 2 lines */
    pub rule id() -> String = s:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
    {
        s.to_string()
    }

 /* Parse a single variable. The cleanest solution uses id() as a helper.
    var() behaves just like id(), except with a different return type.
    Staff solution length: 1 lines */
    pub rule var() -> Expr = name:id()
    {
        Expr::Id(name)
    }

    //get all digits 0 -9
    rule digit() -> String = d:$(['0'..='9']) { d.parse().unwrap() }

  /* Parse a single literal number.
    Staff solution length: 6 lines */
    //couldnt figure out two niche cases but I gave it my best
       pub rule numeral() -> Expr
            = n:$((['-']? digit()+ ("." digit()+)?)) {
            Expr::Numeral(n.parse().unwrap())
        }

  /* Implement a parser for (all the) expressions. You should define
     and call your own helpers. See the precedence-climbing approach
     from the book and lecture to help decide on your helpers.
     Both expr() and decl() will call each other.
     Staff solution length: 10 lines, including 7 helpers */
    pub rule expr() -> Expr
    = "let " d:decl() " in " e:expr() {
        Expr::Let(Box::new(d), Box::new(e))
    }
    / "function " name:id() "(" args:non_empty_arg_list_strings() ")" "{" body:expr() "}" {
        let args_expr: Vec<Expr> = args.iter().map(|arg| Expr::Id(arg.clone())).collect();
        let name_clone = name.clone();

        Expr::Let(
            Box::new(Decl::FunDecl(name, args, Box::new(body))),
            Box::new(Expr::FunCall(name_clone, args_expr)),
        )
    }
    / e:op1() {
        e
    }
    / "(" e:expr() ")" {
        e
    }
    / "let var " name:id() " = " e:expr() " in " e2:expr() {
        Expr::Let(
            Box::new(Decl::VarDecl(name.clone(), Box::new(e))),
            Box::new(e2),
        )
    }
    / "let var " name:id() " = " e:expr() {
        Expr::Let(
            Box::new(Decl::VarDecl(name.clone(), Box::new(e))),
            Box::new(Expr::Id(name.clone())),
        )
    }

    // *
    rule op2() -> Expr
        = a:atom() "*" b:op2() {
            Expr::Times(Box::new(a), Box::new(b))
        }
        / a:atom() {
            a
        }

    //+ and -
    rule op1() -> Expr
        = a:op2() "+" b:op1() {
            Expr::Plus(Box::new(a), Box::new(b))
        }
        / a:op2() "-" b:op1() {
            Expr::Minus(Box::new(a), Box::new(b))
        }
        / a:op2() {
            a
        }

    rule expr_inner() -> Expr
        = "let" d:decl() "in" e:expr() {
            Expr::Let(Box::new(d), Box::new(e))
        }
        / o:op1() {
            o
        }

  /* Implement a parser for (all the) declarations.
     You are allowed to define and call your own helpers if you prefer.
     Both expr() and decl() will call each other.
     Staff solution length: 3 lines, no new helpers.
     Depending on your approach, your "expr" could be shorter and your "decl"
     could be longer, with different numbers of helpers for each.
     ) */
    pub rule decl() -> Decl
        = "var " name:id() " = " e:expr() {
            Decl::VarDecl(name, Box::new(e))
    }
    / "function " name:id() "(" args:non_empty_arg_list_strings() ")" "{" body:expr() "}" {
            Decl::FunDecl(name, args, Box::new(body))
    }
    / "let " d:decl() " in " e:expr() {
            let temp_name = String::from("tempVar");
            Decl::VarDecl(temp_name, Box::new(e))
    }
    //non empty for EXPR
    rule non_empty_arg_list() -> Vec<Expr>
        = first:expr() "," rest:non_empty_arg_list() {
            let mut args = vec![first];
            args.extend(rest);
            args
        }
        / first:expr() {
            vec![first]
        }

    //non empty for STRINGs
    rule non_empty_arg_list_strings() -> Vec<String>
        = first:id() "," rest:non_empty_arg_list_strings() {
            let mut args = vec![first];
            args.extend(rest);
            args
        }
        / first:id() {
            vec![first]
        }

    rule function_call() -> Expr
        = name:id() "(" args:non_empty_arg_list() ")" {
            Expr::FunCall(name, args)
        }

    rule atom() -> Expr
        = n:numeral() {
            n
        }
        / v:var() {
            v
        }
        / f:function_call() {
            f
    }
  }
}
