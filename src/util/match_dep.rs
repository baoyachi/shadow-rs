fn regex(input: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_registry() {
        let input = "shadow-rs v2.0.10 (registry `ssh://git@github.com/baoyachi/shadow-rs.git`)";
    }

    #[test]
    fn test_regex_path() {
        let input = "shadow-rs v0.5.23 (/Users/baoyachi/shadow-rs)";
    }

    #[test]
    fn test_regex_git() {
        let input = "shadow-rs v0.5.23 (https://github.com/baoyachi/shadow-rs#13572c90)";
    }
}