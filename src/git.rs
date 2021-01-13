use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::ci::CIType;
use crate::err::*;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use git2::Error as git2Error;
use git2::{Reference, Repository};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

pub const BRANCH: ShadowConst = "BRANCH";
pub(crate) const TAG: ShadowConst = "TAG";
const SHORT_COMMIT: ShadowConst = "SHORT_COMMIT";
const COMMIT_HASH: ShadowConst = "COMMIT_HASH";
const COMMIT_DATE: ShadowConst = "COMMIT_DATE";
const COMMIT_AUTHOR: ShadowConst = "COMMIT_AUTHOR";
const COMMIT_EMAIL: ShadowConst = "COMMIT_EMAIL";

#[derive(Default, Debug)]
pub struct Git {
    map: HashMap<ShadowConst, ConstVal>,
    ci_type: CIType,
}

impl Git {
    fn update_val(&mut self, c: ShadowConst, v: String) {
        if let Some(val) = self.map.get_mut(c) {
            *val = ConstVal {
                desc: val.desc.clone(),
                v,
                t: ConstType::Str,
            }
        }
    }

    fn init(&mut self, path: &Path, std_env: &HashMap<String, String>) -> SdResult<()> {
        let repo = git2::Repository::discover(path)?;
        let reference = repo.head()?;

        let (branch, tag) = self.get_branch_tag(&reference, &std_env)?;
        self.update_val(BRANCH, branch);
        self.update_val(TAG, tag);

        if let Some(v) = reference.target() {
            let commit = v.to_string();
            self.update_val(COMMIT_HASH, commit.clone());
            let mut short_commit = commit.as_str();

            if commit.len() > 8 {
                short_commit = &short_commit[0..8];
            }
            self.update_val(SHORT_COMMIT, short_commit.to_string());
        }

        let commit = reference.peel_to_commit()?;

        let time_stamp = commit.time().seconds().to_string().parse::<i64>()?;
        let dt = NaiveDateTime::from_timestamp(time_stamp, 0);
        let date_time = DateTime::<Utc>::from_utc(dt, Utc);
        let date_time: DateTime<Local> = DateTime::from(date_time);
        self.update_val(
            COMMIT_DATE,
            date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        );

        let author = commit.author();
        if let Some(v) = author.email() {
            self.update_val(COMMIT_EMAIL, v.to_string());
        }

        if let Some(v) = author.name() {
            self.update_val(COMMIT_AUTHOR, v.to_string());
        }

        Ok(())
    }

    #[allow(clippy::manual_strip)]
    fn get_branch_tag(
        &self,
        reference: &Reference<'_>,
        std_env: &HashMap<String, String>,
    ) -> SdResult<(String, String)> {
        let mut branch = String::new();
        let mut tag = String::new();

        //get branch
        if let Some(v) = reference.shorthand() {
            branch = v.to_string();
        }

        //get HEAD branch
        if let Ok(out) = Command::new("git")
            .args(&["tag", "-l", "--contains", "HEAD"])
            .output()
        {
            tag = String::from_utf8(out.stdout)?.trim().to_string();
        }

        match self.ci_type {
            CIType::Gitlab => {
                let gitlab_branch = if let Some(v) = std_env.get("CI_COMMIT_REF_NAME") {
                    v.to_string()
                } else {
                    branch.clone()
                };
                if let Some(v) = std_env.get("CI_COMMIT_TAG") {
                    tag = v.to_string();
                } else {
                    branch = gitlab_branch;
                }
            }
            CIType::Github => {
                if let Some(v) = std_env.get("GITHUB_REF") {
                    let ref_branch_prefix: &str = "refs/heads/";
                    let ref_tag_prefix: &str = "refs/tags/";

                    if v.starts_with(ref_branch_prefix) {
                        branch = v[ref_branch_prefix.len()..].to_string()
                    } else if v.starts_with(ref_tag_prefix) {
                        tag = v[ref_tag_prefix.len()..].to_string()
                    }
                }
            }
            _ => {}
        }

        Ok((branch, tag))
    }
}

pub fn new_git(
    path: &std::path::Path,
    ci: CIType,
    std_env: &HashMap<String, String>,
) -> HashMap<ShadowConst, ConstVal> {
    let mut git = Git {
        map: Default::default(),
        ci_type: ci,
    };
    git.map
        .insert(BRANCH, ConstVal::new("display current branch"));

    git.map.insert(TAG, ConstVal::new("display current tag"));

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

pub fn git_current_branch<P: AsRef<Path>>(path: P) -> Option<String> {
    git_repo(path)
        .map(|x| git2_current_branch(&x))
        .unwrap_or(command_current_branch())
}

fn git_repo<P: AsRef<Path>>(path: P) -> Result<Repository, git2Error> {
    git2::Repository::discover(path)
}

fn git2_current_branch(repo: &Repository) -> Option<String> {
    repo.head()
        .map(|x| x.shorthand().map(|x| x.to_string()))
        .unwrap_or(None)
}

fn command_current_branch() -> Option<String> {
    Command::new("git")
        .args(&["symbolic-ref", "--short", "HEAD"])
        .output()
        .map(|x| String::from_utf8(x.stdout).ok())
        .map(|x| x.map(|x| x.trim().to_string()))
        .unwrap_or(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Shadow;
    use std::path::Path;

    #[test]
    fn test_git() {
        let map = Shadow::get_env();
        let map = new_git(Path::new("./"), CIType::Github, &map);
        println!("map:{:?}", map);
    }

    #[test]
    fn test_current_branch() {
        let git2_branch = git_repo(".")
            .map(|x| git2_current_branch(&x))
            .unwrap_or(None);
        let command_branch = command_current_branch();
        assert!(git2_branch.is_some());
        assert!(command_branch.is_some());
        assert_eq!(command_branch, git2_branch);

        assert_eq!(git_current_branch("."), git2_branch);
    }
}
