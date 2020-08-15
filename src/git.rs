use crate::build::{ConstVal, ConstType, ShadowGen, ShadowConst};
use crate::err::*;
use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

const BRANCH: ShadowConst = "BRANCH";
const COMMIT_HASH: ShadowConst = "COMMIT_HASH";
const COMMIT_DATE: ShadowConst = "COMMIT_DATE";
const COMMIT_AUTHOR: ShadowConst = "COMMIT_AUTHOR";
const COMMIT_EMAIL: ShadowConst = "COMMIT_EMAIL";

#[derive(Default, Debug)]
struct Git {
    git_path: String,
    map: HashMap<ShadowConst, RefCell<ConstVal>>,
}

impl Git {
    fn new(path: String) -> Self {
        let mut git = Git { git_path: path, map: HashMap::new() };
        git.map.insert(BRANCH, ConstVal::new("display current branch"));
        git.map.insert(COMMIT_HASH, ConstVal::new("display current commit_id"));
        git.map.insert(COMMIT_AUTHOR, ConstVal::new("display current commit author"));
        git.map.insert(COMMIT_EMAIL, ConstVal::new("display current commit email"));
        git.map.insert(COMMIT_DATE, ConstVal::new("display current commit date"));
        git
    }

    fn init(&mut self) -> SdResult<()> {
        let repo = git2::Repository::open(&self.git_path)?;
        let reference = repo.head()?;

        let update_val = |c: ShadowConst, v: String| {
            if let Some(c) = self.map.get(c) {
                let mut val = c.borrow_mut();
                val.t = ConstType::Str;
                val.v = v.to_string();
            }
        };

        if let Some(v) = reference.shorthand() {
            update_val(BRANCH, v.to_string());
        }

        if let Some(v) = reference.target() {
            update_val(COMMIT_HASH, v.to_string());
        }

        let commit = reference.peel_to_commit()?;

        update_val(COMMIT_DATE,commit.time().seconds().to_string());

        let author = commit.author();
        if let Some(v) = author.email() {
            update_val(COMMIT_EMAIL, v.to_string());
        }

        if let Some(v) = author.name() {
            update_val(COMMIT_AUTHOR, v.to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let mut git = Git::new("./".to_string());
        git.init();
        println!("git2:{:?}", git);
    }
}