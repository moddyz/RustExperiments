use crate::minigrep;

#[test]
fn one_result() {
    let pattern = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep::search(pattern, contents)
    );
}

#[test]
fn case_sensitive() {
    let pattern = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        minigrep::search(pattern, contents)
    );
}

#[test]
fn case_insensitive() {
    let pattern = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        minigrep::search_case_insensitive(pattern, contents)
    );
}
