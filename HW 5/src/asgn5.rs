/** In this assignment, you represent and analyze data
 * from the System Usability Scale (SUS).
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
    0
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
    None
}

/** For each index i 0..9, return an integer indicating whether question i of
 * the System Usability Scale is positive keyed (return 1) vs. negative keyed
 * (return -1). Outside the range 0..9, your function can do anything. */
/** Staff solution length: 14 lines (though a 1-line solution exists) */
pub fn index_to_key(i: i32) -> i32 {
    0
}

/** Assume data is a 10-element vector containing a complete set of responses
 * to the System Usability Scale. Compute the acquiescence bias of the
 * responses, using the formula provided in lecture.  We recommend using
 * agreement_to_score as a helper function.
 */
/** Staff solution length: 8 lines */
pub fn acquiescence_bias(data: &Vec<Agreement>) -> f64 {
    420.0
} // return dummy value so it compiles

/** Assume data is a 10-element vector containing a complete set of responses
 * to the System Usability Scale. Compute the final score, i.e., scale value,
 * using the formula provided in lecture.  We recommend using
 * agreement_to_score and index_to_key as helper functions.
 */
/** Staff solution length: 9 lines */
pub fn score(data: &Vec<Agreement>) -> f64 {
    420.0
} // return dummy value so it compiles
