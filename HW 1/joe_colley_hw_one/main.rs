/** You do not need to modify this file. You only modify asgn1.rs.
 * If you wish to debug your code without submitting it to Gradescope, you
 * can run this file to run the testing code. The testing code is provided
 * in this file for help with debugging.
 */
pub mod gradelib;
pub mod asgn1;
use std::fmt::Display;

use asgn1::asgn1::IntMap as im;
fn test_display<T:Eq + Display>(score:f64, e1:T,e2:T,err:&str) -> f64 {
    if e1 != e2 {
        println!("failed test {}\nExpected: {}\nGot: {}\n", err.to_string(),e1.to_string(),e2.to_string());
        0.0
    } else {
        score
    }
}


fn test<T:Eq>(score:f64, e1:T,e2:T,err:&str) -> f64 {
    if e1 != e2 {
        println!("{}", err.to_string());
        0.0
    } else {
        score
    }
}


fn main() {
    // hello world 5pt
    let r1 = test(5.0,(),asgn1::asgn1::hello_world(),
    "The hello world \"test case\" is trivial and should never fail, but it did. Something weird happened.");
    // array basics 15pt
    let r2 = test_display(2.0, 2, asgn1::asgn1::get_third(&[1,5,2]), 
    "get_third(&[1,5,2]):");
    let r3 = test_display(2.0, 9, asgn1::asgn1::get_third(&[8,7,9]), 
    "get_third(&[8,7,9]");
    let r4 = test(2.0, vec![3,9,27], asgn1::asgn1::powers(3),
     "test case powers(3) failed: Expected [3,9,27], got something else");
    let r5 = test(2.0, vec![-1,1,-1], asgn1::asgn1::powers(-1), 
     "test case powers(-1) failed: Expected [-1,1,-1], got something else");
    let r6 = test(2.0, vec![0,0,0], asgn1::asgn1::powers(0), 
     "test case powers(0) failed: Expected [0,0,0], got something else");
    let mut sq1 = [0,0,0];
    let r7 = test(2.0, [0,0,0], {asgn1::asgn1::square_array(&mut sq1); sq1}, 
    "test case square_array(&[0,0,0]) failed: Expected [0,0,0], got something else");
    let mut sq2: [i64;0] = [];
    let r8 = test(1.0, [], {asgn1::asgn1::square_array(&mut sq2); sq2}, 
    "test case square_array(&[]) failed: Expected [], got something else");
    let mut sq3 = [-1,2,1];
    let r9 = test(2.0, [1,4,1], {asgn1::asgn1::square_array(&mut sq3); sq3}, "test case square_arry(&[-1,2,1]) failed: Expected [1,4,1], got something else");
    // loops 30pt
    let mut rev = [-1,2,-3,4,-5,6,-7,8,-9,0];
    let r10 = test(10.0, [0,-9,8,-7,6,-5,4,-3,2,-1], {asgn1::asgn1::reverse_array(&mut rev); rev},  "test case reverse_array(&[0,-9,8,-7,6,-5,4,-3,2,-1]) failed: Expected [-1,2,-3,4,-5,6,-7,8,-9,0], got something else");
    let r11 = test_display(2.0, 0, asgn1::asgn1::sum_to_index(0), 
    "sum_to_index(0)");
    let r12 = test_display(2.0, 1, asgn1::asgn1::sum_to_index(1), "sum_to_index(1)");
    let r13 = test_display(2.0, 5, asgn1::asgn1::sum_to_index(2), "sum_to_index(2)");
    let r14 = test_display(2.0, 14, asgn1::asgn1::sum_to_index(3), "sum_to_index(3)");
    let r15 = test_display(2.0, 30, asgn1::asgn1::sum_to_index(4), "sum_to_index(4)");
    let r16 = test_display(2.0, 0, asgn1::asgn1::sum_until_zero(&[]), "sum_until_zero(&[])");
    let r17 = test_display(2.0, 0, asgn1::asgn1::sum_until_zero(&[0]), "sum_until_zero(&[0])");
    let r18 = test_display(2.0, 5, asgn1::asgn1::sum_until_zero(&[1,-2,6]), "sum_until_zero(&[1,-2,6])");
    let r19 = test_display(2.0, 1, asgn1::asgn1::sum_until_zero(&[1,0,6]), "sum_until_zero(&[1,0,6])");
    let r20 = test_display(2.0, -1, asgn1::asgn1::sum_until_zero(&[1,-2,0,6]), "sum_until_zero(&[1,-2,0,6])");
    let r21 = test_display(2.0, -1, asgn1::asgn1::bsearch(Box::new(im::Empty), 3), "bsearch(E)");
    let r22 = test_display(2.0, 5, asgn1::asgn1::bsearch(Box::new(im::Node(Box::new(im::Empty), 2, 5, Box::new(im::Empty))), 2), "bsearch(N(E,2,5,E), 2)");
    let r23 = test_display(2.0, -3, asgn1::asgn1::bsearch(
        Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty), 1, -3, Box::new(im::Empty)))
        , 2, 5, Box::new(im::Node(Box::new(im::Empty), 8, 18, Box::new(im::Empty))))), 1), 
        "bsearch(N(N(E,1,-3,E),2,5,N(E,8,18,E)), 1)");
    let r24 = test_display(2.0, 18, asgn1::asgn1::bsearch(
        Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty), 1, -3, Box::new(im::Empty)))
        , 2, 5, Box::new(im::Node(Box::new(im::Empty), 8, 18, Box::new(im::Empty))))), 8), 
        "bsearch(N(N(E,1,-3,E),2,5,N(E,8,18,E)),8)");
    let r25 = test_display(2.0, -1, asgn1::asgn1::bsearch(
        Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty), 1, -3, Box::new(im::Empty)))
        , 2, 5, Box::new(im::Node(Box::new(im::Empty), 8, 18, Box::new(im::Empty))))), 0), 
        "bsearch(N(N(E,1,-3,E),2,5,N(E,8,18,E)), 0)");
    let r26 = test_display(3.0, -1, asgn1::asgn1::bsearch(
        Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty), 1, -3, Box::new(im::Empty)))
        , 2, 5, Box::new(im::Node(Box::new(im::Empty), 8, 18, Box::new(im::Empty))))), 10),
         "bsearch(N(N(E,1,-3,E),2,5,N(E,8,18,E)), 10)");
    let i1res = Box::new(im::Node(Box::new(im::Empty), 1, -3, Box::new(im::Empty)));
    let r27 = test(3.0, i1res, 
    asgn1::asgn1::insert(
        Box::new(im::Empty), 1, -3),
        "test case insert(E,1,-3) failed: Expected (N(E,1,-3,E), got something else");
    let i2in =Box::new(im::Node(Box::new(im::Empty), 2, 4, Box::new(im::Empty)));
    let i2res =Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty),1,99,Box::new(im::Empty))), 2, 4, Box::new(im::Empty)));
    let i3res =Box::new(im::Node(Box::new(im::Empty), 2, 4, Box::new(im::Node(Box::new(im::Empty),8,5,Box::new(im::Empty)))));
    let r28 = test(3.0, i2res, 
        asgn1::asgn1::insert(
            i2in.clone(), 1, 99),
            "test case insert(N(E,2,4,E), 1, 99) failed: expected N(N(E,1,99,E),2,4,E), got something else");
    let r29 = test(3.0, i3res.clone(), 
        asgn1::asgn1::insert(
            i2in, 8, 5),
            "test case insert(N(E,2,4,E), 8, 5) failed: expected N(E,2,4,N(E,8,5,E))), got something else");
    let i4res =Box::new(im::Node(Box::new(im::Empty), 2, 4, Box::new(im::Node(Box::new(im::Node(Box::new(im::Empty),7,6,Box::new(im::Empty))),8,5,Box::new(im::Empty)))));
    let r30 = test(3.0, i4res, 
        asgn1::asgn1::insert(
            i3res, 7, 6),
            "test case insert(N(E,2,4,N(E,8,5,E)), 7, 6) failed: expected N(E,2,4,N(N(E,7,6,E),8,5,E))), got something else");
    let score = r1+r2+r3+r4+r5+r6+r7+r8+r9+r10+r11+r12+r13+r14+r15+r16+r17+r18+r19+r20+r21+r22+r23+r24+r25+r26+r27+r28+r29+r30;
    crate::gradelib::gradelib::record_grade(score);
}
