#[test]
fn one_result() {
    use crate::minigrep;

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
