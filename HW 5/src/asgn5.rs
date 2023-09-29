/** In this assignment, you represent and analyze data
 * from the System Usability Scale (SUS).
 * Done by Joe Colley 9/29/23 - 10/10/23
 */
/* BEGIN STARTER CODE */
#[derive(PartialEq, Eq, Debug)]
pub enum Agreement {
    StronglyDisagree,
    SomewhatDisagree,
    Neither,
    SomewhatAgree,
    StronglyAgree,
}

pub type SurveyResults = Vec<Agreement>;

pub const SURVEY_LENGTH: i32 = 10;

/* END STARTER CODE */

/* You  write the rest*/

/* For each agreement value, return the numeric score associated with it,
*  as in a standard Likert scale calculation */
/** Staff solution length: 9 lines */
pub fn agreement_to_score(a: &Agreement) -> i32 {
    match a {
        Agreement::StronglyDisagree => -2,
        Agreement::SomewhatDisagree => -1,
        Agreement::Neither => 0,
        Agreement::SomewhatAgree => 1,
        Agreement::StronglyAgree => 2,
    }
}

/* For each index i in the range 0-9, return Some(txt) where txt is
 * the complete text of the prompt for question i in the System Usability Scale
 * Use Wikipedia as a source, to ensure consistent wording and formatting.
 * Include a terminating period in each question.
 * https://en.wikipedia.org/wiki/System_usability_scale
 * For values outside the range 0-9, your function can do anything
 */
/** Staff solution length: 14 lines*/
pub fn index_to_prompt(i: i32) -> Option<String> {
    match i {
        0 => Some("I think that I would like to use this system frequently.".to_string()),
        1 => Some("I found the system unnecessarily complex.".to_string()),
        2 => Some("I thought the system was easy to use.".to_string()),
        3 => Some("I think that I would need the support of a technical person to be able to use this system.".to_string()),
        4 => Some("I found the various functions in this system were well integrated.".to_string()),
        5 => Some("I thought there was too much inconsistency in this system.".to_string()),
        6 => Some("I would imagine that most people would learn to use this system very quickly.".to_string()),
        7 => Some("I found the system very cumbersome to use.".to_string()),
        8 => Some("I felt very confident using the system.".to_string()),
        9 => Some("I needed to learn a lot of things before I could get going with this system.".to_string()),
        _ => None,
    }
}

/** For each index i 0..9, return an integer indicating whether question i of
 * the System Usability Scale is positive keyed (return 1) vs. negative keyed
 * (return -1). Outside the range 0..9, your function can do anything. */
/** Staff solution length: 14 lines (though a 1-line solution exists) */
pub fn index_to_key(i: i32) -> i32 {
    match i {
        0 | 2 | 4 | 6 | 8 => 1,
        1 | 3 | 5 | 7 | 9 => -1,
        _ => 0,
    }
}

/** Assume data is a 10-element vector containing a complete set of responses
 * to the System Usability Scale. Compute the acquiescence bias of the
 * responses, using the formula provided in lecture.  We recommend using
 * agreement_to_score as a helper function.
 */
/** Staff solution length: 8 lines */
pub fn acquiescence_bias(data: &Vec<Agreement>) -> f64 {
    let sum: i32 = data.iter().map(|a| agreement_to_score(a)).sum();
    sum as f64 / data.len() as f64
}

/** Assume data is a 10-element vector containing a complete set of responses
 * to the System Usability Scale. Compute the final score, i.e., scale value,
 * using the formula provided in lecture.  We recommend using
 * agreement_to_score and index_to_key as helper functions.
 */
/** Staff solution length: 9 lines */

//have to wait as the lecutre hasnt happened yet
pub fn score(data: &Vec<Agreement>) -> f64 {
    420.0
} // return dummy value so it compiles
