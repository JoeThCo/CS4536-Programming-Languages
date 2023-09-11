pub mod gradelib {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    pub fn grade_str(g: f64) -> String {
        print!("= {} / 75\n", g);
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
            Ok(_) => println!("\nwrote json file"),
        }
    }
}
