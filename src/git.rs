#[derive(Default, Debug)]
struct Git {
    tag: String,
    branch: String,
    commit_hash: String,
    short_commit_hash: String,
    git_version: String,
    commit_date: String,
    contributor: String,
}

impl Git {
    fn new() -> Git {
        let repo = git2::Repository::open("./").unwrap();
        let reference = repo.head().unwrap();
        let branch = reference.shorthand();
        let commit_id = reference.target();

        Git::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git() {
        let repo = git2::Repository::open("./").unwrap();
        let reference = repo.head().unwrap();
        println!("{:?}", reference.is_tag());
    }
}
