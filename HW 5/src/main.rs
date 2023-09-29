pub mod asgn5;
pub mod gradelib;

use asgn5::*;
use gradelib::gradelib::*;
/*  pub fn agreement_to_score(a: Agreement) -> i32 {
  pub fn index_to_prompt(i: i32) -> Option<String> {
  pub fn index_to_key(i: i32) -> i32 {
  pub fn acquiescence_bias(data: &Vec<f64>) -> f64 {
  pub fn score(data: &Vec<f64>) -> f64 {
*/

fn main() {
    let r_ats0 = test_display(
        3.0,
        asgn5::agreement_to_score(&Agreement::StronglyDisagree),
        -2,
        "agreement_to_score",
    );
    let r_ats1 = test_display(
        3.0,
        asgn5::agreement_to_score(&Agreement::SomewhatDisagree),
        -1,
        "agreement_to_score",
    );
    let r_ats2 = test_display(
        3.0,
        asgn5::agreement_to_score(&Agreement::Neither),
        0,
        "agreement_to_score",
    );
    let r_ats3 = test_display(
        3.0,
        asgn5::agreement_to_score(&Agreement::SomewhatAgree),
        1,
        "agreement_to_score",
    );
    let r_ats4 = test_display(
        3.0,
        asgn5::agreement_to_score(&Agreement::StronglyAgree),
        2,
        "agreement_to_score",
    );
    let r_agreement = r_ats0 + r_ats1 + r_ats2 + r_ats3 + r_ats4;
    let ri0 = test_display(
        1.0,
        asgn5::index_to_prompt(0),
        Some("I think that I would like to use this system frequently.".to_string()),
        "index_to_prompt",
    );
    let ri1 = test_display(
        2.0,
        asgn5::index_to_prompt(1),
        Some("I found the system unnecessarily complex.".to_string()),
        "index_to_prompt",
    );
    let ri2 = test_display(
        1.0,
        asgn5::index_to_prompt(2),
        Some("I thought the system was easy to use.".to_string()),
        "index_to_prompt",
    );
    let ri3 = test_display(2.0, asgn5::index_to_prompt(3), Some("I think that I would need the support of a technical person to be able to use this system.".to_string()), "index_to_prompt");
    let ri4 = test_display(
        1.0,
        asgn5::index_to_prompt(4),
        Some("I found the various functions in this system were well integrated.".to_string()),
        "index_to_prompt",
    );
    let ri5 = test_display(
        2.0,
        asgn5::index_to_prompt(5),
        Some("I thought there was too much inconsistency in this system.".to_string()),
        "index_to_prompt",
    );
    let ri6 = test_display(
        1.0,
        asgn5::index_to_prompt(6),
        Some(
            "I would imagine that most people would learn to use this system very quickly."
                .to_string(),
        ),
        "index_to_prompt",
    );
    let ri7 = test_display(
        2.0,
        asgn5::index_to_prompt(7),
        Some("I found the system very cumbersome to use.".to_string()),
        "index_to_prompt",
    );
    let ri8 = test_display(
        1.0,
        asgn5::index_to_prompt(8),
        Some("I felt very confident using the system.".to_string()),
        "index_to_prompt",
    );
    let ri9 = test_display(
        2.0,
        asgn5::index_to_prompt(9),
        Some(
            "I needed to learn a lot of things before I could get going with this system."
                .to_string(),
        ),
        "index_to_prompt",
    );
    let r_prompt = ri0 + ri1 + ri2 + ri3 + ri4 + ri5 + ri6 + ri7 + ri8 + ri9;
    let rp0 = test_display(1.0, asgn5::index_to_key(0), 1, "index_to_key");
    let rp1 = test_display(2.0, asgn5::index_to_key(1), -1, "index_to_key");
    let rp2 = test_display(1.0, asgn5::index_to_key(2), 1, "index_to_key");
    let rp3 = test_display(2.0, asgn5::index_to_key(3), -1, "index_to_key");
    let rp4 = test_display(1.0, asgn5::index_to_key(4), 1, "index_to_key");
    let rp5 = test_display(2.0, asgn5::index_to_key(5), -1, "index_to_key");
    let rp6 = test_display(1.0, asgn5::index_to_key(6), 1, "index_to_key");
    let rp7 = test_display(2.0, asgn5::index_to_key(7), -1, "index_to_key");
    let rp8 = test_display(1.0, asgn5::index_to_key(8), 1, "index_to_key");
    let rp9 = test_display(2.0, asgn5::index_to_key(9), -1, "index_to_key");
    let r_key = rp0 + rp1 + rp2 + rp3 + rp4 + rp5 + rp6 + rp7 + rp8 + rp9;

    let responses_a = vec![
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
        Agreement::StronglyAgree,
    ];
    let responses_b = vec![
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
        Agreement::StronglyDisagree,
    ];
    let responses_c = vec![
        Agreement::SomewhatAgree,
        Agreement::SomewhatDisagree,
        Agreement::SomewhatAgree,
        Agreement::SomewhatDisagree,
        Agreement::SomewhatAgree,
        Agreement::SomewhatDisagree,
        Agreement::SomewhatAgree,
        Agreement::SomewhatDisagree,
        Agreement::SomewhatAgree,
        Agreement::SomewhatDisagree,
    ];

    let ra_0 = test_real(
        5.0,
        asgn5::acquiescence_bias(&responses_a),
        2.0,
        "acquiescence_bias",
    );
    let ra_1 = test_real(
        5.0,
        asgn5::acquiescence_bias(&responses_b),
        -2.0,
        "acquiescence_bias",
    );
    let ra_2 = test_real(
        5.0,
        asgn5::acquiescence_bias(&responses_c),
        0.0,
        "acquiescence_bias",
    );
    let r_acquiesce = ra_0 + ra_1 + ra_2;

    let rs_0 = test_real(5.0, asgn5::score(&responses_a), 0.0, "score");
    let rs_1 = test_real(5.0, asgn5::score(&responses_b), 0.0, "score");
    let rs_2 = test_real(5.0, asgn5::score(&responses_c), 1.0, "score");
    let r_score = rs_0 + rs_1 + rs_2;
    let grade = r_agreement + r_prompt + r_key + r_acquiesce + r_score;
    gradelib::gradelib::record_grade(grade);
}
