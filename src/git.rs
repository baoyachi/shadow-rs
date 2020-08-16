use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::err::*;
use std::cell::RefCell;
use std::collections::HashMap;

const BRANCH: ShadowConst = "BRANCH";
const COMMIT_HASH: ShadowConst = "COMMIT_HASH";
const COMMIT_DATE: ShadowConst = "COMMIT_DATE";
const COMMIT_AUTHOR: ShadowConst = "COMMIT_AUTHOR";
const COMMIT_EMAIL: ShadowConst = "COMMIT_EMAIL";

#[derive(Default, Debug)]
pub struct Git {
    map: HashMap<ShadowConst, RefCell<ConstVal>>,
}

impl Git {
    pub(crate) fn new(path: &std::path::Path) -> HashMap<ShadowConst, RefCell<ConstVal>> {
        let mut git = Git::default();
        git.map
            .insert(BRANCH, ConstVal::new("display current branch"));
        git.map
            .insert(COMMIT_HASH, ConstVal::new("display current commit_id"));
        git.map.insert(
            COMMIT_AUTHOR,
            ConstVal::new("display current commit author"),
        );
        git.map
            .insert(COMMIT_EMAIL, ConstVal::new("display current commit email"));
        git.map
            .insert(COMMIT_DATE, ConstVal::new("display current commit date"));

        if let Err(e) = git.init(path) {
            println!("{}", e.to_string());
        }

        git.map
    }

    fn init(&mut self, path: &std::path::Path) -> SdResult<()> {
        let repo = git2::Repository::discover(path)?;
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

        update_val(COMMIT_DATE, commit.time().seconds().to_string());

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