use crate::err::SdResult;
use std::path::Path;

#[derive(Default, Debug)]
pub struct Git {
    tag: String,
    git_version: String,
    branch: String,
    commit_hash: String,
    short_commit_hash: String,
    commit_date: String,
    author_name: String,
    author_email: String,
}

impl Git {
    pub fn new<P: AsRef<Path>>(path: P) -> SdResult<Git> {
        let repo = git2::Repository::open(path)?;
        let reference = repo.head()?;

        let branch = reference.shorthand();
        let commit_id = reference.target();
        let commit = reference.peel_to_commit()?;
        let author = commit.author();
        let commit_date = commit.time().seconds();
        let author_name = author.name();
        let author_email = author.email();

        let mut desc_opt = git2::DescribeOptions::new();
        desc_opt.describe_tags().show_commit_oid_as_fallback(true);
        let tag = repo
            .describe(&desc_opt)
            .and_then(|desc| desc.format(None))
            .unwrap();

        Ok(Git {
            tag: tag.to_string(),
            git_version: "".to_string(),
            short_commit_hash: "".to_string(),

            branch: branch.unwrap().to_string(),
            commit_hash: commit_id.unwrap().to_string(),
            commit_date: commit_date.to_string(),
            author_name: author_name.unwrap().to_string(),
            author_email: author_email.unwrap().to_string(),
        })
    }
}