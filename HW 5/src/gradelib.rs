pub mod gradelib {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn test_display<T: Eq + Debug>(score: f64, e1: T, e2: T, err: &str) -> f64 {
        if e1 != e2 {
            println!(
                "failed test {}\nGot: {:?}\nExpected: {:?}\n",
                err.to_string(),
                e1,
                e2
            );
            0.0
        } else {
            score
        }
    }

    // inexact comparison
    pub fn test_real(score: f64, e1: f64, e2: f64, err: &str) -> f64 {
        let epsilon = 0.0001;
        if (e1 - e2).abs() > epsilon {
            println!(
                "failed test {}\nGot: {:?}\nExpected: {:?}\n",
                err.to_string(),
                e1,
                e2
            );
            0.0
        } else {
            score
        }
    }

    pub fn test<T: Eq>(score: f64, e1: T, e2: T, err: &str) -> f64 {
        if e1 != e2 {
            println!("{}", err.to_string());
            0.0
        } else {
            score
        }
    }

    pub fn grade_str(g: f64) -> String {
        format!("{{\"score\": {},\"output\":\"Grader ran successfully\",\"stdout_visibility\":\"visible\"}}", g)
    }

    pub fn record_grade(g: f64) -> () {
        let path = Path::new("autograder/results/results.json");
        let mut file = match File::create(&path) {
            Err(_) => panic!("file"),
            Ok(f) => f,
        };
        match file.write_all(grade_str(g).as_bytes()) {
            Err(_why) => panic!("couldn't write file"),
            Ok(_) => println!("wrote json file"),
        }
    }
}
