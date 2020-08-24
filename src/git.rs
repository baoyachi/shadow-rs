use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::ci::CIType;
use crate::err::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use git2::Reference;
use std::cell::RefCell;
use std::collections::HashMap;

const BRANCH: ShadowConst = "BRANCH";
const SHORT_COMMIT: ShadowConst = "SHORT_COMMIT";
const COMMIT_HASH: ShadowConst = "COMMIT_HASH";
const COMMIT_DATE: ShadowConst = "COMMIT_DATE";
const COMMIT_AUTHOR: ShadowConst = "COMMIT_AUTHOR";
const COMMIT_EMAIL: ShadowConst = "COMMIT_EMAIL";

#[derive(Default, Debug)]
pub struct Git {
    map: HashMap<ShadowConst, RefCell<ConstVal>>,
    ci_type: CIType,
}

impl Git {
    fn init(&mut self, path: &std::path::Path, std_env: &HashMap<String, String>) -> SdResult<()> {
        let repo = git2::Repository::discover(path)?;
        let reference = repo.head()?;

        let update_val = |c: ShadowConst, v: String| {
            if let Some(c) = self.map.get(c) {
                let mut val = c.borrow_mut();
                val.t = ConstType::Str;
                val.v = v;
            }
        };

        update_val(BRANCH, self.get_branch(&reference, std_env));

        if let Some(v) = reference.target() {
            let commit = v.to_string();
            update_val(COMMIT_HASH, commit.clone());
            let mut short_commit = commit.as_str();

            if commit.len() > 8 {
                short_commit = &short_commit[0..8];
            }
            update_val(SHORT_COMMIT, short_commit.to_string());
        }

        let commit = reference.peel_to_commit()?;

        let time_stamp = commit.time().seconds().to_string().parse::<i64>()?;
        let dt = NaiveDateTime::from_timestamp(time_stamp, 0);
        let date_time = DateTime::<Utc>::from_utc(dt, Utc);
        update_val(COMMIT_DATE, date_time.to_rfc3339());

        let author = commit.author();
        if let Some(v) = author.email() {
            update_val(COMMIT_EMAIL, v.to_string());
        }

        if let Some(v) = author.name() {
            update_val(COMMIT_AUTHOR, v.to_string());
        }

        Ok(())
    }

    fn get_branch(&self, reference: &Reference<'_>, std_env: &HashMap<String, String>) -> String {
        let mut branch = "";
        if let Some(v) = reference.shorthand() {
            branch = v;
        }
        match self.ci_type {
            CIType::Gitlab => {
                if let Some(v) = std_env.get("CI_COMMIT_REF_NAME") {
                    branch = v;
                }
            }
            CIType::Github => {
                if let Some(v) = std_env.get("CI_COMMIT_REF_NAME") {
                    branch = v;
                }
            }
            _ => {}
        }

        branch.to_string()
    }
}

pub fn new_git(
    path: &std::path::Path,
    ci: CIType,
    std_env: &HashMap<String, String>,
) -> HashMap<ShadowConst, RefCell<ConstVal>> {
    let mut git = Git {
        map: Default::default(),
        ci_type: ci,
    };
    git.map
        .insert(BRANCH, ConstVal::new("display current branch"));
    git.map
        .insert(COMMIT_HASH, ConstVal::new("display current commit_id"));

    git.map.insert(
        SHORT_COMMIT,
        ConstVal::new("display current short commit_id"),
    );

    git.map.insert(
        COMMIT_AUTHOR,
        ConstVal::new("display current commit author"),
    );
    git.map
        .insert(COMMIT_EMAIL, ConstVal::new("display current commit email"));
    git.map
        .insert(COMMIT_DATE, ConstVal::new("display current commit date"));

    if let Err(e) = git.init(path, std_env) {
        println!("{}", e.to_string());
    }

    git.map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_git() {
        let map = new_git(Path::new("./"), CIType::Github);
        println!("map:{:?}", map);
    }
}
