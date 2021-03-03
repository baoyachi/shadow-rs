use crate::build::{ConstType, ConstVal, ShadowConst};
use crate::ci::CiType;
use crate::err::*;
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
    ci_type: CiType,
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
        self.init_git2(path)?;

        // use command branch
        if let Some(x) = command_current_branch() {
            self.update_val(BRANCH, x)
        };

        // use command tag
        if let Some(x) = command_current_tag() {
            self.update_val(TAG, x)
        }

        // try use ci branch,tag
        self.ci_branch_tag(std_env);
        Ok(())
    }

    fn init_git2(&mut self, path: &Path) -> SdResult<()> {
        #[cfg(feature = "git2")]
        {
            use crate::git::git2_mod::git_repo;
            use chrono::{DateTime, Local, NaiveDateTime, Utc};

            let repo = git_repo(path).map_err(ShadowError::new)?;
            let reference = repo.head().map_err(ShadowError::new)?;

            //get branch
            let branch = reference
                .shorthand()
                .map(|x| x.trim().to_string())
                .or_else(command_current_branch)
                .unwrap_or_default();

            //get HEAD branch
            let tag = command_current_tag().unwrap_or_default();

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

            let commit = reference.peel_to_commit().map_err(ShadowError::new)?;

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
        }
        Ok(())
    }

    #[allow(clippy::manual_strip)]
    fn ci_branch_tag(&mut self, std_env: &HashMap<String, String>) {
        let mut branch: Option<String> = None;
        let mut tag: Option<String> = None;
        match self.ci_type {
            CiType::Gitlab => {
                if let Some(v) = std_env.get("CI_COMMIT_TAG") {
                    tag = Some(v.to_string());
                } else if let Some(v) = std_env.get("CI_COMMIT_REF_NAME") {
                    branch = Some(v.to_string());
                }
            }
            CiType::Github => {
                if let Some(v) = std_env.get("GITHUB_REF") {
                    let ref_branch_prefix: &str = "refs/heads/";
                    let ref_tag_prefix: &str = "refs/tags/";

                    if v.starts_with(ref_branch_prefix) {
                        branch = Some(v[ref_branch_prefix.len()..].to_string())
                    } else if v.starts_with(ref_tag_prefix) {
                        tag = Some(v[ref_tag_prefix.len()..].to_string())
                    }
                }
            }
            _ => {}
        }
        if let Some(x) = branch {
            self.update_val(BRANCH, x);
        }

        if let Some(x) = tag {
            self.update_val(TAG, x);
        }
    }
}

pub fn new_git(
    path: &std::path::Path,
    ci: CiType,
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

#[cfg(feature = "git2")]
pub mod git2_mod {
    use git2::Error as git2Error;
    use git2::Repository;
    use std::path::Path;

    pub fn git_repo<P: AsRef<Path>>(path: P) -> Result<Repository, git2Error> {
        git2::Repository::discover(path)
    }

    pub fn git2_current_branch(repo: &Repository) -> Option<String> {
        repo.head()
            .map(|x| x.shorthand().map(|x| x.to_string()))
            .unwrap_or(None)
    }
}

/// get current repository git branch.
///
/// When current repository exists git folder.
///
/// It's use default feature.This function try use [git2] crates get current branch.
/// If not use git2 feature,then try use [Command] to get.
pub fn branch() -> String {
    #[cfg(feature = "git2")]
    {
        use crate::git::git2_mod::{git2_current_branch, git_repo};
        git_repo(".")
            .map(|x| git2_current_branch(&x))
            .unwrap_or_else(|_| command_current_branch())
            .unwrap_or_default()
    }
    #[cfg(not(feature = "git2"))]
    {
        command_current_branch().unwrap_or_default()
    }
}

/// get current repository git tag.
///
/// When current repository exists git folder.
/// I's use [Command] to get.
pub fn tag() -> String {
    command_current_tag().unwrap_or_default()
}

/// Command exec git current tag
fn command_current_tag() -> Option<String> {
    Command::new("git")
        .args(&["tag", "-l", "--contains", "HEAD"])
        .output()
        .map(|x| String::from_utf8(x.stdout).ok())
        .map(|x| x.map(|x| x.trim().to_string()))
        .unwrap_or(None)
}

/// Command exec git current branch
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
        let env_map = Shadow::get_env();
        let map = new_git(Path::new("./"), CiType::Github, &env_map);
        for (k, v) in map {
            println!("k:{},v:{:?}", k, v);
            assert!(!v.desc.is_empty());
            if !k.eq(TAG) && !k.eq(BRANCH) {
                assert!(!v.v.is_empty());
                continue;
            }

            //assert github tag always exist value
            if let Some(github_ref) = env_map.get("GITHUB_REF") {
                println!("github_ref:{}", github_ref);
                if github_ref.starts_with("refs/tags/") && k.eq(TAG) {
                    assert!(!v.v.is_empty());
                } else if github_ref.starts_with("refs/heads/") && k.eq(BRANCH) {
                    assert!(!v.v.is_empty());
                }
            }
        }
    }

    #[test]
    fn test_current_branch() {
        if Shadow::get_env().get("GITHUB_REF").is_some() {
            return;
        }
        #[cfg(feature = "git2")]
        {
            use crate::git::git2_mod::{git2_current_branch, git_repo};
            let git2_branch = git_repo(".")
                .map(|x| git2_current_branch(&x))
                .unwrap_or(None);
            let command_branch = command_current_branch();
            assert!(git2_branch.is_some());
            assert!(command_branch.is_some());
            assert_eq!(command_branch, git2_branch);
        }

        assert_eq!(Some(branch()), command_current_branch());
    }
}
