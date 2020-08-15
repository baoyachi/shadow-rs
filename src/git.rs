use crate::err::SdResult;
use std::path::Path;
use crate::build::{ConstMessage, ConstType};

#[derive(Default, Debug)]
pub struct Git {
    tag: ConstMessage,
    git_version: ConstMessage,
    branch: ConstMessage,
    commit_hash: ConstMessage,
    short_commit_hash: ConstMessage,
    commit_date: ConstMessage,
    author_name: ConstMessage,
    author_email: ConstMessage,
}

const BRANCH: &str = "BRANCH";
const COMMIT_HASH: &str = "COMMIT_HASH";
const COMMIT_DATE: &str = "COMMIT_DATE";
const COMMIT_AUTHOR: &str = "COMMIT_AUTHOR";
const COMMIT_EMAIL: &str = "COMMIT_EMAIL";

impl Git {
    pub fn new<P: AsRef<Path>>(path: P) -> SdResult<Vec<ConstMessage>> {
        let mut vec = vec![];
        let repo = git2::Repository::open(path)?;
        let reference = repo.head()?;

        if let Some(v) = reference.shorthand() {
            vec.push(ConstMessage {
                desc: "display current branch".to_string(),
                key: BRANCH.into(),
                val: v.to_string(),
                t: ConstType::Str,
            });
        }

        if let Some(v) = reference.target() {
            vec.push(ConstMessage {
                desc: "display current commit_id".to_string(),
                key: COMMIT_HASH.into(),
                val: v.to_string(),
                t: ConstType::Str,
            });
        }

        let commit = reference.peel_to_commit()?;

        let author = commit.author();
        if let Some(v) = author.name() {
            vec.push(ConstMessage {
                desc: "display current commit author".to_string(),
                key: COMMIT_AUTHOR.into(),
                val: v.to_string(),
                t: ConstType::Str,
            });
        }

        if let Some(v) = author.email() {
            vec.push(ConstMessage {
                desc: "display current commit email".to_string(),
                key: COMMIT_EMAIL.into(),
                val: v.to_string(),
                t: ConstType::Str,
            });
        }

        vec.push(ConstMessage {
            desc: "display current commit date".to_string(),
            key: COMMIT_DATE.into(),
            val: commit.time().seconds().to_string(),
            t: ConstType::Str,
        });

        let mut desc_opt = git2::DescribeOptions::new();
        desc_opt.describe_tags().show_commit_oid_as_fallback(true);
        let tag = repo
            .describe(&desc_opt)
            .and_then(|desc| desc.format(None))
            .unwrap();

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git() {
        println!("{:?}",Git::new("./").unwrap());
    }
}