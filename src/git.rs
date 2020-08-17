use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::err::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::cell::RefCell;
use std::collections::HashMap;
use git2::Reference;
use crate::ci::CIType;

const BRANCH: ShadowConst = "BRANCH";
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
    pub(crate) fn new(path: &std::path::Path, ci: CIType) -> HashMap<ShadowConst, RefCell<ConstVal>> {
        let mut git = Git { map: Default::default(), ci_type: ci };
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

        update_val(BRANCH, self.get_branch(&reference));

        if let Some(v) = reference.target() {
            update_val(COMMIT_HASH, v.to_string());
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

    fn get_branch(&self, reference: &Reference<'_>) -> String {
        let mut branch = "";
        if let Some(v) = reference.shorthand() {
            branch = v;
        }
        match self.ci_type {
            CIType::Gitlab => {
                if let Some(v) = option_env!("CI_COMMIT_REF_NAME") {//GITLAB_CI
                    branch = v;
                }
            }
            CIType::Github => {
                if let Some(v) = option_env!("CI_COMMIT_REF_NAME") {//GITHUB_ACTIONS
                    branch = v;
                }
            }
            _ => {}
        }


        branch.to_string()
    }
}
