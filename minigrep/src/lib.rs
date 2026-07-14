//! Functions containing the necessary logic for searching
//! for a given string in a multi-line string.

/// Searches for the given `query` in all the lines of `contents`.
///
/// # Examples
///
/// ```
/// let query = "fox";
/// let contents = "\
/// The quick
/// brown fox jumps over
/// the lazy dog.";
///
/// let result = minigrep::search(query, contents).collect::<Vec<&str>>();
///
/// assert_eq!(result, vec!["brown fox jumps over"]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).collect::<Vec<&str>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }
}
