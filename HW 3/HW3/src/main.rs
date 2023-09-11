pub mod gradelib;
pub mod asgn3;
use asgn3::*;
use asgn3::Expr as ex;
use asgn3::Defn as dc;
use asgn3::Value as val;
use rpds::HashTrieMap;

fn test_expr_internal(score: f64, in_env: HashTrieMap<String,EnvRecord>, in_e: Expr, out: Value) -> f64 {
    match (asgn3::eval_expr(&in_env, &in_e), out) {
      (e1, e2) =>
        if e1 == e2 {
          return score
        } else {
          {println!("Expression test case {:?}; {:?} expected expr {:?} but got {:?}", in_env,in_e, e2,e1); }
          return 0.0
        },      
    }
  }

  fn test_defn_internal(score: f64, in_env: HashTrieMap<String,EnvRecord>, in_d: Defn, out: HashTrieMap<String,EnvRecord>) -> f64 {
    match (asgn3::eval_defn(&in_env, &in_d), out) {
      (e1, e2) =>
        if e1 == e2 {
          return score
        } else {
          {println!("Definition test case {:?}; {:?} expected output environment {:?} but got {:?}", in_env,in_d, e2,e1); }
          return 0.0
        },      
    }
  }

  fn test_expr(score: f64, in_e: Expr, out: Value) -> f64 {
    test_expr_internal(score, HashTrieMap::new(), in_e, out)
  }
  fn test_defn(score: f64,  in_d: Defn, out: HashTrieMap<String,EnvRecord>) -> f64 {
    test_defn_internal(score, HashTrieMap::new(), in_d, out)
  }

fn main() {
    let r1 = test_expr(5.0, ex::Numeral(3), val::Numeral(3));
    let r2 = test_expr(5.0, ex::Times(Box::new(ex::Numeral(3)),Box::new(ex::Numeral(5))), val::Numeral(15));
    let r3 = test_expr(5.0, ex::Plus(Box::new(ex::Numeral(3)),Box::new(ex::Numeral(5))), val::Numeral(8));
    let r4 = test_expr(5.0, ex::Minus(Box::new(ex::Numeral(3)),Box::new(ex::Numeral(5))), val::Numeral(-2));
    let r5 = test_expr(5.0, ex::Let(Box::new(dc::VarDefn("x".to_string(), Box::new(ex::Numeral(10)))),
        Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string()))))), val::Numeral(100));
    let r6 = test_expr(5.0, ex::Minus(Box::new(ex::Let(Box::new(dc::VarDefn("x".to_string(), Box::new(ex::Numeral(10)))),
    Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))),Box::new(ex::Let(Box::new(dc::VarDefn("x".to_string(), Box::new(ex::Numeral(20)))),
    Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string()))))))), val::Numeral(-300));
    let r7 = test_defn(5.0, dc::VarDefn("x".to_string(), Box::new(ex::Numeral(2))), HashTrieMap::new().insert("x".to_string(), EnvRecord::VarRecord(val::Numeral(2))));
    let r8 = test_defn(5.0, dc::VarDefn("z".to_string(), Box::new(ex::Plus(Box::new(ex::Numeral(2)),Box::new(ex::Numeral(2))))), HashTrieMap::new().insert("z".to_string(), EnvRecord::VarRecord(val::Numeral(4))));
    let r_nofun = r1+r2+r3+r4+r5+r6+r7+r8;
    
    let r9 = test_expr(5.0, ex::Let(Box::new(dc::FunDefn("sq".to_string(), vec!["x".to_string()], 
          Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))),Box::new(ex::Call("sq".to_string(), vec![ex::Numeral(4)]))),
        val::Numeral(16));
    let r10 = test_expr(5.0, ex::Let(Box::new(dc::FunDefn("sq".to_string(), vec!["x".to_string()], 
          Box::new(ex::Times(Box::new(ex::Id("x".to_string())),Box::new(ex::Id("x".to_string())))))),
          Box::new(ex::Call("sq".to_string(), vec![ex::Call("sq".to_string(), vec![ex::Numeral(4)])]))),
        val::Numeral(256));
    let r11 = test_expr(5.0, ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["a".to_string()],
      Box::new(ex::Let(Box::new(dc::FunDefn("g".to_string(), vec!["b".to_string()],
        Box::new(ex::Minus(Box::new(ex::Numeral(0)),Box::new(ex::Id("b".to_string())))))),
        Box::new(ex::Call("g".to_string(), vec![ex::Times(Box::new(ex::Numeral(2)),Box::new(ex::Id("a".to_string())))])))))),
        Box::new(ex::Call("f".to_string(), vec![ex::Numeral(5)]))), val::Numeral(-10));
    let r12 = test_expr(5.0,ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["a".to_string()], Box::new(ex::Plus(Box::new(ex::Id("a".to_string())), Box::new(ex::Numeral(2)))))),
      Box::new(ex::Times(Box::new(ex::Call("f".to_string(),vec![ex::Numeral(3)])),Box::new(ex::Call("f".to_string(),vec![ex::Numeral(5)]))))), val::Numeral(35));
    let r13 = test_expr(5.0, ex::Times(Box::new(ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["c".to_string()], Box::new(ex::Plus(Box::new(ex::Numeral(5)),Box::new(ex::Id("c".to_string())))))),  Box::new(ex::Call("f".to_string(), vec![ex::Numeral(5)])))),Box::new(ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["c".to_string()], Box::new(ex::Plus(Box::new(ex::Numeral(5)),Box::new(ex::Id("c".to_string())))))),  Box::new(ex::Call("f".to_string(), vec![ex::Numeral(10)]))))), val::Numeral(150));
    let r14 = test_expr(5.0, 
    ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["c".to_string()], 
    Box::new(ex::Plus(Box::new(ex::Numeral(5)),Box::new(ex::Id("c".to_string())))))),  
    Box::new(ex::Call("f".to_string(), vec![ex::Call("f".to_string(), vec![ex::Call("f".to_string(), vec![ex::Numeral(10)])])]))),
     val::Numeral(25));
    let r15 = test_expr(5.0, ex::Let(Box::new(dc::FunDefn("f".to_string(), vec!["a".to_string()],
          Box::new(ex::Times(Box::new(ex::Numeral(2)),Box::new(ex::Id("a".to_string())))))),Box::new(
          ex::Let(Box::new(dc::FunDefn("g".to_string(), vec!["b".to_string()],
            Box::new(ex::Minus(Box::new(ex::Numeral(0)),Box::new(ex::Id("b".to_string())))))), 
            Box::new(ex::Call("f".to_string(), vec![ex::Call("g".to_string(), vec![ex::Numeral(2)])]))))
          ), val::Numeral(-4));
    let r_fun = r9+r10+r11+r12+r13+r14+r15;
    let score = r_nofun + r_fun;
    gradelib::gradelib::record_grade(score);
  }
  